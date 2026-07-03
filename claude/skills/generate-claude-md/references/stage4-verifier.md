# Stage 4 Verifier — Reference

This document defines the full verification pipeline for Stage 4: Checklist Verification, Iterative Fix Loop, and Blind Reviewer.

---

## Phase 1 — Checklist Verification

### Agent Definition

- `subagent_type`: `general-purpose`
- `model`: sonnet
- `run_in_background`: false

### 10-Item Checklist

Apply line by line to every generated/modified file:

1. **Universality / Necessity / Redundancy (prune test)**: Apply the authoritative prune test (claude-code-best-practices.md) — *"Would removing this cause Claude to make mistakes?"* If not, cut it. Also: does it apply to all tasks? Is it obvious from reading the code? → If it fails the prune test, delete or move to AGENTS.md / contributing-docs/
2. **Linter role**: Is this a code style rule? → Delete and recommend replacing with a linter or hook
3. **Speculation exclusion**: Does it include anything not confirmed in Stage 1–2? → If so, delete
4. **Verifiability**: Can compliance with each instruction be verified? → If not, make it concrete
5. **Size constraints**: Root CLAUDE.md under 100 lines soft / **200 hard** (official ceiling, claude-code-best-practices.md)? Nested CLAUDE.md under 50 lines (hard limit 100)? Individual instructions under 50 items? → If exceeded, consolidate, delete, or split into `.claude/rules/`
6. **Hierarchy / Scope**: Does CLAUDE.md reference contributing-docs/ directly (should go via AGENTS.md)? Does AGENTS.md contain Claude-specific content only? Does a nested CLAUDE.md cover content outside its directory or repeat parent content? Do rules/ files duplicate CLAUDE.md content? Are glob-scopeable rules using alwaysApply: true? Do rules/ files overlap in role with contributing-docs/? Can alwaysApply: true rules without path scoping be moved to CLAUDE.md? → If any, move/delete or replace with parent reference
7. **Reference integrity**: Are relative paths in nested CLAUDE.md valid? → Verify relative paths
8. **Discoverability**: Can an agent learn this by reading the code? → If yes, delete
9. **Staleness risk**: Does the line reference specific versions, tool names, or dependencies that may become inaccurate within 6 months? → If risky, delete or add expiry comment
10. **Static instruction problem**: Is this an unconditional instruction that applies identically to all task types? → If it can be made conditional, add explicit conditions

### Anti-Pattern Detection

Warn the user when any of the following are detected:

- **Over-specified CLAUDE.md** (primary anti-pattern, claude-code-best-practices.md): the file is long enough that real rules get lost in the noise and adherence drops. Fix: ruthlessly prune, or convert a must-run rule to a hook
- **Auto-generated content**: LLM-summarized content of the codebase is included verbatim
- **Information duplication**: Content already present in README, CONTRIBUTING.md, or CI configuration is repeated
- **Stale content**: Technologies, dependencies, or patterns are described that do not match the current codebase

### Self-Test Questions

- "Would a senior engineer look at this CLAUDE.md and say 'this is too much'?"
- "Is the CLAUDE.md → AGENTS.md → contributing-docs/ hierarchy and the CLAUDE.md → .claude/rules/ path clearly separated?"
- "Would root CLAUDE.md alone be sufficient for work in a nested directory, making the nested file removable?" → If sufficient, recommend deleting the nested file
- "Do any nested CLAUDE.md files contradict each other?"
- "If this line were deleted, would an agent reading the code reach the same conclusion?" → If yes, delete
- "Can this item be solved by code / linter / CI?" → If yes, recommend fixing in code and removing the item

### Output Format

Return a structured report:

```
VERIFICATION REPORT
===================
PASS items: [count]
FAIL items: [count]
WARNINGS: [count]

FAIL — [Checklist item number]: [Specific line quoted] — [Reason]
WARNING — [Anti-pattern name]: [Location] — [Description]
```

---

## Phase 2 — Iterative Fix Loop

**Maximum iterations: 3**

### Flow

```
Verify (Phase 1)
  → Any FAIL items?
      No  → Proceed to Phase 3
      Yes → Apply fixes → Re-verify
              → Any FAIL items?
                  No  → Proceed to Phase 3
                  Yes → Apply fixes → Re-verify (iteration 3)
                          → Report remaining FAILs to user → Proceed to Phase 3
```

### Fix Rules

- **Orchestrator applies fixes**, not the Verifier agent
- Address only specific FAIL items from the Verification Report
- Do not introduce new content; only remove or restructure existing content
- Track which FAIL items have been resolved across iterations
- After iteration 3: if FAIL items remain, surface them to the user with explanation and proceed

---

## Phase 3 — Blind Reviewer

### Trigger

Spawn the Blind Reviewer when output includes more than a single root CLAUDE.md (i.e., AGENTS.md, contributing-docs/, or nested CLAUDE.md files are included).

### Skip Conditions

- Generated output is a single root CLAUDE.md only
- User explicitly requested fast generation

### Agent Definition

- `subagent_type`: `general-purpose`
- `model`: fable (cross-model review — catches blind spots shared by the sonnet-based Phase 1/2 pipeline)
- `run_in_background`: false (orchestrator must receive results before final report)

### Fallback (fable unavailable)

If the fable dispatch fails with an Agent-call error (model outage, unsupported environment):

1. Re-dispatch the identical prompt with `model: sonnet` (the previous convention model). Do not skip the review.
2. State the substitution in the final report — "Review model: sonnet (fable unavailable fallback)" — so the user knows the review strength differs.
3. If the fallback dispatch also fails, escalate via the existing advisor path (condition ③).

The blind-review independence constraints below apply unchanged to the fallback run — only the model changes.

### What to Provide

Generated file contents **only**.

### What NOT to Provide

- Stage 1 or Stage 2 analysis results
- User interview answers
- Orchestrator reasoning or internal notes
- Verifier results from Phase 1/2

Providing only the generated files ensures the Reviewer evaluates independently without context bias.

### Reviewer Prompt

```
You are reviewing generated CLAUDE.md and related files. You did NOT write these.
Review independently using these criteria:

Files to review: {list_of_generated_file_paths}

1. Discoverability: Does each line pass the test "Can an agent learn this by reading the code?" If yes → flag for removal
2. Staleness risk: Does any line reference specific versions, tool names, or dependencies that may become inaccurate within 6 months? → flag with reason
3. Redundancy: Is any content duplicated between CLAUDE.md, AGENTS.md, and contributing-docs/? → flag the duplicate
4. Hierarchy: Does CLAUDE.md reference contributing-docs/ directly (should go through AGENTS.md)? → flag
5. Nested CLAUDE.md: Does any nested file repeat content from root CLAUDE.md? Does scope exceed its directory? → flag
6. Size: Is root CLAUDE.md under 100 lines soft / 200 hard (official ceiling)? Nested under 50 (hard limit 100)? Flag any line that fails the prune test ("would removing this cause a mistake?")
7. Actionability: Is every instruction verifiable? Any vague guidance? → flag with suggestion

Report: PASS/FAIL per criterion. For each FAIL, quote the specific line and explain why.
Do NOT fix issues — only report them.
```

### After Reviewer

The orchestrator reads the Reviewer report and applies fixes for all FAIL items before producing the final output.

---

## Update Mode Additional Checks

When running in update mode, apply these checks in addition to the common checklist:

- Were any lines that should not have changed modified unintentionally?
- Are all reference paths still valid after the modification?
- Is cross-file consistency maintained? (Re-verify U2 axis 3)
