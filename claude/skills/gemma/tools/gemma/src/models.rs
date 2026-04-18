use crate::{keychain, log};
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

fn cache_path() -> PathBuf {
    env::var("GEMMA_MODELS_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/gemma-skill-models.cache"))
}

fn cache_ttl() -> u64 {
    env::var("GEMMA_MODELS_TTL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300)
}

fn force_refresh() -> bool {
    env::var("GEMMA_MODELS_FORCE").ok().as_deref() == Some("1")
}

/// Read cached list if present and still fresh. Used by gemini::pick_model.
/// Returns empty Vec (never errors) when cache is stale/missing to keep the caller silent.
pub fn list_cached() -> Option<Vec<String>> {
    // Try cache first.
    if !force_refresh() {
        if let Some(contents) = read_fresh_cache() {
            return Some(parse_lines(&contents));
        }
    }
    // Fall back to live fetch.
    fetch_live().ok().map(|raw| parse_lines(&raw))
}

/// CLI entrypoint for `gemma list-models`.
pub fn run_list_cli() -> u8 {
    if !force_refresh() {
        if let Some(contents) = read_fresh_cache() {
            let _ = io::stdout().write_all(contents.as_bytes());
            if !contents.ends_with('\n') {
                let _ = writeln!(io::stdout());
            }
            return 0;
        }
    }

    let raw = match fetch_live() {
        Ok(v) => v,
        Err((msg, rc)) => {
            log::err(&msg);
            return rc;
        }
    };

    if raw.trim().is_empty() {
        log::err("empty model list from API");
        return 6;
    }

    let _ = fs::write(cache_path(), &raw);
    let _ = io::stdout().write_all(raw.as_bytes());
    if !raw.ends_with('\n') {
        let _ = writeln!(io::stdout());
    }
    0
}

fn read_fresh_cache() -> Option<String> {
    let path = cache_path();
    let meta = fs::metadata(&path).ok()?;
    let mtime = meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    if now.saturating_sub(mtime) < cache_ttl() {
        fs::read_to_string(&path).ok()
    } else {
        None
    }
}

fn fetch_live() -> Result<String, (String, u8)> {
    let key = match env::var("GOOGLE_AI_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => keychain::fetch_api_key()?,
    };

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={key}");
    let agent = ureq::AgentBuilder::new().timeout(FETCH_TIMEOUT).build();
    let resp = agent
        .get(&url)
        .call()
        .map_err(|e| (format!("failed to list models: {e}"), 5))?;
    let body: Value = resp
        .into_json()
        .map_err(|e| (format!("invalid JSON from Gemini: {e}"), 6))?;

    let arr = body["models"]
        .as_array()
        .ok_or_else(|| ("Gemini response missing 'models' array".to_string(), 6))?;

    let mut out = String::new();
    for m in arr {
        if let Some(name) = m["name"].as_str() {
            let trimmed = name.strip_prefix("models/").unwrap_or(name);
            out.push_str(trimmed);
            out.push('\n');
        }
    }
    Ok(out)
}

fn parse_lines(raw: &str) -> Vec<String> {
    raw.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}
