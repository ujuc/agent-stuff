#!/usr/bin/env bash
# list-gemini-models.sh — List model IDs exposed by the Google AI Studio / Gemini API.
#
# Usage:
#   list-gemini-models.sh
#
# Env overrides:
#   GOOGLE_AI_API_KEY      If set, used directly. Otherwise fetched via fetch-api-key.sh.
#   GEMMA_MODELS_CACHE     Cache file path (default: /tmp/gemma-skill-models.cache)
#   GEMMA_MODELS_TTL       Cache TTL in seconds (default: 300 = 5 min)
#   GEMMA_MODELS_FORCE=1   Bypass cache.
#
# Output: one model short name (e.g., "gemini-flash-latest") per line on stdout.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CACHE_FILE="${GEMMA_MODELS_CACHE:-/tmp/gemma-skill-models.cache}"
CACHE_TTL="${GEMMA_MODELS_TTL:-300}"

err() { printf 'error: %s\n' "$*" >&2; }

# --- Cache hit? ---
if [[ "${GEMMA_MODELS_FORCE:-0}" != "1" && -f "$CACHE_FILE" ]]; then
  if [[ "$(uname)" == "Darwin" ]]; then
    mtime=$(stat -f %m "$CACHE_FILE" 2>/dev/null || echo 0)
  else
    mtime=$(stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0)
  fi
  now=$(date +%s)
  if (( now - mtime < CACHE_TTL )); then
    cat "$CACHE_FILE"
    exit 0
  fi
fi

# --- Resolve API key ---
KEY="${GOOGLE_AI_API_KEY:-}"
if [[ -z "$KEY" ]]; then
  KEY="$(bash "${SCRIPT_DIR}/fetch-api-key.sh")"
fi

if [[ -z "$KEY" ]]; then
  err "no API key available."
  exit 4
fi

# --- Fetch ---
response="$(curl -sfm 10 "https://generativelanguage.googleapis.com/v1beta/models?key=${KEY}")" || {
  err "failed to list models (curl exit $?)"
  exit 5
}

# The API returns "name": "models/gemini-flash-latest". Strip the "models/" prefix.
models="$(printf '%s' "$response" | jq -r '.models[].name' | sed 's|^models/||')"

if [[ -z "$models" ]]; then
  err "empty model list from API"
  printf '%s\n' "$response" >&2
  exit 6
fi

printf '%s\n' "$models" | tee "$CACHE_FILE"
