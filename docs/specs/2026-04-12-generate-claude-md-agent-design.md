# Design: generate-claude-md Stage-Based Agent Refactor

**Date**: 2026-04-12
**Status**: Draft
**Scope**: `agents/claude/skills/generate-claude-md/`

---

## Problem

The current `generate-claude-md` skill is a monolithic ~440-line SKILL.md that runs entirely on opus. It handles project analysis, user interview, file generation, and verification in a single file. This has three issues:

1. **Token inefficiency** — mechanical tasks (file scanning, template application) consume opus tokens unnecessarily
2. **No iterative verification** — Stage 4 runs a one-pass checklist with no fix-and-recheck loop
3. **Poor testability** — stages cannot be tested or improved independently

## Solution

Refactor into a **stage-based orchestrator** pattern: SKILL.md becomes a ~120-line flow controller that delegates each stage to purpose-built sub-agents. The orchestrator stays on opus; sub-agents run on sonnet. advisor(opus) provides independent second-opinion checks at 3 escalation points.

---

## Architecture

### Model Routing

| Component | Model | Role |
|-----------|-------|------|
| SKILL.md (orchestrator) | **opus** | Mode detection, flow control, interview, decisions |
| Stage 1 Explore agents × 3 | **sonnet** (explicit) | Project exploration |
| Stage 3 Generator agent | **sonnet** (explicit) | File generation |
| Stage 4 Verifier agent | **sonnet** (explicit) | Checklist verification + iterative fix |
| Stage 4 Reviewer agent | **sonnet** (explicit) | Independent blind review |
| advisor() | opus | Fresh-eyes structural review at 3 escalation points |

### File Structure (After)

```
skills/generate-claude-md/
├── SKILL.md                          # Orchestrator (~120 lines)
└── references/
    ├── SOUL.md                       # Existing — no change
    ├── karpathy-guidelines.md        # Existing — no change
    ├── osmani-guidelines.md          # Existing — no change
    ├── entry-router-guidelines.md    # Existing — no change
    ├── stage1-analyzer.md            # NEW: Stage 1 agent prompts + orchestration
    ├── stage3-generator.md           # NEW: Stage 3 generation rules + agent prompt
    ├── stage4-verifier.md            # NEW: Stage 4 verification loop + Reviewer prompt
    └── update-mode.md               # NEW: U1–U3 update mode procedures
```

**Deleted**: `subagent-guidelines.md` — its content is absorbed into each stage file.

---

## Stage Design

### Stage 1: Analyzer (`references/stage1-analyzer.md`)

Adopts the deep-read `.partial/` pattern for parallel exploration.

**Output path**: `.research/partials/claude-md-{type}.md` (3 files)

| Agent | subagent_type | model | Detection Target | Skip Condition |
|-------|--------------|-------|------------------|----------------|
| config-explorer | Explore | sonnet | Package/build/test/lint config | ≤2 config files |
| structure-explorer | Explore | sonnet | Monorepo/submodule/directory layout | Single-package repo |
| docs-explorer | Explore | sonnet | Docs/CI/existing CLAUDE.md | No docs or CI files |

**Flow**:

1. Orchestrator runs initial glob to assess project complexity
2. Complex project (3+ config types, monorepo indicators, submodules): spawn 3 Explore agents in parallel (`run_in_background: true`)
3. Simple project (≤2 config files, flat structure): orchestrator explores directly, no sub-agents
4. Collect results into `.research/partials/` → orchestrator merges
5. **advisor() call ①**: when the 3 agents report contradictory findings, or when discoverable/undiscoverable classification is ambiguous
6. Cleanup: delete `.research/partials/claude-md-*.md` after merge

**Prompt templates**: Each agent prompt follows the existing pattern from `subagent-guidelines.md` but is embedded directly in `stage1-analyzer.md`.

### Stage 2: Interview (Orchestrator — direct execution)

No delegation. The orchestrator handles this directly because:
- It requires user interaction (AskUserQuestion)
- Sub-agents cannot converse with the user

**Content** (preserved from current SKILL.md):
- WHY/WHAT/HOW interview framework
- Ask only about undiscoverable items from Stage 1
- Present assumptions from Stage 1 for user confirmation
- Optional Explore-Deep agent during AskUserQuestion wait (large monorepos only)

**Update mode integration**:
- After Stage 1: execute U1 (audit) from `update-mode.md`
- During interview: integrate U2 (drift comparison) results

**advisor() call ②**: when the scope of files to generate is ambiguous (e.g., whether to create nested CLAUDE.md vs. rules/ separation)

### Stage 3: Generator (`references/stage3-generator.md`)

Delegates file creation to **1 general-purpose sub-agent** (model: sonnet).

**What the orchestrator provides to the agent**:
- Stage 1 merged analysis (summary, not raw partials)
- Stage 2 interview answers
- Target file list with structure templates for each
- Condensed generation principles from the 4 guideline files

**What the agent does**:
- Generate all target files (CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, rules/)
- Apply discoverability test line-by-line
- Write files directly

**What the agent does NOT do**:
- Interview the user
- Make structural scope decisions (those are made in Stage 2)

**Update mode**: instead of generation, execute U3 (apply) — surgical Edit-based modifications per `update-mode.md`.

**Generation rules to include in stage3-generator.md**:
- Section A–E file specifications (currently lines 132–281 of SKILL.md)
- Common writing rules (discoverability test, no code snippets, file:line references only)
- Size constraints (root CLAUDE.md ≤100 lines, nested ≤50 lines, rules/ ≤50 lines per file)

### Stage 4: Verifier (`references/stage4-verifier.md`)

Adopts the skill-improver **iterative fix loop** pattern.

