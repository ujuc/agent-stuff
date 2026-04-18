use crate::log;
use serde_json::{Value, json};
use std::env;
use std::io::{self, Write};
use std::time::Duration;

const REACHABILITY_TIMEOUT: Duration = Duration::from_secs(3);
const MODELS_LIST_TIMEOUT: Duration = Duration::from_secs(5);

fn host() -> String {
    env::var("GEMMA_LMSTUDIO_HOST").unwrap_or_else(|_| "http://localhost:1234".to_string())
}

pub fn reachable() -> bool {
    let url = format!("{}/v1/models", host());
    ureq::AgentBuilder::new()
        .timeout(REACHABILITY_TIMEOUT)
        .build()
        .get(&url)
        .call()
        .is_ok()
}

fn list_models() -> Result<Vec<String>, String> {
    let url = format!("{}/v1/models", host());
    let agent = ureq::AgentBuilder::new().timeout(MODELS_LIST_TIMEOUT).build();
    let body: Value = agent
        .get(&url)
        .call()
        .map_err(|e| format!("LM Studio /v1/models failed: {e}"))?
        .into_json()
        .map_err(|e| format!("invalid JSON from LM Studio: {e}"))?;
    let ids = body["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    Ok(ids)
}

fn pick_model(variant: &str) -> Option<String> {
    let ids = list_models().ok()?;
    let v_lower = variant.to_lowercase();
    // Prefer models containing both "gemma" and the variant literal (case-insensitive).
    for id in &ids {
        let low = id.to_lowercase();
        if low.contains("gemma") && low.contains(&v_lower) {
            return Some(id.clone());
        }
    }
    // Fallback: any gemma model.
    for id in &ids {
        if id.to_lowercase().contains("gemma") {
            return Some(id.clone());
        }
    }
    None
}

/// Attempt an LM Studio call. Returns Ok(()) when the response was printed to stdout,
/// or Err(reason) when caller should log a warning and consider fallback.
pub fn attempt(variant: &str, prompt: &str, timeout_secs: u64) -> Result<(), String> {
    if !reachable() {
        return Err(format!("LM Studio unreachable at {}", host()));
    }
    let model = match pick_model(variant) {
        Some(m) => m,
        None => return Err("LM Studio has no gemma model loaded.".to_string()),
    };
    log::info(&format!("backend=lmstudio model={model}"));

    let payload = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": false,
        "temperature": 0.7,
    });
    let url = format!("{}/v1/chat/completions", host());
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build();
    let resp = agent
        .post(&url)
        .set("Content-Type", "application/json")
        .send_json(payload)
        .map_err(|e| format!("LM Studio call failed: {e}"))?;
    let body: Value = resp
        .into_json()
        .map_err(|e| format!("invalid JSON from LM Studio: {e}"))?;
    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "LM Studio response missing choices[0].message.content".to_string())?;
    let mut out = io::stdout().lock();
    let _ = out.write_all(content.as_bytes());
    if !content.ends_with('\n') {
        let _ = out.write_all(b"\n");
    }
    Ok(())
}
