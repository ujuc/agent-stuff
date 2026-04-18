use crate::{keychain, log, models};
use regex::Regex;
use serde_json::{Value, json};
use std::env;
use std::io::{self, Write};
use std::time::Duration;

pub fn run(variant: &str, prompt: &str, timeout_secs: u64, reason: Option<&str>) -> u8 {
    let key = match resolve_api_key() {
        Ok(k) => k,
        Err(rc) => return rc,
    };
    let model = match pick_model(variant) {
        Some(m) => m,
        None => {
            log::err("could not resolve a Gemini model (list unavailable and no override).");
            return 5;
        }
    };

    let prefix = if model.starts_with("gemma-") {
        format!("backend=gemini model={model}")
    } else {
        format!("backend=gemini model={model} (Gemma not available on API)")
    };
    let line = match reason {
        Some(r) => {
            if model.starts_with("gemma-") {
                format!("{prefix} ({r})")
            } else {
                format!("{prefix}, {r}")
            }
        }
        None => prefix,
    };
    log::info(&line);

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}"
    );
    let payload = json!({ "contents": [{ "parts": [{ "text": prompt }] }] });
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build();
    let resp = match agent
        .post(&url)
        .set("Content-Type", "application/json")
        .send_json(payload)
    {
        Ok(r) => r,
        Err(e) => {
            log::err(&format!("Gemini API call failed: {e}"));
            return 5;
        }
    };
    let body: Value = match resp.into_json() {
        Ok(v) => v,
        Err(e) => {
            log::err(&format!("invalid JSON from Gemini: {e}"));
            return 6;
        }
    };
    match body["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        Some(text) => {
            let mut out = io::stdout().lock();
            let _ = out.write_all(text.as_bytes());
            if !text.ends_with('\n') {
                let _ = out.write_all(b"\n");
            }
            0
        }
        None => {
            log::err("unexpected Gemini response:");
            let _ = writeln!(io::stderr(), "{body}");
            6
        }
    }
}

fn resolve_api_key() -> Result<String, u8> {
    if let Ok(k) = env::var("GOOGLE_AI_API_KEY") {
        if !k.is_empty() {
            return Ok(k);
        }
    }
    keychain::fetch_api_key().map_err(|(msg, rc)| {
        log::err(&msg);
        rc
    })
}

fn pick_model(variant: &str) -> Option<String> {
    if let Ok(m) = env::var("GEMMA_GEMINI_MODEL") {
        if !m.is_empty() {
            return Some(m);
        }
    }

    let models_list = models::list_cached().unwrap_or_default();

    let pattern = match variant {
        "e2b" => Some(r"^gemma-[0-9]+n?-e2b-it$".to_string()),
        "e4b" => Some(r"^gemma-[0-9]+n?-e4b-it$".to_string()),
        "26b" => Some(r"^gemma-[0-9]+-(26b|27b)-it$".to_string()),
        "31b" => Some(r"^gemma-[0-9]+-31b-it$".to_string()),
        "pro" | "flash" => None,
        other => Some(format!(r"^gemma-[0-9]+.*{}.*-it$", regex::escape(other))),
    };

    let fallback = match variant {
        "31b" | "pro" => "gemini-pro-latest",
        _ => "gemini-flash-latest",
    };

    if let Some(pat) = pattern.as_deref() {
        if !models_list.is_empty() {
            if let Ok(re) = Regex::new(pat) {
                let mut matches: Vec<(u32, &String)> = models_list
                    .iter()
                    .filter(|id| re.is_match(id))
                    .map(|id| (leading_version(id), id))
                    .collect();
                matches.sort_by(|a, b| b.0.cmp(&a.0));
                if let Some((_, id)) = matches.first() {
                    return Some((*id).clone());
                }
            }
        }
    }

    if !models_list.is_empty() && !models_list.iter().any(|m| m == fallback) {
        log::warn(&format!(
            "Gemini alias '{fallback}' not in listed models; trying anyway"
        ));
    }
    Some(fallback.to_string())
}

/// Extract the leading digit group from the version component of a model id.
/// `gemma-3-26b-it` → 3, `gemma-4n-e4b-it` → 4.
fn leading_version(id: &str) -> u32 {
    id.split('-').nth(1).map_or(0, |part| {
        let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        digits.parse().unwrap_or(0)
    })
}
