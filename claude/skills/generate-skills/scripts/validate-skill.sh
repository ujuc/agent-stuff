#!/usr/bin/env bash
# validate-skill.sh — Validate a Claude skill directory structure and frontmatter.
# Usage: validate-skill.sh <skill-directory>
# Requires: yq (brew install yq)

set -euo pipefail

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# --- Helpers ---
pass() { echo -e "  ${GREEN}✓${NC} $1"; }
fail() { echo -e "  ${RED}✗${NC} $1"; errors=$((errors + 1)); }
warn() { echo -e "  ${YELLOW}!${NC} $1"; }

# --- Args ---
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <skill-directory>"
  exit 1
fi

SKILL_DIR="${1%/}"
SKILL_FILE="${SKILL_DIR}/SKILL.md"
FOLDER_NAME="$(basename "$SKILL_DIR")"
errors=0

echo "Validating skill: ${SKILL_DIR}"
echo ""

# --- Dependency check ---
if ! command -v yq &>/dev/null; then
  echo -e "${RED}Error:${NC} yq is not installed."
  echo "  Install with: brew install yq"
  exit 1
fi

# --- Check 1: SKILL.md exists ---
echo "Structure checks:"
if [[ ! -f "$SKILL_FILE" ]]; then
  fail "SKILL.md not found (${SKILL_FILE})"
  echo ""
  echo -e "${RED}Fatal:${NC} SKILL.md is required. Cannot continue."
  exit 1
fi
pass "SKILL.md exists"

# --- Check 2: No README.md ---
if [[ -f "${SKILL_DIR}/README.md" ]]; then
  fail "README.md found (skill folders must not contain README.md)"
else
  pass "No README.md"
fi

echo ""
echo "Frontmatter checks:"

# --- Extract frontmatter ---
FRONTMATTER=$(yq --front-matter=extract eval '.' "$SKILL_FILE" 2>/dev/null) || {
  fail "Failed to parse YAML frontmatter"
  echo ""
  echo -e "${RED}${errors} error(s) found.${NC}"
  exit 1
}

# --- Check 3: Frontmatter delimiters ---
FIRST_LINE=$(head -1 "$SKILL_FILE")
if [[ "$FIRST_LINE" == "---" ]]; then
  pass "Opening --- delimiter on line 1"
else
  fail "First line must be --- (got: '${FIRST_LINE}')"
fi

# --- Check 4: name field exists + kebab-case ---
NAME=$(echo "$FRONTMATTER" | yq '.name // ""')
if [[ -z "$NAME" ]]; then
  fail "name field is missing"
else
  pass "name field exists: ${NAME}"

  if [[ "$NAME" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
    pass "name is kebab-case"
  else
    fail "name is not kebab-case: '${NAME}' (expected pattern: ^[a-z0-9]+(-[a-z0-9]+)*\$)"
  fi
fi

# --- Check 5: name matches folder name ---
if [[ -n "$NAME" ]]; then
  if [[ "$NAME" == "$FOLDER_NAME" ]]; then
    pass "name matches folder name"
  else
    fail "name '${NAME}' does not match folder name '${FOLDER_NAME}'"
  fi
fi

# --- Check 6: description field exists + length ---
DESC=$(echo "$FRONTMATTER" | yq '.description // ""')
if [[ -z "$DESC" ]]; then
  fail "description field is missing"
else
  DESC_LEN=${#DESC}
  if [[ $DESC_LEN -le 1024 ]]; then
    pass "description exists (${DESC_LEN} chars, max 1024)"
  else
    fail "description exceeds 1024 chars (${DESC_LEN} chars)"
  fi
fi

# --- Check 7: No XML tags in frontmatter ---
if echo "$FRONTMATTER" | grep -qE '<[a-zA-Z][a-zA-Z0-9]*[^>]*>'; then
  fail "XML tags found in frontmatter"
else
  pass "No XML tags in frontmatter"
fi

# --- Check 8: name does not start with claude/anthropic ---
if [[ -n "$NAME" ]]; then
  if [[ "$NAME" == claude* ]] || [[ "$NAME" == anthropic* ]]; then
    fail "name must not start with 'claude' or 'anthropic': '${NAME}'"
  else
    pass "name does not use reserved prefix"
  fi
fi

# --- Summary ---
echo ""
if [[ $errors -eq 0 ]]; then
  echo -e "${GREEN}All checks passed.${NC}"
  exit 0
else
  echo -e "${RED}${errors} error(s) found.${NC}"
  exit 1
fi
