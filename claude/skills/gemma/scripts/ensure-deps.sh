#!/usr/bin/env bash
# ensure-deps.sh — Verify and optionally install dependencies for the gemma skill.
#
# Usage:
#   ensure-deps.sh [--lmstudio] [--gemini] [--all]
#
# Flags select which dependency group(s) to check. Default: --all.
#
# Env overrides:
#   GEMMA_AUTO_INSTALL=1   Skip interactive confirmation; install silently.

set -euo pipefail

CHECK_LMSTUDIO=0
CHECK_GEMINI=0

if [[ $# -eq 0 ]]; then
  CHECK_LMSTUDIO=1
  CHECK_GEMINI=1
else
  for arg in "$@"; do
    case "$arg" in
      --lmstudio) CHECK_LMSTUDIO=1 ;;
      --gemini)   CHECK_GEMINI=1 ;;
      --all)      CHECK_LMSTUDIO=1; CHECK_GEMINI=1 ;;
      *) echo "error: unknown flag '$arg'" >&2; exit 64 ;;
    esac
  done
fi

log()  { printf 'info: %s\n' "$*" >&2; }
warn() { printf 'warn: %s\n' "$*" >&2; }
err()  { printf 'error: %s\n' "$*" >&2; }

# --- Homebrew presence ---
if ! command -v brew >/dev/null 2>&1; then
  err "Homebrew (brew) not found."
  err "install from https://brew.sh then re-run."
  exit 2
fi

confirm_install() {
  local pkg="$1" display="$2"
  if [[ "${GEMMA_AUTO_INSTALL:-0}" == "1" ]]; then
    return 0
  fi
  printf 'install %s via brew? [y/N] ' "$display" >&2
  local reply
  read -r reply </dev/tty || reply=""
  [[ "$reply" =~ ^[Yy]$ ]]
}

ensure_formula() {
  local bin="$1" formula="$2"
  if command -v "$bin" >/dev/null 2>&1; then
    return 0
  fi
  if confirm_install "$formula" "$formula"; then
    log "installing $formula..."
    brew install "$formula" >&2
  else
    err "'$bin' missing; install manually: brew install $formula"
    return 1
  fi
}

ensure_cask() {
  local bin="$1" cask="$2" path_hint="$3"
  if command -v "$bin" >/dev/null 2>&1; then
    return 0
  fi
  if confirm_install "--cask $cask" "$cask (cask)"; then
    log "installing $cask..."
    brew install --cask "$cask" >&2
  else
    err "'$bin' missing; install manually: brew install --cask $cask"
    return 1
  fi
  # Post-install PATH hint (cask binaries often live outside /usr/local/bin)
  if ! command -v "$bin" >/dev/null 2>&1 && [[ -n "$path_hint" ]]; then
    warn "$bin still not on PATH; add this to your shell rc:"
    warn "  export PATH=\"$path_hint:\$PATH\""
  fi
}

# --- Core deps (always required) ---
ensure_formula curl curl
ensure_formula jq   jq

# --- Backend-specific deps ---
if [[ $CHECK_LMSTUDIO -eq 1 ]]; then
  ensure_cask lms lm-studio "\$HOME/.lmstudio/bin"
fi

if [[ $CHECK_GEMINI -eq 1 ]]; then
  ensure_cask op 1password-cli ""
fi

log "all required dependencies present."
