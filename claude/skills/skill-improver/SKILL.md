---
name: skill-improver
description: "스킬/에이전트 정의를 테스트 시나리오 기반으로 자동 개선한다. 7일 주기로 세션 시작 시 비차단 알림이 뜨고, 구조 검증 후 심층 최적화가 필요하면 autoresearch로 위임한다. skill-improver, 스킬 개선해줘, 스킬 최적화, 스킬 테스트해줘, test skills 요청 시 사용한다."
group: meta
model: sonnet
allowed-tools: Read, Write, Edit, Glob, Grep, Bash(bash:*), Bash(git:*), Bash(date:*), Agent, Skill, advisor
argument-hint: "[skill-name ...]"
---

# Skill Improver

Test-driven improvement loop for skills and agent definitions. Validates structure and semantics, auto-fixes safe issues, and re-verifies — up to 3 iterations per target. Runs on a 7-day cadence via the session-start protocol.

## Periodic Execution

This skill is meant to run regularly, not just on demand.

- The session-start protocol in `~/.claude/CLAUDE.md` ("Skills (Local Policy)" section) reads `~/.claude/.last_skill_improver_run`. If the date is older than 7 days (or the file is missing), it surfaces a non-blocking prompt offering to run a full sweep.
- **On decline**: the session-start hook writes today's date so the prompt does not repeat next session.
- **On accept**: the session-start hook does NOT write the timestamp; Phase 6 of this skill writes it only on successful completion. If the run crashes mid-flight (Phase 0–5 errors), the user is re-prompted next session — this is the desired "failed runs re-prompt" behavior (Gotcha #4).

To force an immediate run regardless of cadence: invoke `Skill("skill-improver")` directly.

## Language Policy

When auto-editing skill or agent metadata in Phase 4, preserve the user's language conventions:

- **`description` field is Korean.** Korean trigger phrases (`업데이트`, `수정`, `테스트해줘`, etc.) are matched against user utterances. Never translate the WHEN clause; never replace Korean keywords with English equivalents.
- **SKILL.md body is English.** Auto-fixes that touch body content keep English. Do not introduce Korean prose into bodies.
- **Trigger keywords are functional identifiers.** Treat them like code — they cannot be paraphrased.

If a target violates the policy (Korean prose in body, English-only description with Korean triggers expected), record as finding **B.7 — language policy drift** and ask the user before translating.

## Phase 0 — Pre-flight Checks

1. **Toolchain**: check `cargo` is installed (validate-skill is a Rust binary):
   ```bash
   command -v cargo &>/dev/null || { echo "cargo required: https://rustup.rs"; exit 1; }
   ```
2. **Validator path**: confirm `<repo-root>/agents/claude/skills/generate-skills/scripts/validate-skill` (no `.sh` suffix) exists. If not found, report and stop.
3. **Repo guard**: verify CWD is the agent-stuff repository (`CLAUDE.md` and `claude/skills/` present).
4. **Spec freshness**: read the YAML frontmatter at the top of generate-skills' `frontmatter-spec.md` reference doc (under `agents/claude/skills/generate-skills/`). Compute `today - last_upstream_check`. If beyond `check_interval_days` (default 14), log a warning suggesting `/generate-skills` to refresh upstream spec — do not block, but findings about new frontmatter fields may be stale.

If any toolchain/path/repo check fails, report the issue with an actionable fix and stop — do not proceed to Phase 1.

## Phase 1 — Inventory & Intent Extraction

1. **Argument parsing**: if arguments specify skill or agent names, target those; otherwise sweep all skills in `claude/skills/` and `.claude/skills/`, plus all agent definitions in `agents/claude/agents/` (and equivalents for other hosts).
2. **Mode classification**: tag each target as `skill` (has `SKILL.md`) or `agent` (`.md` file under `agents/<host>/agents/`). Mode determines which Phase 2 dimensions apply.
3. **Cross-skill map**: collect ALL skills' descriptions across both `claude/skills/` and `.claude/skills/` in a single pass (needed for B.6 CLAUDE.md sync). Store as name→description map.
4. **Per-target read**: for each target, parse:
   - Frontmatter: `name`, `description`, `model`, `allowed-tools`, plus optional fields per `frontmatter-spec.md`.
   - Body: core procedure steps, constraints, prohibited actions.
   - Referenced file paths in the body (`references/`, `scripts/`, agent paths).
   - Trigger keywords from the description.
5. Summarize each target's intent in 1 line for Phase 2.

## Phase 2 — Test Scenario Generation

Generate tests using a **test category matrix** with three skill dimensions plus a mode-specific dimension D for agents.

### Dimension A — Structural (skill mode only)

Run `validate-skill <path>` (Rust binary, not the legacy `.sh`). This single execution covers all structural checks (frontmatter format, naming, size limits, **`group` field presence and slug validity**). Do not duplicate in Dimension B. **Skip for agent mode** — no equivalent validator exists yet; rely on Dimension D.

### Dimension B — Semantic (skill-improver's core value)

| Test | What it checks | How |
|------|----------------|-----|
| **B.1 Description-body alignment** | Description's WHAT clause matches actual procedure steps | Read procedure, compare with description. Flag if description claims capabilities not present in the body, or misses major capabilities |
| **B.5 Reference integrity** | All file paths in the body point to existing files | Glob/Read each referenced path. Flag broken references. **Skip for agent files** unless body explicitly mentions external paths |
| **B.6 CLAUDE.md table sync** | Skill's entry in `claude/CLAUDE.md` matches its frontmatter | Compare triggers and model columns against actual frontmatter values. **Tolerate trailing annotations on the model column** (e.g., `sonnet + advisor`, `opus + tool`) — strip ` + <annotation>` before comparing the model token to frontmatter `model:` |
| **B.7 Language policy** | Description Korean, body English, trigger keywords intact | Parse frontmatter description for Korean characters; scan body for non-trivial Korean prose |

> **Scope boundary**: trigger completeness, trigger uniqueness, and model fitness checks belong to the skill-engineer agent. Do not duplicate them here.

### Dimension C — Type-specific (skills)

- **Skills with scripts** (`scripts/` directory exists): execute scripts with no args or `--help` to verify usage output and non-zero exit code.
- **Pipeline skills** (skills that reference other skill names): verify referenced skill names exist as actual skill directories.

### Dimension D — Agent-specific (agent mode only)

See "Agent Definition Mode" section below for the full check list. Quick summary: `model` field present, description follows WHAT + WHEN, body has a clear role statement, structured-output spec when applicable.

For complex skills (multi-agent-orchestrator, autoresearch, etc.), call `advisor()` after generating semantic tests to review whether scenarios capture the skill's intent adequately.

Each test is a concrete check with expected outcome (PASS criteria).

## Agent Definition Mode

When the target is an agent `.md` file (not a `SKILL.md`):

| Check | Required | Notes |
|-------|----------|-------|
| `name` frontmatter field | Yes | kebab-case, matches filename |
| `description` frontmatter field | Yes | WHAT + WHEN format |
| `model` frontmatter field | Yes | One of `sonnet`, `opus`, `haiku` |
| `tools` field | Optional | Comma-separated list when restricted |
| Role statement in body | Yes | First non-frontmatter paragraph defines the role |
| Output format spec | Conditional | Required if the agent produces structured output |
| External path references | Optional | Validate via B.5 only when present |

**Skipped vs skill mode**: Dimension A (no agent-side validator), Dimension C (no `scripts/` siblings), B.5 by default (skip unless paths in body), B.6 by default (agents are not in the skills CLAUDE.md table — check `claude/CLAUDE.md`'s Agents section if present, otherwise SKIP).

## Phase 3 — Test Execution & Capture

Execute tests in order: Dimension A → B → C/D.

For each test:

1. Run the check (Bash command, file read, or comparison).
2. Capture output and result.
3. Classify:
   - **PASS**: result matches expectations.
   - **FAIL**: result does not match expectations.
   - **WARN**: non-critical issue detected (e.g., optional field missing).
   - **SKIP**: test not applicable to this target type.

**Early exit**: if Dimension A produces 3+ errors, skip remaining dimensions for that target — structural problems must be fixed first.

Display results as a table after each target completes.

## Phase 4 — Failure Analysis & Auto-Fix

For each FAIL result:

1. Analyze the error pattern.
2. Classify fixability and apply fixes.

### Auto-fixable (apply with Edit tool)

| Category | Trigger | Fix |
|----------|---------|-----|
| Frontmatter corrections | Missing fields, typos, invalid format | Add/correct frontmatter fields |
| Description WHAT enrichment | B.1 fails | Generate accurate WHAT clause from procedure steps. **Never modify the WHEN clause (trigger phrases) without user approval** |
| CLAUDE.md table sync | B.6 fails | Update the row in `claude/CLAUDE.md` to match SKILL.md frontmatter (frontmatter is source of truth) |
| Reference path repair | B.5 fails | Fix the path if a similarly-named file exists nearby; otherwise report as manual |
| Language policy hint | B.7 fails | Report only — never auto-translate without user approval |

### Manual (report to user, do not attempt)

- Cross-skill dependency issues (e.g., referenced skill doesn't exist).
- Core logic or workflow changes.
- Description WHEN clause modifications (trigger phrases).
- Body language translations (B.7 prose drift).
- **Missing `group` field** — guessing from directory name or description risks wrong placement (e.g., a `frontend-*` skill might belong to `verify` or `build`). Surface the failure with the 8-slug list and ask the user to choose.
- Any structural issue requiring design decisions.

When fixability classification is ambiguous, call `advisor()` to decide. Misclassifying can damage the skill's intent.

## Phase 5 — Re-verification (max 3 iterations)

1. After applying fixes, re-run only the tests that previously FAILED.
2. **Regression guard**: if a fix introduces a NEW failure not present in the original run, immediately revert the fix and reclassify as manual.
3. If all re-run tests PASS → proceed to Phase 6.
4. If failures remain and iteration count < 3 → return to Phase 4.
5. If iteration count reaches 3 → call `advisor()` to decide whether to continue, stop, or reconsider whether the test scenario itself is wrong.

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

1. Show the full diff to the user.
2. Ask for confirmation before committing.
3. Commit following Korean conventional commit rules:
   `refactor(skills): skill-improver로 <target> 스킬을 개선하다`

After the report (with or without fixes), update the periodic-run timestamp:

```bash
date -u +%Y-%m-%d > ~/.claude/.last_skill_improver_run
```

This signals to the session-start protocol that skill-improver has run today, preventing repeat notifications next session. **Do not write the timestamp earlier in the workflow** — failed runs (Phase 0–5 errors) should re-prompt next session.

## Advisor Escalation

This skill runs on sonnet by default. Call `advisor()` (no parameters — full context is forwarded automatically) at these decision points:

1. **Phase 2 — semantic test quality review**: after generating tests for complex skills (multi-agent-orchestrator, autoresearch, etc.), review whether scenarios capture cross-skill interactions and intent adequately.
2. **Phase 4 — fixability classification ambiguity**: when a failure sits on the boundary between auto-fixable and manual.
3. **Phase 5 — failures remain after 3 iterations**: to decide whether to keep auto-fixing, stop and escalate, or reconsider the test scenario.

## Phase 7 — Deep Optimization via Autoresearch (Optional)

After Phase 6, if the user requested "스킬 최적화" or deeper improvement beyond structural fixes:

1. Ask the user whether to proceed with eval-based optimization.
2. If confirmed, invoke `Skill("autoresearch", args: "<target-skill-path>")` to delegate.
3. Autoresearch handles the eval → mutate → score → keep/discard loop on the skill's content.

This phase is skipped for structural-only runs ("스킬 테스트해줘", "test skills").

## Constraints

- Never modify a skill's core logic or workflow without user approval.
- Auto-fixes are limited to metadata, descriptions, and structural issues.
- Always show diffs before committing.
- Do not run the target skill itself (only validate its structure and content).
- Validator path is `agents/claude/skills/generate-skills/scripts/validate-skill` (no `.sh` suffix).
- When run via the `maintain` skill in `full` mode, check for existing skill-engineer output before running redundant checks.
- Trigger overlap, completeness, and model fitness checks belong to skill-engineer — do not duplicate.
- Phase 6 timestamp write is the only side effect outside of skill files and CLAUDE.md.

## Gotchas

1. **cargo dependency**: `validate-skill` is a Rust binary launched via `scripts/validate-skill`. First invocation compiles the workspace (~6–30s). Phase 0 must check `cargo`, not `bash` or `yq`. The launcher lost its `.sh` suffix in 2026-04 — older docs may still reference `validate-skill.sh`.

2. **Description enrichment risk**: auto-generating a WHAT clause can accidentally remove trigger keywords the user placed intentionally. Always show the diff for description changes and never touch the WHEN clause.

3. **CLAUDE.md target**: the skills table lives in `claude/CLAUDE.md`, NOT the project root `CLAUDE.md`. The B.6 sync fix must target the correct file.

4. **Periodic-run timestamp drift**: if skill-improver crashes mid-Phase 4 without reaching Phase 6, the timestamp is not updated and the user gets re-prompted next session. This is desired (failed runs re-prompt) — do not move the write earlier.

5. **Agent definition files lack `references/` siblings**: B.5 reference-integrity must skip agent files unless the body explicitly mentions external paths.

6. **Spec staleness ≠ blocker**: Phase 0's spec freshness check is informational. Stale `frontmatter-spec.md` only means new fields might be unknown; it does not invalidate existing checks. Warn the user but continue.

## Eval Criteria

Five binary checks for autoresearch reuse:

```
EVAL 1: Phase 0 environment guard
  Question: When cargo or the validate-skill launcher is missing, does the
            skill stop with a clear actionable message instead of crashing
            in Phase 2?
  Pass: Stops with the install instruction; no Phase 1+ work attempted.
  Fail: Continues into Phase 1 with broken environment.

EVAL 2: Description preservation
  Question: After auto-fixing a skill's description, are all original Korean
            trigger keywords still present?
  Pass: Diff shows only WHAT clause changes; WHEN/trigger phrases intact.
  Fail: Any Korean trigger keyword removed or translated.

EVAL 3: Regression guard effectiveness
  Question: When a Phase 4 fix introduces a NEW failure, is the fix reverted
            before Phase 5 records the new failure permanently?
  Pass: Fix reverted, target reclassified as manual, original test result
        restored.
  Fail: New failure persists in final report.

EVAL 4: Iteration ceiling
  Question: Does the skill stop auto-fixing at iteration 3 and escalate to
            advisor() instead of looping indefinitely?
  Pass: Exits the loop at 3, advisor() is invoked.
  Fail: Continues past 3 or silently gives up.

EVAL 5: Timestamp update
  Question: After Phase 6 completes (with or without fixes), does
            ~/.claude/.last_skill_improver_run contain today's UTC date?
  Pass: File contains YYYY-MM-DD matching today.
  Fail: File missing, stale, or contains malformed date.

EVAL 6: Group field enforcement
  Question: When a SKILL.md is missing the local-required `group` field
            (or has a slug outside the 8 allowed values), does the run
            classify the failure as manual and surface the 8-slug choice
            list to the user?
  Pass: Phase 4 reports it as manual, no auto-fix attempted, the user
        sees the slug list for their decision.
  Fail: skill-improver auto-fills a guessed group, or treats it as a
        warning without surfacing it.
```
