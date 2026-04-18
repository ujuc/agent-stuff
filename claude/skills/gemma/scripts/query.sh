#!/usr/bin/env bash
# query.sh — Route a prompt to local LM Studio or Google AI Studio / Gemini API.
#
# Usage:
#   query.sh [--local|--cloud] [variant] <prompt...>
#
# Variant examples:  e2b, e4b, 26b, 31b, pro, flash
# Default variant:   e4b
#
# Routing (default):
#   e2b, e4b  → LM Studio (local)
#   26b, 31b  → Gemini API (remote)
#   --local   → force LM Studio
#   --cloud   → force Gemini API
#   LM Studio unreachable / no matching model loaded → auto-fallback to Gemini
#     (unless GEMMA_NO_FALLBACK=1)
#
# Remote model selection:
#   GEMMA_GEMINI_MODEL env → use as-is.
#   Otherwise: scan `list-gemini-models.sh` for the highest-version Gemma
#   matching variant; if none, fall back to gemini-flash-latest (e2b/e4b/26b)
#   or gemini-pro-latest (31b).
#
# Env overrides:
#   GEMMA_BACKEND           lmstudio | gemini (equivalent to --local/--cloud)
#   GEMMA_LMSTUDIO_HOST     default http://localhost:1234
#   GEMMA_GEMINI_MODEL      full model ID override for remote path
#   GEMMA_OP_REFERENCE      1Password secret reference (forwarded to fetch-api-key.sh)
#   GOOGLE_AI_API_KEY       API key override (skip 1Password)
#   GEMMA_TIMEOUT           HTTP timeout seconds (default 120)
#   GEMMA_NO_FALLBACK       1 = disable LM Studio→Gemini auto-fallback
#   GEMMA_AUTO_INSTALL      1 = let ensure-deps.sh install without prompting

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LMSTUDIO_HOST="${GEMMA_LMSTUDIO_HOST:-http://localhost:1234}"
TIMEOUT="${GEMMA_TIMEOUT:-120}"

log()  { printf 'info: %s\n' "$*" >&2; }
warn() { printf 'warn: %s\n' "$*" >&2; }
err()  { printf 'error: %s\n' "$*" >&2; }

# --- Arg parsing ---

FORCE_BACKEND="${GEMMA_BACKEND:-}"
if [[ $# -gt 0 ]]; then
  case "${1:-}" in
    --local) FORCE_BACKEND=lmstudio; shift ;;
    --cloud) FORCE_BACKEND=gemini;   shift ;;
  esac
fi

