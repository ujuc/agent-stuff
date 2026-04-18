#!/usr/bin/env bash
# fetch-api-key.sh — Thin launcher over the Rust workspace in tools/.
# See tools/gemma/src/keychain.rs for 1Password integration.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLS_DIR="${SCRIPT_DIR}/../tools"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found. Install Rust via rustup: https://rustup.rs" >&2
  exit 127
fi

exec cargo run \
  --manifest-path "${TOOLS_DIR}/Cargo.toml" \
  --bin gemma \
  --release \
  --quiet \
  -- fetch-api-key "$@"
