use crate::{gemini, lmstudio, log};
use clap::Args;
use std::env;

#[derive(Args, Debug)]
pub struct QueryArgs {
    /// Force LM Studio (local) backend; disable fallback to Gemini.
    #[arg(long, conflicts_with = "cloud")]
    pub local: bool,
    /// Force Google AI Studio / Gemini (cloud) backend.
    #[arg(long)]
    pub cloud: bool,
    /// Positional args: [variant] prompt...
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub rest: Vec<String>,
}

const DEFAULT_VARIANT: &str = "e4b";

pub fn run(args: QueryArgs) -> u8 {
    // Matches bash FORCE_BACKEND semantics: env and flags populate the same slot.
    // `forced_lmstudio` is what blocks fallback (along with GEMMA_NO_FALLBACK),
    // regardless of whether the force came from --local or GEMMA_BACKEND=lmstudio.
    let env_backend = env::var("GEMMA_BACKEND").ok();
    let force: Option<&str> = if args.local {
        Some("lmstudio")
    } else if args.cloud {
        Some("gemini")
    } else {
        match env_backend.as_deref() {
            Some("lmstudio") => Some("lmstudio"),
            Some("gemini") => Some("gemini"),
            _ => None,
        }
    };
    let forced_lmstudio = force == Some("lmstudio");

    if args.rest.is_empty() {
        log::err("usage: gemma query [--local|--cloud] [variant] <prompt>");
        return 64;
    }

    let (variant, prompt) = split_variant_and_prompt(&args.rest);
    if prompt.is_empty() {
        log::err("prompt is empty.");
        return 64;
    }

    let backend = force.unwrap_or_else(|| pick_default_backend(&variant));
    let timeout_secs: u64 = env::var("GEMMA_TIMEOUT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(120);
    let no_fallback = env::var("GEMMA_NO_FALLBACK").ok().as_deref() == Some("1");

    match backend {
        "lmstudio" => {
            match lmstudio::attempt(&variant, &prompt, timeout_secs) {
                Ok(()) => 0,
                Err(e) => {
                    log::warn(&e);
                    if no_fallback || forced_lmstudio {
                        log::err(
                            "LM Studio unavailable and fallback disabled \
                             (GEMMA_NO_FALLBACK=1, --local, or GEMMA_BACKEND=lmstudio)",
                        );
                        log::err("hint: lms server start  &&  lms load <model>");
                        return 3;
                    }
                    gemini::run(&variant, &prompt, timeout_secs, Some("fallback from LM Studio"))
                }
            }
        }
        _ => gemini::run(&variant, &prompt, timeout_secs, None),
    }
}

fn split_variant_and_prompt(rest: &[String]) -> (String, String) {
    if rest.len() >= 2 && is_variant_token(&rest[0]) {
        (rest[0].clone(), rest[1..].join(" "))
    } else {
        (DEFAULT_VARIANT.to_string(), rest.join(" "))
    }
}

fn is_variant_token(s: &str) -> bool {
    let len = s.len();
    (1..=6).contains(&len) && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

fn pick_default_backend(variant: &str) -> &'static str {
    match variant {
        "e2b" | "e4b" => "lmstudio",
        "26b" | "31b" | "pro" | "flash" => "gemini",
        _ => "lmstudio",
    }
}
