#!/usr/bin/env bash
# fetch-api-key.sh — Read the Google AI Studio API key from 1Password and print it to stdout.
#
# Usage:
#   fetch-api-key.sh
#
# Env overrides:
#   GEMMA_OP_REFERENCE   (default: op://key/gemini-key/credential)

set -euo pipefail

REF="${GEMMA_OP_REFERENCE:-op://key/gemini-key/credential}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

err() { printf 'error: %s\n' "$*" >&2; }

if ! command -v op >/dev/null 2>&1; then
  err "1Password CLI (op) not found."
  err "install: bash ${SCRIPT_DIR}/ensure-deps.sh --gemini"
  exit 2
fi

# Skip `op whoami` (session-based; always fails under Touch ID integration).
# Try `op read` directly — Touch ID integration will prompt if needed.
if ! op account list --format=json 2>/dev/null | grep -q '"url"'; then
  err "no 1Password account registered."
  err "run: op account add"
  exit 3
fi

if ! op read "$REF" 2>/tmp/op-err.$$; then
  err "failed to read '$REF'."
  if grep -qi 'not signed in\|session' /tmp/op-err.$$; then
    err "run: eval \$(op signin)   (or enable Touch ID integration in the 1Password app → Developer)"
  else
    err "check the vault/item/field exists, or override via GEMMA_OP_REFERENCE."
    sed 's/^/  op: /' /tmp/op-err.$$ >&2
  fi
  rm -f /tmp/op-err.$$
  exit 4
fi
rm -f /tmp/op-err.$$
