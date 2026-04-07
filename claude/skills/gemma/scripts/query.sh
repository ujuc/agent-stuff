#!/usr/bin/env bash
# query.sh — Send a prompt to the latest local Gemma model via Ollama and print the response.
#
# Usage:
#   query.sh "<prompt>"                # uses default variant on latest gemma version
#   query.sh <variant> "<prompt>"      # explicit variant (e.g., e2b, e4b, 26b, 31b)
#
# Requires: ollama (serving on localhost:11434), curl, jq
# Env overrides:
#   GEMMA_HOST     (default: http://localhost:11434)
#   GEMMA_TIMEOUT  (default: 120  — seconds)
#   GEMMA_VARIANT  (default: latest)
#   GEMMA_MODEL    (default: auto-detect — full override, e.g., gemma4:e4b)

set -euo pipefail

HOST="${GEMMA_HOST:-http://localhost:11434}"
TIMEOUT="${GEMMA_TIMEOUT:-120}"
DEFAULT_VARIANT="${GEMMA_VARIANT:-latest}"

# --- Dependency checks ---
for dep in curl jq; do
  if ! command -v "$dep" >/dev/null 2>&1; then
    echo "error: required dependency '$dep' not found." >&2
    echo "hint: brew install $dep" >&2
    exit 2
  fi
done

# --- Arg parsing ---
if [[ $# -eq 0 ]]; then
  echo "usage: $(basename "$0") [variant] <prompt>" >&2
  echo "       variant: short tag like e2b, e4b, 26b, 31b, etc. (default: ${DEFAULT_VARIANT})" >&2
  exit 64
fi

VARIANT="$DEFAULT_VARIANT"
if [[ $# -ge 2 ]] && [[ "${1}" =~ ^[a-z0-9]{1,5}$ ]]; then
  VARIANT="$1"
  shift
fi

if [[ $# -eq 0 ]]; then
  echo "error: prompt is empty." >&2
  exit 64
fi

PROMPT="$*"

# --- Ollama reachability ---
if ! curl -sfm 3 "${HOST}/api/tags" >/dev/null 2>&1; then
  echo "error: Ollama not reachable at ${HOST}" >&2
  echo "hint: start it with 'ollama serve' (or open the Ollama app)." >&2
  exit 3
fi

# --- Fetch tags ---
tags_json="$(curl -sfm 5 "${HOST}/api/tags")"

# --- Auto-detect latest gemma version ---
detect_latest_gemma() {
  printf '%s' "$1" \
    | jq -r '[.models[].details.family // empty] | unique | .[]' \
    | grep -E '^gemma[0-9]+$' \
    | sed 's/gemma//' | sort -n | tail -1 | sed 's/^/gemma/'
}

if [[ -n "${GEMMA_MODEL:-}" ]]; then
  # Full override — skip auto-detection entirely
  MODEL="$GEMMA_MODEL"
else
  BASE_MODEL="$(detect_latest_gemma "$tags_json")"
  if [[ -z "$BASE_MODEL" ]]; then
    echo "error: no gemma model found in Ollama." >&2
    echo "hint: ollama pull gemma4" >&2
    exit 4
  fi

  # Resolve tag
  if [[ "$VARIANT" == "latest" ]]; then
    MODEL="${BASE_MODEL}:latest"
    FALLBACK_MODEL=""
  else
    MODEL="${BASE_MODEL}:${VARIANT}"
    FALLBACK_MODEL="${BASE_MODEL}:latest"
  fi
fi

echo "info: using model ${MODEL}" >&2

# --- Model availability ---
if ! printf '%s' "$tags_json" | jq -e --arg m "$MODEL" '.models[] | select(.name == $m)' >/dev/null 2>&1; then
  if [[ -n "${FALLBACK_MODEL:-}" ]] && printf '%s' "$tags_json" | jq -e --arg m "$FALLBACK_MODEL" '.models[] | select(.name == $m)' >/dev/null 2>&1; then
    echo "info: variant '${VARIANT}' not found, falling back to ${FALLBACK_MODEL}" >&2
    MODEL="$FALLBACK_MODEL"
  else
    echo "error: model '${MODEL}' is not installed in Ollama." >&2
    echo "hint: ollama pull ${MODEL}" >&2
    exit 4
  fi
fi

# --- Build JSON payload safely (jq handles quoting) ---
payload="$(jq -n --arg m "$MODEL" --arg p "$PROMPT" \
  '{model: $m, prompt: $p, stream: false}')"

# --- Call Ollama ---
response="$(curl -sfm "$TIMEOUT" -H 'Content-Type: application/json' \
  -d "$payload" "${HOST}/api/generate")" || {
  rc=$?
  if [[ $rc -eq 28 ]]; then
    echo "error: request timed out after ${TIMEOUT}s." >&2
    echo "hint: shorten the prompt or set GEMMA_TIMEOUT to a larger value." >&2
  else
    echo "error: curl failed with exit code ${rc}." >&2
  fi
  exit 5
}

# --- Extract and print ---
if ! printf '%s' "$response" | jq -e '.response' >/dev/null 2>&1; then
  echo "error: unexpected response from Ollama:" >&2
  printf '%s\n' "$response" >&2
  exit 6
fi

printf '%s\n' "$response" | jq -r '.response'