**Phase 1 — Checklist verification** (Verifier agent, sonnet):

10-item checklist applied line-by-line to all generated files:

1. Universality/necessity/redundancy
2. Linter role check
3. Speculation exclusion
4. Verifiability
5. Size constraints
6. Hierarchy/scope
7. Reference integrity
8. Discoverability
9. Staleness risk
10. Static instruction check

**Phase 2 — Iterative fix loop** (max 3 iterations):

```
Verify → violations found?
  ├─ No → Pass to Phase 3
  └─ Yes → auto-fix violations → re-verify (iteration N+1)
              └─ 3 iterations exhausted → report remaining issues
```

**Phase 3 — Blind Reviewer** (separate Reviewer agent, sonnet):

- Receives ONLY the generated file contents
- Does NOT receive Stage 1/2 analysis or orchestrator reasoning
- Evaluates independently against 7 criteria (discoverability, staleness, redundancy, hierarchy, nested scope, size, actionability)
- Reports PASS/FAIL per criterion with specific line quotes

**After Reviewer**:

- Orchestrator receives Reviewer report
- FAIL items → orchestrator applies fixes
- **advisor() call ③**: when verification fails 2+ times, or when the fix direction is unclear (e.g., Verifier and Reviewer disagree on whether a line is discoverable)

**Anti-patterns to detect**:
- Auto-generated content inclusion
- Information duplication with README/CONTRIBUTING.md/CI
- Stale references to outdated tech/dependencies

---

## Update Mode (`references/update-mode.md`)

Consolidates U1, U2, U3 (currently inline in SKILL.md lines 294–398) into a single reference file.

### U1: Audit (runs after Stage 1)

- Detect existing files: CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, rules/
- Summarize in table format (file, line count, status, notes)
- If no files exist: propose switching to generation mode

### U2: Drift Comparison (runs during Stage 2)

Three-axis comparison:
1. **Codebase drift**: current code state vs. documented content
2. **Principle re-application**: discoverability test, size constraints, staleness on all existing lines
3. **Structural integrity**: cross-file reference validation

Output: categorized report (Change Required / Remove Recommended / Add Candidate / No Change)

### U3: Apply (runs instead of Stage 3 generation)

- Surgical changes only (Edit tool, not Write)
- Bottom-up order: contributing-docs/ → rules/ → nested CLAUDE.md → AGENTS.md → root CLAUDE.md
- Show each change to user before applying
- File deletion: propose only, execute after user approval

---

## Advisor Escalation Points

| # | When | What to ask | Expected value |
|---|------|-------------|----------------|
| ① | Stage 1 merge | Agent results contradict or discoverable/undiscoverable classification is ambiguous | Resolve conflicts, validate classification |
| ② | Pre-Stage 3 | File scope/structure decision is unclear (nested CLAUDE.md vs. rules/ vs. AGENTS.md) | Structural decision with reasoning |
| ③ | Stage 4 repeated failure | Verification fails 2+ times, or Verifier/Reviewer disagree | Fix direction, whether to accept or escalate to user |

**Advisor is NOT called for**:
- Simple questions that sonnet agents can answer
- Routine checklist items
- File generation mechanics

---

## Migration Plan

### What moves where

| Current location (SKILL.md lines) | Destination |
|-----------------------------------|-------------|
| 1–6: Frontmatter | SKILL.md (updated) |
| 8–55: Mode detection + philosophy | SKILL.md (condensed) |
| 56–86: Stage 1 analysis | `references/stage1-analyzer.md` |
| 87–120: Stage 2 interview | SKILL.md (kept inline) |
| 121–281: Stage 3 generation (A–E) | `references/stage3-generator.md` |
| 282–398: U1–U3 update mode | `references/update-mode.md` |
| 399–439: Stage 4 verification | `references/stage4-verifier.md` |
| `references/subagent-guidelines.md` | Deleted — absorbed into stage files |

### What stays in SKILL.md (~120 lines)

1. Frontmatter (model: opus, allowed-tools)
2. Mode detection logic (generation vs. update, target file identification)
3. Generation philosophy summary (pointers to 4 guideline files)
4. Stage flow control (when to spawn agents, when to act directly)
5. Stage 2 interview (inline — requires user interaction)
6. Advisor escalation criteria (3 call points with trigger conditions)
7. Final output format

### Files unchanged

- `references/SOUL.md`
- `references/karpathy-guidelines.md`
- `references/osmani-guidelines.md`
- `references/entry-router-guidelines.md`

---

## Patterns Reused

| Pattern | Source skill | How it's applied |
|---------|-------------|------------------|
| `.partial/` parallel collection + merge | deep-read | Stage 1: 3 Explore agents write partials, orchestrator merges |
| Iterative verify → fix → re-verify loop (max 3) | skill-improver | Stage 4: Verifier runs checklist, auto-fixes, re-checks |
| Blind independent Reviewer | existing generate-claude-md | Stage 4: preserved and formalized as separate agent |
| model: sonnet + advisor escalation | deep-read, annotate-plan, skill-improver | Sub-agents on sonnet, advisor for structural decisions |

---

## Success Criteria

1. SKILL.md is ≤120 lines (orchestration only)
2. All 4 new reference files exist and are self-contained
3. `subagent-guidelines.md` is deleted
4. Sub-agents explicitly specify `model: "sonnet"`
5. advisor() is called at exactly 3 defined escalation points
6. Stage 4 includes iterative fix loop (max 3 iterations)
7. Stage 4 Blind Reviewer runs as a separate agent
8. Update mode (U1–U3) is consolidated in `update-mode.md`
9. Existing 4 guideline references are unchanged
10. Skill produces equivalent or better output quality compared to current version
