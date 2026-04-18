use crate::log;
use std::env;
use std::io::{self, Write};
use std::process::Command;

const DEFAULT_REFERENCE: &str = "op://key/gemini-key/credential";

fn reference() -> String {
    env::var("GEMMA_OP_REFERENCE").unwrap_or_else(|_| DEFAULT_REFERENCE.to_string())
}

/// Library entrypoint. Returns the API key or (human message, exit code) on failure.
pub fn fetch_api_key() -> Result<String, (String, u8)> {
    if which("op").is_err() {
        return Err((
            "1Password CLI (op) not found. install via `gemma ensure-deps --gemini`.".to_string(),
            2,
        ));
    }

    if !has_account() {
        return Err((
            "no 1Password account registered. run: op account add".to_string(),
            3,
        ));
    }

    let r = reference();
    let output = Command::new("op")
        .arg("read")
        .arg(&r)
        .output()
        .map_err(|e| (format!("failed to spawn `op`: {e}"), 4))?;

    if output.status.success() {
        let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if key.is_empty() {
            return Err(("1Password returned an empty secret.".to_string(), 4));
        }
        return Ok(key);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let low = stderr.to_lowercase();
    let mut msg = format!("failed to read '{r}' from 1Password.");
    if low.contains("not signed in") || low.contains("session") {
        msg.push_str(
            "\nrun: eval \"$(op signin)\"   (or enable Touch ID integration in the 1Password app → Developer)",
        );
    } else {
        msg.push_str(&format!(
            "\ncheck the vault/item/field exists, or override via GEMMA_OP_REFERENCE.\n  op: {}",
            stderr.trim().replace('\n', "\n  op: ")
        ));
    }
    Err((msg, 4))
}

/// CLI entrypoint for `gemma fetch-api-key`.
pub fn run_fetch_cli() -> u8 {
    match fetch_api_key() {
        Ok(key) => {
            let _ = io::stdout().write_all(key.as_bytes());
            0
        }
        Err((msg, rc)) => {
            log::err(&msg);
            rc
        }
    }
}

fn which(cmd: &str) -> Result<(), ()> {
    let status = Command::new("command")
        .arg("-v")
        .arg(cmd)
        .output();
    match status {
        Ok(o) if o.status.success() && !o.stdout.is_empty() => Ok(()),
        _ => {
            // Fallback: check PATH directly.
            if let Ok(path) = env::var("PATH") {
                for dir in path.split(':') {
                    if std::path::Path::new(dir).join(cmd).exists() {
                        return Ok(());
                    }
                }
            }
            Err(())
        }
    }
}

fn has_account() -> bool {
    let output = Command::new("op")
        .arg("account")
        .arg("list")
        .arg("--format=json")
        .output();
    matches!(output, Ok(o) if o.status.success() && String::from_utf8_lossy(&o.stdout).contains("\"url\""))
}
