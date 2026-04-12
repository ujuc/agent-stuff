---
name: skill-improver
description: "스킬/에이전트 정의를 테스트 시나리오 기반으로 자동 개선한다. 구조 검증 후 심층 최적화가 필요하면 autoresearch로 위임한다. skill-improver, 스킬 개선해줘, 스킬 최적화, 스킬 테스트해줘, test skills 요청 시 사용한다."
model: sonnet
allowed-tools: Read, Write, Edit, Glob, Grep, Bash(bash:*), Bash(git:*), Agent, Skill, advisor
argument-hint: "[skill-name ...]"
---

# Skill Improver

Test-driven improvement loop for skills and agent definitions. Validates structure and semantics, auto-fixes issues, and re-verifies — up to 3 iterations per target.

## Phase 0 — Pre-flight Checks

Before any validation work, verify the environment:

1. Check `yq` is installed:
   ```bash
   command -v yq &>/dev/null || { echo "yq is required: brew install yq"; exit 1; }
   ```
2. Check `validate-skill.sh` exists at `skills/generate-skills/scripts/validate-skill.sh` (relative to the repo root). If not found, report the missing path and stop.
3. Verify the current working directory is the agent-stuff repository by checking for `CLAUDE.md` and `claude/skills/` directory.

If any check fails, report the issue with an actionable fix and stop — do not proceed to Phase 1.

## Phase 1 — Inventory & Intent Extraction

1. If arguments specify skill names, target those; otherwise target all skills in `skills/`
2. Collect ALL skills' descriptions across both `claude/skills/` and `.claude/skills/` in a single pass (needed for cross-skill analysis in Phase 2). Store as a name→description map.
3. For each target, read SKILL.md (or agent `.md` file) and extract:
   - Frontmatter: name, description, model, allowed-tools
   - Body: core procedure steps, constraints, prohibited actions
   - Referenced file paths mentioned in the body (`references/`, `scripts/`, agent paths)
   - Trigger keywords from the description
4. Summarize each target's intent in 1 line for the test generation phase

## Phase 2 — Test Scenario Generation

Generate tests per target using a **test category matrix** with three dimensions:

### Dimension A — Structural (validate-skill.sh)

Run `validate-skill.sh <path>` and capture results. This single execution covers all 19 structural checks (frontmatter format, naming, size limits). Do not duplicate these checks.

### Dimension B — Semantic (skill-improver's core value)

| Test | What it checks | How |
|------|----------------|-----|
| **B.1 Description-body alignment** | Description's WHAT clause matches actual procedure steps | Read procedure, compare with description. Flag if description claims capabilities not present in the body, or misses major capabilities |
| **B.5 Reference integrity** | All file paths in the body point to existing files | Glob/Read each referenced path. Flag broken references |
| **B.6 CLAUDE.md table sync** | Skill's entry in `claude/CLAUDE.md` matches its frontmatter | Compare triggers and model columns against actual frontmatter values |

> **Scope boundary**: Trigger completeness, trigger uniqueness, and model fitness checks are owned by the skill-engineer agent. Do not duplicate them here.

### Dimension C — Type-specific

- **Skills with scripts** (`scripts/` directory exists): Execute scripts with no args or `--help` to verify they produce usage output and non-zero exit code
- **Agent definitions** (`.md` files in `agents/`): Verify `model` field is set, description follows WHAT+WHEN format
- **Pipeline skills** (skills that reference other skill names): Verify referenced skill names exist as actual skill directories

For complex skills (multi-agent-orchestrator, autoresearch, etc.), call `advisor()` after generating semantic tests to review whether the scenarios adequately capture the skill's intent.

Each test is a concrete check with expected outcome (PASS criteria).

## Phase 3 — Test Execution & Capture

Execute tests in order: Dimension A → B → C.

For each test:

1. Run the check (Bash command, file read, or comparison)
2. Capture output and result
3. Classify:
   - **PASS**: result matches expectations
   - **FAIL**: result does not match expectations
   - **WARN**: non-critical issue detected (e.g., optional field missing)
   - **SKIP**: test not applicable to this target type

**Early exit**: If Dimension A produces 3+ errors, skip Dimensions B and C for that target — structural problems must be fixed first.

Display results as a table after each target completes.

## Phase 4 — Failure Analysis & Auto-Fix

For each FAIL result:

1. Analyze the error pattern
2. Classify fixability and apply fixes:

