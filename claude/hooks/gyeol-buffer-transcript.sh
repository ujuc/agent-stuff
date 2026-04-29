#!/usr/bin/env bash
# gyeol-buffer-transcript.sh — Thin launcher invoking the Rust binary.
#
# Wired via the SessionEnd hook in ~/.claude/settings.json. Reads the JSON
# payload from stdin and writes a copy of the transcript into the gyeol
# session buffer for later `/eos` processing.
#
# Rebuilds the binary on the fly when the source has changed; otherwise
# execs the cached release artifact.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/tools/gyeol-buffer-transcript"
BINARY="$CRATE_DIR/target/release/gyeol-buffer-transcript"
SOURCE="$CRATE_DIR/src/main.rs"
MANIFEST="$CRATE_DIR/Cargo.toml"

if [[ ! -x "$BINARY" || "$SOURCE" -nt "$BINARY" || "$MANIFEST" -nt "$BINARY" ]]; then
    cargo build --release --quiet --manifest-path "$MANIFEST" >&2 || exit 0
fi

exec "$BINARY"