if [[ $# -eq 0 ]]; then
  err "usage: $(basename "$0") [--local|--cloud] [variant] <prompt>"
  exit 64
fi

VARIANT="e4b"
# Treat short alnum token as variant unless it's the only arg (then it's the prompt).
if [[ $# -ge 2 ]] && [[ "$1" =~ ^[a-z0-9]{1,6}$ ]]; then
  VARIANT="$1"
  shift
fi

PROMPT="$*"
if [[ -z "$PROMPT" ]]; then
  err "prompt is empty."
  exit 64
fi

# --- Dependency sanity (lightweight; deeper install handled by ensure-deps.sh) ---
for dep in curl jq; do
  if ! command -v "$dep" >/dev/null 2>&1; then
    warn "required dep '$dep' missing; attempting auto-install"
    bash "${SCRIPT_DIR}/ensure-deps.sh" >&2 || { err "ensure-deps failed"; exit 2; }
    break
  fi
done

# --- Pick default backend if not forced ---

pick_default_backend() {
  case "$VARIANT" in
    e2b|e4b) echo lmstudio ;;
    26b|31b|pro|flash) echo gemini ;;
    *) echo lmstudio ;;
  esac
}

BACKEND="${FORCE_BACKEND:-$(pick_default_backend)}"

# =========================================================================
# LM Studio path
# =========================================================================

lmstudio_reachable() {
  curl -sfm 3 "${LMSTUDIO_HOST}/v1/models" >/dev/null 2>&1
}

lmstudio_pick_model() {
  local tags_json models_csv matched
  tags_json="$(curl -sfm 5 "${LMSTUDIO_HOST}/v1/models")" || return 1
  # Prefer model whose id contains both "gemma" and the variant literal.
  matched="$(printf '%s' "$tags_json" \
    | jq -r --arg v "$VARIANT" '.data[].id | select(test("gemma"; "i")) | select(test($v; "i"))' \
    | head -1)"
  if [[ -z "$matched" ]]; then
    # Fall back to any loaded gemma model.
    matched="$(printf '%s' "$tags_json" \
      | jq -r '.data[].id | select(test("gemma"; "i"))' | head -1)"
  fi
  [[ -n "$matched" ]] && printf '%s' "$matched"
}

run_lmstudio() {
  local model payload response
  model="$(lmstudio_pick_model)" || return 1
  if [[ -z "$model" ]]; then
    warn "LM Studio has no gemma model loaded."
    return 1
  fi
  log "backend=lmstudio model=${model}"
  payload="$(jq -n --arg m "$model" --arg p "$PROMPT" \
    '{model: $m, messages: [{role:"user", content:$p}], stream:false, temperature:0.7}')"
  response="$(curl -sfm "$TIMEOUT" -H 'Content-Type: application/json' \
    -d "$payload" "${LMSTUDIO_HOST}/v1/chat/completions")" || return 1
  printf '%s' "$response" | jq -re '.choices[0].message.content'
}

# =========================================================================
# Gemini API path
# =========================================================================

resolve_api_key() {
  if [[ -n "${GOOGLE_AI_API_KEY:-}" ]]; then
    printf '%s' "$GOOGLE_AI_API_KEY"
    return 0
  fi
  bash "${SCRIPT_DIR}/fetch-api-key.sh"
}

gemini_pick_model() {
  if [[ -n "${GEMMA_GEMINI_MODEL:-}" ]]; then
    printf '%s' "$GEMMA_GEMINI_MODEL"
    return 0
  fi

  local models pattern matched gemini_fallback
  models="$(bash "${SCRIPT_DIR}/list-gemini-models.sh" 2>/dev/null || true)"

  # Gemma pattern per variant.
  case "$VARIANT" in
    e2b)  pattern='gemma-[0-9]+n?-e2b-it' ;;
    e4b)  pattern='gemma-[0-9]+n?-e4b-it' ;;
    26b)  pattern='gemma-[0-9]+-(26b|27b)-it' ;;
    31b)  pattern='gemma-[0-9]+-31b-it' ;;
    pro|flash) pattern='' ;;   # go straight to Gemini aliases
    *)    pattern='gemma-[0-9]+.*'"$VARIANT"'.*-it' ;;
  esac

  # Gemini fallback target.
  case "$VARIANT" in
    31b|pro) gemini_fallback="gemini-pro-latest" ;;
    *)       gemini_fallback="gemini-flash-latest" ;;
  esac

  if [[ -n "$pattern" && -n "$models" ]]; then
    # Pick highest-version match (sort by leading digit after "gemma-").
    matched="$(printf '%s\n' "$models" \
      | grep -E "^${pattern}$" \
      | awk -F'-' '{ n=$2; gsub(/[^0-9]/,"",n); print n, $0 }' \
      | sort -k1,1nr \
      | awk '{ print $2 }' \
      | head -1)"
  fi

  if [[ -n "${matched:-}" ]]; then
    printf '%s' "$matched"
  else
    # Verify fallback exists; if model list unavailable, still try the alias.
    if [[ -n "$models" ]] && ! printf '%s\n' "$models" | grep -qxF "$gemini_fallback"; then
      warn "Gemini alias '$gemini_fallback' not in listed models; trying anyway"
    fi
    printf '%s' "$gemini_fallback"
  fi
}

run_gemini() {
  local key model payload response reason="${1:-}"
  key="$(resolve_api_key)" || return 1
  model="$(gemini_pick_model)" || return 1

  case "$model" in
    gemma-*) log "backend=gemini model=${model}${reason:+ (${reason})}" ;;
    *)       log "backend=gemini model=${model} (Gemma not available on API)${reason:+, ${reason}}" ;;
  esac

  payload="$(jq -n --arg p "$PROMPT" '{contents:[{parts:[{text:$p}]}]}')"
  response="$(curl -sfm "$TIMEOUT" -H 'Content-Type: application/json' \
    -d "$payload" "https://generativelanguage.googleapis.com/v1beta/models/${model}:generateContent?key=${key}")" || {
    local rc=$?
    err "Gemini API call failed (curl exit ${rc})"
    return 5
  }

  if ! printf '%s' "$response" | jq -re '.candidates[0].content.parts[0].text' 2>/dev/null; then
    err "unexpected Gemini response:"
    printf '%s\n' "$response" >&2
    return 6
  fi
}

# =========================================================================
# Dispatch
# =========================================================================

if [[ "$BACKEND" == "lmstudio" ]]; then
  if lmstudio_reachable; then
    if run_lmstudio; then
      exit 0
    fi
    warn "LM Studio call failed or no suitable model loaded"
  else
    warn "LM Studio unreachable at ${LMSTUDIO_HOST}"
  fi

  if [[ "${GEMMA_NO_FALLBACK:-0}" == "1" || "${FORCE_BACKEND:-}" == "lmstudio" ]]; then
    err "LM Studio unavailable and fallback disabled (GEMMA_NO_FALLBACK=1 or --local)"
    err "hint: lms server start  &&  lms load <model>"
    exit 3
  fi

  # Auto-fallback to Gemini.
  run_gemini "fallback from LM Studio"
  exit $?
fi

# Gemini direct path
run_gemini