### Auto-fixable (apply with Edit tool)

| Category | Trigger | Fix |
|----------|---------|-----|
| Frontmatter corrections | Missing fields, typos, invalid format | Add/correct frontmatter fields |
| Description WHAT enrichment | B.1 fails: description doesn't match procedure | Generate accurate WHAT clause from procedure steps. **Never modify the WHEN clause (trigger phrases) without user approval** |
| CLAUDE.md table sync | B.6 fails: table row doesn't match frontmatter | Update the row in `claude/CLAUDE.md` to match SKILL.md frontmatter (frontmatter is source of truth) |
| Reference path repair | B.5 fails: path is wrong but similar file exists | Fix the path if a similarly-named file exists nearby; otherwise report as manual |

### Manual (report to user, do not attempt)

- Cross-skill dependency issues (e.g., referenced skill doesn't exist)
- Core logic or workflow changes
- Description WHEN clause modifications (trigger phrases)
- Any structural issue requiring design decisions

When fixability classification is ambiguous (boundary between auto-fixable and manual), call `advisor()` to decide. Misclassifying can damage the skill's intent.

## Phase 5 — Re-verification (max 3 iterations)

1. After applying fixes, re-run only the tests that previously FAILED (PASS tests do not regress from metadata-only fixes)
2. **Regression guard**: If a fix introduces a NEW failure not present in the original run, immediately revert the fix and reclassify as manual
3. If all re-run tests PASS → proceed to Phase 6
4. If failures remain and iteration count < 3 → return to Phase 4
5. If iteration count reaches 3 → call `advisor()` to decide whether to continue, stop, or reconsider whether the test scenario itself is wrong

## Phase 6 — Summary & Commit

Output a changelog table:

```
## skill-improver Results

| Target | Tests | Iterations | Status | Changes |
|--------|-------|------------|--------|---------|
| commit | 6/6 PASS | 1 | Clean | no changes needed |
| generate-skills | 5/7 PASS | 2 | Improved | description enriched, CLAUDE.md synced |
```

If any fixes were applied:
1. Show the full diff to the user
2. Ask for confirmation before committing
3. Commit following Korean conventional commit rules:
   `refactor(skills): skill-improver로 <target> 스킬을 개선하다`

## Advisor Escalation

This skill runs on sonnet by default. Call `advisor()` (no parameters — full context is forwarded automatically) at these decision points:

1. **Phase 2 — semantic test quality review**: After generating tests for complex skills (multi-agent-orchestrator, autoresearch, etc.), review whether the scenarios capture the skill's cross-skill interactions and intent adequately
2. **Phase 4 — fixability classification ambiguity**: When a failure sits on the boundary between auto-fixable and manual. Misclassifying can damage the skill's intent
3. **Phase 5 — failures remain after 3 iterations**: To decide whether to keep auto-fixing, stop and escalate to the user, or reconsider whether the test scenario itself is wrong

## Phase 7 — Deep Optimization via Autoresearch (Optional)

After Phase 6, if the user requested "스킬 최적화" or deeper improvement beyond structural fixes:

1. Ask the user whether to proceed with eval-based optimization
2. If confirmed, invoke `Skill("autoresearch", args: "<target-skill-path>")` to delegate to autoresearch
3. Autoresearch will handle the eval → mutate → score → keep/discard loop on the skill's content

This phase is skipped for structural-only runs ("스킬 테스트해줘", "test skills").

## Constraints

- Never modify a skill's core logic or workflow without user approval
- Auto-fixes are limited to metadata, descriptions, and structural issues
- Always show diffs before committing
- Do not run the target skill itself (only validate its structure and content)
- Respect existing validate-skill.sh at `skills/generate-skills/scripts/validate-skill.sh`
- When run via the `maintain` skill in `full` mode, check for existing skill-engineer output before running redundant checks
- Trigger overlap, trigger completeness, and model fitness checks belong to skill-engineer — do not duplicate

## Gotchas

- **yq dependency**: validate-skill.sh silently fails with a parse error if yq is missing. Phase 0 prevents this.
- **Description enrichment risk**: Auto-generating a WHAT clause can accidentally remove trigger keywords the user placed intentionally. Always show the diff for description changes and never touch the WHEN clause.
- **CLAUDE.md target**: The skills table lives in `claude/CLAUDE.md`, NOT the project root `CLAUDE.md`. The sync fix must target the correct file.
