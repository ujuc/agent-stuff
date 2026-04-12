---
name: skill-improver
description: "스킬/에이전트 정의를 테스트 시나리오 기반으로 자동 개선한다. skill-improver, 스킬 개선해줘, 스킬 테스트해줘, test skills 요청 시 사용한다."
model: sonnet
allowed-tools: Read, Write, Edit, Glob, Grep, Bash(bash:*), Bash(git:*), Agent, advisor
argument-hint: "[skill-name ...]"
---

# Skill Improver

Test-driven improvement loop for skills and agent definitions. Generates test scenarios, runs them, analyzes failures, auto-fixes issues, and re-verifies — up to 3 iterations per target.

## Phase 1 — Inventory & Intent Extraction

1. If arguments specify skill names, target those; otherwise target all skills in `skills/`
2. For each target, read SKILL.md (or agent `.md` file) and extract:
   - Frontmatter: name, description, model, allowed-tools
   - Body: core procedure steps, constraints, prohibited actions
3. Summarize each target's intent in 1 line for the test generation phase

## Phase 2 — Test Scenario Generation

Generate 3 test scenarios per target:

| Type | Purpose | Example |
|------|---------|---------|
| **Happy path** | Normal input produces expected output | `validate-skill.sh <path>` exits 0 |
| **Edge case** | Boundary values, unusual but valid input | SKILL.md body exceeds 500 lines |
| **Error case** | Invalid input/environment → graceful error | Script with no args → usage + non-zero exit |

Test types by target category:

- **Skills with scripts** (gemma, generate-skills): execute scripts directly via Bash
- **Structure-only skills**: run `validate-skill.sh` + custom frontmatter/content checks
- **Agent definitions**: verify required fields exist, referenced file paths are valid

Each test is a concrete, executable shell command with expected exit code and output pattern.

## Phase 3 — Test Execution & Capture

For each test scenario:

1. Run the test command via Bash
2. Capture stdout, stderr, and exit code
3. Classify result:
   - **PASS**: exit code and output match expectations
   - **FAIL**: exit code or output mismatch
   - **WARN**: non-critical issue detected (e.g., optional field missing)
   - **SKIP**: test not applicable to this target type

Display results as a table after each target completes.

## Phase 4 — Failure Analysis & Auto-Fix

For each FAIL result:

1. Analyze the error pattern (missing field, broken path, script bug, etc.)
2. Classify fixability:
   - **Auto-fixable**: frontmatter typos, missing fields, broken references, description gaps
   - **Manual**: logic errors, design issues, external dependency problems
3. Apply auto-fixes using Edit tool:
   - Frontmatter field corrections
   - Description trigger keyword enrichment
   - Reference path repairs
   - Script argument handling improvements
4. Report manual-fix items to user without attempting changes

## Phase 5 — Re-verification (max 3 iterations)

1. After applying fixes, re-run all FAIL tests from Phase 3
2. If all tests PASS → proceed to Phase 6
3. If failures remain and iteration count < 3 → return to Phase 4
4. If iteration count reaches 3 → stop and report remaining failures

## Phase 6 — Summary & Commit

Output a changelog table:

```
## skill-improver Results

| Target | Tests | Iterations | Status | Changes |
|--------|-------|------------|--------|---------|
| generate-skills | 3/3 PASS | 2 | Improved | description enriched, frontmatter fixed |
| health-checker | 2/2 PASS | 1 | Clean | no changes needed |
```

If any fixes were applied:
1. Show the full diff to the user
2. Ask for confirmation before committing
3. Commit following Korean conventional commit rules:
   `refactor(skills): skill-improver로 <target> 스킬을 개선하다`

## Advisor Escalation

This skill runs on sonnet by default. At the decision points below, call `advisor()` to borrow higher-tier reasoning:

- **Phase 4 — fixability classification is ambiguous**: when a failure sits on the boundary between Auto-fixable (frontmatter typos, path repairs) and Manual (logic or design issues). Misclassifying can damage the skill's intent.
- **Phase 5 — when failures remain after 3 iterations**: to decide whether to keep auto-fixing, stop and escalate to the user, or reconsider whether the test scenario itself is wrong.

How to call: invoke `advisor()` with no parameters. The full current conversation context (test results, failure patterns, prior fix history) is automatically forwarded to the higher-tier model. Use this to check whether the underlying approach is flawed.

## Constraints

- Never modify a skill's core logic or workflow without user approval
- Auto-fixes are limited to metadata, descriptions, and structural issues
- Always show diffs before committing
- Do not run the target skill itself (only validate its structure and scripts)
- Respect existing validate-skill.sh at `skills/generate-skills/scripts/validate-skill.sh`
