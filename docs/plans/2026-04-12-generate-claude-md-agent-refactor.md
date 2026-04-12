# generate-claude-md Stage-Based Agent Refactor — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the monolithic generate-claude-md SKILL.md (~467 lines) into a stage-based orchestrator (~120 lines) with 4 reference files, adopting sonnet sub-agents + opus advisor escalation.

**Architecture:** SKILL.md becomes a pure orchestrator controlling 4 stages. Each stage's detailed logic lives in `references/stage{N}-*.md`. Sub-agents run on sonnet (explicitly specified); orchestrator stays on opus. advisor() is called at 3 defined escalation points for independent structural review.

**Tech Stack:** Claude Code skills (SKILL.md + references/), Agent tool with model parameter, advisor() tool

**Spec:** `docs/specs/2026-04-12-generate-claude-md-agent-design.md`

---

## File Map

| Action | File | Responsibility |
|--------|------|----------------|
| Rewrite | `agents/claude/skills/generate-claude-md/SKILL.md` | Orchestrator: mode detection, flow control, Stage 2 interview, advisor calls |
| Create | `agents/claude/skills/generate-claude-md/references/stage1-analyzer.md` | Stage 1: project analysis agent prompts + orchestration logic |
| Create | `agents/claude/skills/generate-claude-md/references/stage3-generator.md` | Stage 3: file generation rules + agent prompt |
| Create | `agents/claude/skills/generate-claude-md/references/stage4-verifier.md` | Stage 4: verification loop + Reviewer agent prompt |
| Create | `agents/claude/skills/generate-claude-md/references/update-mode.md` | U1-U3: update mode audit, drift comparison, surgical apply |
| Delete | `agents/claude/skills/generate-claude-md/references/subagent-guidelines.md` | Absorbed into stage files |
| No change | `agents/claude/skills/generate-claude-md/references/SOUL.md` | — |
| No change | `agents/claude/skills/generate-claude-md/references/karpathy-guidelines.md` | — |
| No change | `agents/claude/skills/generate-claude-md/references/osmani-guidelines.md` | — |
| No change | `agents/claude/skills/generate-claude-md/references/entry-router-guidelines.md` | — |

---

### Task 1: Create `references/stage1-analyzer.md`

**Files:**
- Create: `agents/claude/skills/generate-claude-md/references/stage1-analyzer.md`
- Reference: `agents/claude/skills/generate-claude-md/SKILL.md:56-86` (current Stage 1)
- Reference: `agents/claude/skills/generate-claude-md/references/subagent-guidelines.md:34-132` (Explore agent prompts)
- Pattern reference: `agents/claude/skills/deep-read/SKILL.md` (`.partial/` pattern)

- [ ] **Step 1: Read source material**

Read the following to extract content:
- Current SKILL.md lines 56-86 (Stage 1 analysis logic)
- `references/subagent-guidelines.md` lines 34-132 (Explore-Config, Explore-Structure, Explore-Docs, Explore-Deep prompts)
- `agents/claude/skills/deep-read/SKILL.md` lines 20-65 (`.partial/` orchestration pattern)

- [ ] **Step 2: Write stage1-analyzer.md**

Create `agents/claude/skills/generate-claude-md/references/stage1-analyzer.md` with the following content:

```markdown
# Stage 1: Project Analyzer

> Parallel exploration of target project using 3 Explore agents.
> Adopts the deep-read `.partial/` collection pattern.

---

## Complexity Assessment

Before spawning agents, the orchestrator runs a quick glob to assess project complexity:

```
Glob: {target_path}/**/{package.json,Cargo.toml,pyproject.toml,go.mod,Gemfile,pom.xml,.gitmodules}
```

**Complex project** (any of: 3+ config file types, monorepo indicators, .gitmodules exists):
→ Spawn 3 Explore agents in parallel

**Simple project** (none of the above):
→ Orchestrator explores directly, skip agents

---

## Agent Definitions

All agents: `subagent_type: "Explore"`, `model: "sonnet"`, `run_in_background: true`

### config-explorer

**Skip condition**: ≤2 config files found in complexity assessment.

**Prompt**:

> Explore the project at {target_path} to find all package/build/test/lint/format configuration:
> 1. Package managers: package.json, Cargo.toml, pyproject.toml, go.mod, Gemfile, pom.xml
> 2. Test config: jest.config, vitest.config, pytest.ini, .mocharc
> 3. Lint/format: .eslintrc, .prettierrc, biome.json, ruff.toml, .golangci.yml, rustfmt.toml
> 4. Build: webpack, vite, tsconfig, Makefile, CMakeLists.txt, build.gradle
>
> Report: list each found file with its key fields (scripts, dependencies count, test command).
> Do NOT include file contents — summarize only.
> Write your findings to `.research/partials/claude-md-config.md`.

### structure-explorer

**Skip condition**: Single-package repository (no workspaces, no .gitmodules, no nested package managers).

**Prompt**:

> Analyze repository structure at {target_path}:
> 1. Monorepo detection: workspaces in package.json/pnpm-workspace.yaml, packages/, apps/
> 2. Submodules: parse .gitmodules for paths, URLs, independent repo status
> 3. Nested package managers: subdirectories with their own package.json/Cargo.toml/etc.
> 4. Directory tree: top 2 levels only, noting purpose of major directories
>
> Report: structure type (monorepo/single/hybrid), list of independent units with tech stack.
> Write your findings to `.research/partials/claude-md-structure.md`.

### docs-explorer

**Skip condition**: No documentation files or CI config detected in initial glob.

**Prompt**:

> Scan documentation and CI at {target_path}:
> 1. Existing AI config: CLAUDE.md (root + nested), AGENTS.md, .cursorrules, .github/copilot
> 2. Contributing docs: CONTRIBUTING.md, contributing-docs/, docs/
> 3. CI/CD: .github/workflows/*.yml, .gitlab-ci.yml, Jenkinsfile — extract test/build/deploy commands
> 4. Nested CLAUDE.md: list all paths, note content length and key sections
> 5. Existing .claude/rules/: file list, each file's description/globs/alwaysApply
>
> Report: list of files found with one-line summary of each. For existing CLAUDE.md files, note section headings and line count.
> Write your findings to `.research/partials/claude-md-docs.md`.

---

## Explore-Deep (Optional, Stage 2)

**Trigger**: Large monorepo (5+ packages) or complex project where Stage 1 results raised unanswered questions.

**Skip condition**: Stage 1 results are sufficient. Simple or medium-sized projects. User response arrived quickly.

Spawned during AskUserQuestion wait (`run_in_background: true`).

**Prompt**:

> Deep analysis of {target_path} based on Stage 1 gaps:
> 1. {specific_gap_1}: e.g., "Determine relationship between packages/core and packages/cli"
> 2. {specific_gap_2}: e.g., "Find external service dependencies (DB connections, API calls)"
> 3. Cross-package dependencies: which packages depend on which
> 4. Non-obvious patterns: custom build steps, code generation, unusual testing patterns
>
> Report: findings for each gap, with file:line references where relevant.

---

## Merge Protocol

After all agents complete:

1. Read `.research/partials/claude-md-config.md`, `claude-md-structure.md`, `claude-md-docs.md`
2. Merge into a unified analysis, classifying each finding as:
   - **Discoverable**: agent can learn this by reading the code → exclude from generation candidates
   - **Undiscoverable**: must be explicitly stated → include as generation candidate
3. Identify **facts** vs **assumptions** in the merged analysis
4. List items that automatic detection could NOT resolve
5. Prepare nested CLAUDE.md candidate table:
   | Path | Type (monorepo package/submodule) | Existing CLAUDE.md? | Recommend creation? |
6. Present merged results to user

## Cleanup

After merge is complete, delete partial files:
- `.research/partials/claude-md-config.md`
- `.research/partials/claude-md-structure.md`
- `.research/partials/claude-md-docs.md`
```

- [ ] **Step 3: Verify file was created correctly**

Run: `wc -l agents/claude/skills/generate-claude-md/references/stage1-analyzer.md`
Expected: ~95-105 lines

- [ ] **Step 4: Commit**

```bash
git -C agents add claude/skills/generate-claude-md/references/stage1-analyzer.md
git -C agents commit -m "feat(skills): generate-claude-md Stage 1 Analyzer 참조 파일을 추가하다"
```

---

### Task 2: Create `references/stage3-generator.md`

**Files:**
- Create: `agents/claude/skills/generate-claude-md/references/stage3-generator.md`
- Reference: `agents/claude/skills/generate-claude-md/SKILL.md:123-281` (current Stage 3 generation rules A-E)

- [ ] **Step 1: Read source material**

Read current SKILL.md lines 123-281 — the full generation rules for sections A through E.

- [ ] **Step 2: Write stage3-generator.md**

Create `agents/claude/skills/generate-claude-md/references/stage3-generator.md` with the following content:

```markdown
# Stage 3: Generator

> File generation rules and sub-agent prompt for creating CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, and .claude/rules/.
> Delegated to a single general-purpose agent (model: sonnet).

---

## Agent Definition

- `subagent_type`: `general-purpose`
- `model`: `"sonnet"`
- `run_in_background`: `false` (orchestrator waits for results)
- `description`: "Generate CLAUDE.md and related files"

## What the Orchestrator Provides

The orchestrator constructs a prompt containing:

1. **Stage 1 merged analysis** (summary — not raw partials)
2. **Stage 2 interview answers** from the user
3. **Target file list** with the structure template for each (from sections A-E below)
4. **Generation principles** (condensed):
   - Discoverability test: "Can an agent learn this by reading the code?" If yes → exclude (osmani-guidelines.md)
   - Simplicity first: minimum code solving the problem (karpathy-guidelines.md)
   - Surgical changes: modify only necessary content (karpathy-guidelines.md)
   - Entry Router governance: include CORE rules in Boundaries when project requires safety guardrails (entry-router-guidelines.md)

## Common Writing Rules

All generated files follow these rules:

- No code snippets — use file:line references only
- **Discoverability test**: for every line, ask "Can an agent learn this by reading the code?" If yes → do not include
- No auto-generated content: do not include LLM summaries of the codebase
- Only include facts confirmed in Stage 1-2. No speculation or "nice to have" items.

---

## Section A: Root CLAUDE.md

**Size constraint**: target ≤100 lines, hard limit 300 lines.

**Do NOT include**:
- Code style rules (delegate to linters/formatters)
- Direct references to contributing-docs/ (go through AGENTS.md)
- Content discoverable from code/config files

**Structure**:

```
# Project Overview
(WHY: 1-2 lines — project purpose, only if not in README)

# Technical Stack
(WHAT: core technologies only — omit section if obvious from package.json/go.mod/etc.)

# Development Commands
(HOW: build, test, lint commands — only if not in README/Makefile)

# Work Rules
(HOW: branch strategy, commit conventions, PR rules — universal)

# Behavioral Guidelines
(Undiscoverable project-specific constraints. E.g., "Always confirm before running DB migrations")

# References
- **[AGENTS.md](./AGENTS.md)** — Undiscoverable operational info, detailed guide
(List nested CLAUDE.md subdirectories here)
```

## Section B: AGENTS.md

**Standard**: agents.md/v1 format.

**Treat as a codesmell list**: each entry ideally should be solved by code/linter/CI. Remove entries when the codebase improves.

**Do NOT include**: Claude-specific content (that belongs in rules/).

**Structure**:

- YAML frontmatter (name, description, version, standard)
- Project Overview (only if not in README)
- Operational Gotchas: things agents cannot discover from code (external system behaviors, non-obvious ordering, environmental constraints)
- Non-Obvious Conventions: conventions not inferable from code patterns (only what linters don't enforce)
- Build & Test Gotchas: non-obvious build/test requirements only (not standard commands)
- Git Workflow: branch strategy, commit conventions (only if not in CONTRIBUTING.md)
- Boundaries: Always Do / Ask First / Never Do
- Contributing Docs reference section: list of contributing-docs/ files

## Section C: contributing-docs/ Separate Documents

Generate only those that apply to the project:

- `contributing-docs/architecture.md`: service structure, communication patterns, data flow
- `contributing-docs/building_the_project.md`: detailed build/deploy procedures
- `contributing-docs/testing.md`: test strategy, test data setup
- `contributing-docs/database.md`: schema structure, migration methods
- `contributing-docs/conventions.md`: code conventions not enforceable by linters
- `contributing-docs/behavioral.md`: project-specific behavioral constraints (if applicable)

Each document follows common writing rules. Keep concise.

## Section D: Nested CLAUDE.md (Monorepo Packages / Submodules)

**All conditions must be met**:
- Has its own package manager file OR is a git submodule
- Needs different tech stack, build commands, or work rules than root
- User approved creation in Stage 2

**Size constraint**: target ≤50 lines, hard limit 100 lines.

**Rules**:
- Scope limited: only content about that directory
- No duplication: do not repeat root CLAUDE.md content — state differences only
- Reference parent: use relative path `../CLAUDE.md`
- Self-contained heading: `# CLAUDE.md — {package/submodule name}`

**Structure**:

```
# CLAUDE.md — {name}

(1 line: purpose/role of this directory)

## Technical Stack
(Only differences from parent. Omit if same.)

## Development Commands
(Commands specific to this directory)

## Work Rules
(Only if different from parent. Omit if same.)

## References
- **[../CLAUDE.md](../CLAUDE.md)** — Project-wide rules
```

**Reference path rules**:
- Parent CLAUDE.md: always relative path (`../CLAUDE.md`)
- Submodule: reference parent repo CLAUDE.md via URL or relative path
- Sibling directories: do not reference directly (go through parent)

## Section E: .claude/rules/ Rule Files

**Generate only when** (1+ condition met):
1. Path-scoped rules exist (specific to certain directories)
2. CLAUDE.md would exceed 100 lines without splitting
3. 3+ independent concern groups identified

**Rules**:
- Path scoping first: always specify globs when applicable
- Minimize alwaysApply: prefer CLAUDE.md for universal rules
- One concern per file
- Filename: `{concern}.md` (e.g., `api-conventions.md`, `testing.md`)
- Size: ≤50 lines per file
- Discoverability test: inherited from common writing rules

**File format**:

```
---
description: (one-line description)
globs: ["src/api/**/*.ts"]    # optional: path scoping
alwaysApply: false            # true only when CLAUDE.md overflows
---

(Rule content — undiscoverable information only)
```

**Role distinction from contributing-docs/**:

| Aspect | contributing-docs/ | rules/ |
|--------|-------------------|--------|
| Audience | All AI agents + humans | Claude Code only |
| Loading | Referenced from AGENTS.md, read on demand | Auto-injected every session |
| Path scoping | Not possible | Via globs |
| Content | Detailed documents | Short behavioral rules |
```

- [ ] **Step 3: Verify file was created correctly**

Run: `wc -l agents/claude/skills/generate-claude-md/references/stage3-generator.md`
Expected: ~155-170 lines

- [ ] **Step 4: Commit**

```bash
git -C agents add claude/skills/generate-claude-md/references/stage3-generator.md
git -C agents commit -m "feat(skills): generate-claude-md Stage 3 Generator 참조 파일을 추가하다"
```

---

### Task 3: Create `references/stage4-verifier.md`

**Files:**
- Create: `agents/claude/skills/generate-claude-md/references/stage4-verifier.md`
- Reference: `agents/claude/skills/generate-claude-md/SKILL.md:402-454` (current Stage 4 verification)
- Reference: `agents/claude/skills/generate-claude-md/references/subagent-guidelines.md:135-170` (Reviewer prompt)
- Pattern reference: `agents/claude/skills/skill-improver/SKILL.md` (iterative fix loop)

- [ ] **Step 1: Read source material**

Read:
- Current SKILL.md lines 402-454 (Stage 4 checklist + Reviewer)
- `references/subagent-guidelines.md` lines 135-170 (Reviewer agent definition)
- `agents/claude/skills/skill-improver/SKILL.md` (Phase 4-5: failure analysis + re-verify pattern)

- [ ] **Step 2: Write stage4-verifier.md**

Create `agents/claude/skills/generate-claude-md/references/stage4-verifier.md` with the following content:

```markdown
# Stage 4: Verifier

> Checklist verification with iterative fix loop (max 3 iterations) + independent blind Reviewer.
> Adopts skill-improver's verify → fix → re-verify pattern.

---

## Phase 1: Checklist Verification (Verifier Agent)

**Agent**: `subagent_type: "general-purpose"`, `model: "sonnet"`, `run_in_background: false`
**Description**: "Verify generated CLAUDE.md files against checklist"

The Verifier agent receives all generated/modified files and applies the 10-item checklist line-by-line.

### Checklist

1. **Universality/necessity/redundancy**: Does this apply to all tasks? Would agents fail without it? Is it obvious from reading code? → If no to first two or yes to third: remove or move to AGENTS.md/contributing-docs/
2. **Linter role**: Is this a code style rule? → Remove, recommend linter/hook instead
3. **Speculation exclusion**: Was this confirmed in Stage 1-2? → If not confirmed: remove
4. **Verifiability**: Can compliance with this instruction be objectively verified? → If not: make specific
5. **Size constraints**: Root CLAUDE.md ≤100 lines (hard limit 300), individual instructions ≤50 items? → If exceeded: consolidate or remove
6. **Hierarchy/scope**: Does CLAUDE.md reference contributing-docs/ directly? Does AGENTS.md contain Claude-specific content? Does nested CLAUDE.md exceed its directory scope or repeat parent content? Do rules/ files duplicate CLAUDE.md? Are globs-eligible rules using alwaysApply: true? → Fix violations
7. **Reference integrity**: Are all cross-file references (CLAUDE.md → AGENTS.md, AGENTS.md → contributing-docs/, nested → parent, rules/ globs → actual paths) valid? → Fix broken references
8. **Discoverability**: Can an agent learn this by reading the code? → If yes: remove
9. **Staleness risk**: Does any line reference specific versions, tool names, or dependencies that may become inaccurate within 6 months? → If yes: remove or add expiry comment
10. **Static instruction**: Is this an unconditional rule applied identically to all task types? → If conditional is better: add conditions

### Anti-pattern Detection

Flag and report if found:
- Auto-generated content: LLM-summarized code included verbatim
- Information duplication: content already in README, CONTRIBUTING.md, or CI config
- Stale references: tech/dependencies/patterns that don't match current codebase

### Self-test Questions

After checklist, the Verifier asks itself:
- "Would a senior engineer call this CLAUDE.md excessive?"
- "Is the CLAUDE.md → AGENTS.md → contributing-docs/ hierarchy and the CLAUDE.md → .claude/rules/ path clearly separated?"
- "Would removing a nested CLAUDE.md leave the parent CLAUDE.md sufficient for that directory?" → If yes: recommend deletion
- "Do any nested CLAUDE.md files contradict each other?"
- "Could deleting this line still let an agent reach the same conclusion from code?" → If yes: delete
- "Can this item be solved by code/linter/CI?" → If yes: recommend code fix, remove item

### Output Format

The Verifier returns a structured report:

```
## Verification Report — Iteration {N}

### PASS (N items)
(Count only — no details needed)

### FAIL (N items)
| # | File | Line | Criterion | Issue | Suggested Fix |
|---|------|------|-----------|-------|---------------|

### WARNINGS (N items)
| # | File | Line | Concern |
```

---

## Phase 2: Iterative Fix Loop (max 3 iterations)

Adopted from skill-improver's iterative test pattern.

```
Iteration 1: Verifier runs checklist
  ├─ All PASS → proceed to Phase 3
  └─ FAIL items found → orchestrator applies fixes → Iteration 2
Iteration 2: Verifier re-runs checklist on fixed files
  ├─ All PASS → proceed to Phase 3
  └─ FAIL items found → orchestrator applies fixes → Iteration 3
Iteration 3: Final verification
  ├─ All PASS → proceed to Phase 3
  └─ FAIL items remain → report to user with remaining issues
```

**Fix rules**:
- Orchestrator applies fixes (not the Verifier agent)
- Each fix must address a specific FAIL item
- Do not introduce new content during fixes — only remove or modify flagged lines
- Track which items were fixed across iterations to avoid loops

---

## Phase 3: Blind Reviewer (Independent Verification)

**Trigger**: Generated output includes AGENTS.md, contributing-docs/, or nested CLAUDE.md (i.e., more than a single root CLAUDE.md).

**Skip conditions**:
- Only a single root CLAUDE.md was generated
- User explicitly requested fast generation

**Agent**: `subagent_type: "general-purpose"`, `model: "sonnet"`, `run_in_background: false`
**Description**: "Blind review generated files"

### What to Provide

Generated file contents ONLY. List each file path and its full content.

### What NOT to Provide

- Stage 1 detection results
- Stage 2 interview answers
- Orchestrator's internal reasoning
- Verifier's checklist results

This ensures blind, independent review.

### Reviewer Prompt

> You are reviewing generated CLAUDE.md and related files. You did NOT write these.
> Review independently using these criteria:
>
> Files to review: {list_of_generated_file_paths}
>
> 1. Discoverability: Does each line pass "Can an agent learn this by reading the code?" If yes → flag for removal
> 2. Staleness risk: Does any line reference specific versions, tool names, or dependencies that may become inaccurate within 6 months? → flag with reason
> 3. Redundancy: Is any content duplicated between CLAUDE.md, AGENTS.md, and contributing-docs/? → flag the duplicate
> 4. Hierarchy: Does CLAUDE.md reference contributing-docs/ directly (should go through AGENTS.md)? → flag
> 5. Nested scope: Does any nested CLAUDE.md repeat content from root or exceed its directory scope? → flag
> 6. Size: Is root CLAUDE.md under 100 lines (hard limit 300)? Nested under 50 (hard limit 100)?
> 7. Actionability: Is every instruction verifiable? Any vague guidance? → flag with suggestion
>
> Report: PASS/FAIL per criterion. For each FAIL, quote the specific line and explain why.
> Do NOT fix issues — only report them.

### After Reviewer

- Orchestrator receives Reviewer report
- FAIL items → orchestrator applies fixes (same rules as Phase 2)
- If fixes are applied, no additional Reviewer pass needed (Verifier already iterated)

---

## Update Mode Additional Checks

When running in update mode, add these checks before the standard checklist:

- Lines that were NOT modified: verify they were not accidentally changed
- Reference paths: still valid after modifications?
- Cross-file consistency: maintained? (re-run U2 axis 3 check from update-mode.md)
```

- [ ] **Step 3: Verify file was created correctly**

Run: `wc -l agents/claude/skills/generate-claude-md/references/stage4-verifier.md`
Expected: ~145-160 lines

- [ ] **Step 4: Commit**

```bash
git -C agents add claude/skills/generate-claude-md/references/stage4-verifier.md
git -C agents commit -m "feat(skills): generate-claude-md Stage 4 Verifier 참조 파일을 추가하다"
```

---

### Task 4: Create `references/update-mode.md`

**Files:**
- Create: `agents/claude/skills/generate-claude-md/references/update-mode.md`
- Reference: `agents/claude/skills/generate-claude-md/SKILL.md:294-398` (current U1-U3)

- [ ] **Step 1: Read source material**

Read current SKILL.md lines 294-398 — the full U1 (audit), U2 (drift comparison), U3 (apply) procedures.

- [ ] **Step 2: Write update-mode.md**

Create `agents/claude/skills/generate-claude-md/references/update-mode.md` with the following content:

```markdown
# Update Mode: U1–U3 Procedures

> Audit, drift comparison, and surgical application for updating existing CLAUDE.md and related files.
> Integrated into the main stage flow: U1 after Stage 1, U2 during Stage 2, U3 instead of Stage 3.

---

## U1: Audit (after Stage 1)

Assess the current state of existing generated files.

### Procedure

1. Detect existing files via Glob/Read:
   - Root CLAUDE.md: line count, section list (`#`/`##` header parsing), reference path validity
   - AGENTS.md: frontmatter fields, section list, contributing-docs/ reference validity, Boundaries presence
   - contributing-docs/: file list, line count per file, whether referenced from AGENTS.md
   - Nested CLAUDE.md: locations, line count, parent reference path validity, content overlap with parent
   - .claude/rules/: file list, each file's description/globs/alwaysApply, content overlap with CLAUDE.md

2. Present audit summary to user as a table:

| File | Lines | Status | Notes |
|------|-------|--------|-------|
| CLAUDE.md | N | exists | N sections |
| ... | ... | ... | ... |

3. If no files found: propose "No existing files found. Switch to generation mode?" via AskUserQuestion and halt.

### Parallelization

U1 file reading can run in parallel with Stage 1 Explore agents. However, if target files are ≤3, read directly (more efficient than spawning agents).

---

## U2: Drift Comparison (during Stage 2)

Compare Stage 1 re-analysis results with U1 audit results across 3 axes.

### Axis 1: Codebase Drift

Compare current code state (Stage 1) with existing document content:

| File Type | Compare | Drift Criteria |
|-----------|---------|----------------|
| Root CLAUDE.md | Tech stack vs actual config files | Tech added/removed/version changed |
| Root CLAUDE.md | Dev commands vs actual scripts/Makefile | Commands changed/added/removed |
| AGENTS.md | Operational Gotchas vs current code | Resolved gotcha remains, new gotcha needed |
| contributing-docs/ | Each doc content vs actual structure/config | Structure/strategy changed |
| Nested CLAUDE.md | Directory tech/commands vs document | Subdirectory changed |
| .claude/rules/ | globs patterns vs actual file paths | Scoped paths no longer exist |

### Axis 2: Principle Re-application

Re-apply Stage 3 generation philosophy to all existing lines:

- **Discoverability test**: items now discoverable due to code improvements → remove candidates
- **Size constraints**: root CLAUDE.md >100 lines, nested >50 lines → trim candidates
- **Staleness risk**: specific version/tool/dependency that no longer matches current state
- **Redundancy**: content duplicated between files

### Axis 3: Structural Integrity

Validate cross-file reference relationships:

- CLAUDE.md → AGENTS.md reference path valid
- AGENTS.md → contributing-docs/ references match actual files
- Nested CLAUDE.md → parent reference path correct
- .claude/rules/ globs point to existing paths

### Comparison Report

Present categorized results to user:

```
## Drift Comparison Results

### Change Required
| # | File | Item | Reason | Recommended Action |

### Remove Recommended
| # | File | Item | Reason |

### Add Candidate
| # | File | Item | Reason |

### No Change
(Summary count of unchanged items only)
```

Then AskUserQuestion: "Which items would you like to apply?" (all / selective / per-file)

---

## U3: Apply (instead of Stage 3)

Apply user-approved changes using Edit tool for surgical modifications.

### Apply Principles

- **Surgical changes**: modify only lines that need changing. Never regenerate entire files.
- **Apply order** (leaf-first, upstream-last):
  1. contributing-docs/ individual files
  2. .claude/rules/ individual files
  3. Nested CLAUDE.md
  4. AGENTS.md (including contributing-docs/ reference updates)
  5. Root CLAUDE.md (including AGENTS.md reference updates)
- **Before each file edit**: show the change to user and get confirmation
- **File deletion**: propose only, execute after user approval
- **File addition**: follow Stage 3 generation rules (from stage3-generator.md)

After all changes applied, proceed to Stage 4 (verification).
```

- [ ] **Step 3: Verify file was created correctly**

Run: `wc -l agents/claude/skills/generate-claude-md/references/update-mode.md`
Expected: ~100-115 lines

- [ ] **Step 4: Commit**

```bash
git -C agents add claude/skills/generate-claude-md/references/update-mode.md
git -C agents commit -m "feat(skills): generate-claude-md Update Mode 참조 파일을 추가하다"
```

---

### Task 5: Rewrite SKILL.md as Orchestrator

**Files:**
- Rewrite: `agents/claude/skills/generate-claude-md/SKILL.md`

This is the core task. The current ~467-line SKILL.md is replaced with a ~120-line orchestrator.

- [ ] **Step 1: Read current SKILL.md in full**

Read the entire current SKILL.md to ensure nothing is missed during rewrite. Confirm all stage content has been extracted to reference files in Tasks 1-4.

Cross-check:
- Lines 56-86 (Stage 1) → `stage1-analyzer.md` ✓
- Lines 123-281 (Stage 3 A-E) → `stage3-generator.md` ✓
- Lines 294-398 (U1-U3) → `update-mode.md` ✓
- Lines 402-454 (Stage 4) → `stage4-verifier.md` ✓
- Lines 87-120 (Stage 2) → stays in SKILL.md (user interaction)
- Lines 458-466 (LLM context principles) → stays in SKILL.md (philosophy summary)

- [ ] **Step 2: Write new SKILL.md**

Replace entire SKILL.md with:

```markdown
---
name: generate-claude-md
description: "CLAUDE.md, AGENTS.md, contributing-docs/, .claude/rules/ 파일을 가이드 원칙에 따라 생성하거나 업데이트한다. /generate-claude-md, CLAUDE.md 업데이트, AGENTS.md 갱신 요청 시 사용한다."
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Agent, advisor
---

# CLAUDE.md Generator — Orchestrator

Stage-based orchestrator for generating and updating CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, and .claude/rules/ files.

## Mode Detection

Analyze $ARGUMENTS to determine mode:

- **Update mode**: $ARGUMENTS contains "업데이트", "수정", "갱신", "update", "refresh"
  → Stage 1 → U1 (audit) → Stage 2 + U2 (drift) → U3 (apply) → Stage 4
- **Generation mode**: otherwise
  → Stage 1 → Stage 2 → Stage 3 → Stage 4

### Target File Identification

| Keyword in $ARGUMENTS | Target |
|----------------------|--------|
| "CLAUDE.md" (alone) | Root CLAUDE.md only |
| "AGENTS.md" | AGENTS.md + contributing-docs/ |
| "rules" | .claude/rules/ only |
| No keyword + "update" | All 5 types |

If $ARGUMENTS is empty, run generation mode on current working directory.

---

## Generation Philosophy

These principles govern all stages. Detailed content is in references/:

- **Design**: think before coding, simplicity first, surgical changes, goal-driven execution (references/karpathy-guidelines.md)
- **Content**: include only undiscoverable information; treat AGENTS.md as a codesmell list (references/osmani-guidelines.md)
- **Performance**: auto-generated context → success rate -2~3%, cost +20%; hand-written gotchas → +4% (ETH Zurich). Every line must justify its existence.
- **Governance**: for projects needing autonomous agent safeguards, reflect Entry Router CORE rules in Boundaries (references/entry-router-guidelines.md)
- **LLM context**: LLMs are in-context learners — they follow existing code patterns naturally. Non-universal instructions increase ignore probability. Upstream errors (CLAUDE.md) amplify exponentially downstream (plan → code).

---

## Stage 1: Project Analysis

Read `references/stage1-analyzer.md` and execute.

1. Assess project complexity via initial glob
2. Complex: spawn 3 Explore agents (config/structure/docs) in parallel, model: sonnet
3. Simple: explore directly without agents
4. Merge results, classify discoverable vs undiscoverable
5. Present findings to user

**advisor() call ①**: when agent results contradict each other, or discoverable/undiscoverable classification is ambiguous.

**Update mode**: run U1 (audit) from `references/update-mode.md` in parallel with or after Stage 1.

---

## Stage 2: Interview (Direct Execution)

This stage runs in the orchestrator — NOT delegated to a sub-agent — because it requires user interaction.

Ask only about undiscoverable items from Stage 1. Follow the WHY/WHAT/HOW framework:

**WHY** (always undiscoverable):
- Project purpose/role

**WHAT** (only gaps from Stage 1):
- Monorepo package roles
- Submodule relationship with parent repo
- External service dependencies

**HOW** (only gaps from Stage 1):
- Special workflow/branch/PR/commit rules
- Recurring agent mistakes → solvable by code, or needs explicit instruction?
- Nested CLAUDE.md candidates: show list, confirm creation for each

Additional rules:
- Present possible interpretations for ambiguous items, let user choose
- Confirm Stage 1 assumptions with user
- Optional: spawn Explore-Deep during AskUserQuestion wait for large monorepos (see stage1-analyzer.md)

**advisor() call ②**: when file scope/structure decisions are ambiguous (e.g., nested CLAUDE.md vs rules/ separation).

**Update mode**: integrate U2 (drift comparison) from `references/update-mode.md` into interview, present comparison report, confirm update scope with user.

---

## Stage 3: Generation

Read `references/stage3-generator.md` and execute.

1. Construct prompt with: Stage 1 analysis (summary), Stage 2 answers, target file list, generation principles
2. Spawn 1 general-purpose agent (model: sonnet) to generate all files
3. Agent writes files directly using Write tool

**Update mode**: execute U3 (apply) from `references/update-mode.md` instead — surgical Edit, leaf-first order, user confirmation per file.

---

## Stage 4: Verification

Read `references/stage4-verifier.md` and execute.

1. **Phase 1**: Verifier agent (sonnet) applies 10-item checklist
2. **Phase 2**: iterative fix loop — FAIL items → fix → re-verify (max 3 iterations)
3. **Phase 3**: Blind Reviewer agent (sonnet) — independent evaluation, receives generated files only

**advisor() call ③**: when verification fails 2+ times, or Verifier and Reviewer disagree on fix direction.

Present final results to user. If FAIL items remain after 3 iterations, report them explicitly with reasons.

---

## Advisor Escalation Summary

| # | When | Trigger Condition |
|---|------|-------------------|
| ① | Stage 1 merge | Agent results contradict; discoverable/undiscoverable classification ambiguous |
| ② | Pre-Stage 3 | File scope/structure decision unclear (nested CLAUDE.md vs rules/ vs AGENTS.md) |
| ③ | Stage 4 failure | Verification fails 2+ iterations; Verifier/Reviewer disagree |

**Do NOT call advisor for**: simple questions, routine checklist items, file generation mechanics.
```

- [ ] **Step 3: Verify line count**

Run: `wc -l agents/claude/skills/generate-claude-md/SKILL.md`
Expected: ~115-125 lines

- [ ] **Step 4: Commit**

```bash
git -C agents add claude/skills/generate-claude-md/SKILL.md
git -C agents commit -m "refactor(skills): generate-claude-md를 스테이지 기반 오케스트레이터로 리팩터링하다"
```

---

### Task 6: Delete `references/subagent-guidelines.md`

**Files:**
- Delete: `agents/claude/skills/generate-claude-md/references/subagent-guidelines.md`

- [ ] **Step 1: Verify content has been migrated**

Confirm all content from `subagent-guidelines.md` exists in the new reference files:
- Explore-Config prompt → `stage1-analyzer.md` config-explorer ✓
- Explore-Structure prompt → `stage1-analyzer.md` structure-explorer ✓
- Explore-Docs prompt → `stage1-analyzer.md` docs-explorer ✓
- Explore-Deep prompt → `stage1-analyzer.md` Explore-Deep section ✓
- Reviewer prompt → `stage4-verifier.md` Phase 3 ✓
- Decision criteria → absorbed into each stage's skip conditions ✓
- Parallelism rules → `stage1-analyzer.md` (Stage 1), `stage4-verifier.md` (Stage 4) ✓
- Anti-patterns → absorbed into stage files ✓

- [ ] **Step 2: Delete the file**

```bash
git -C agents rm claude/skills/generate-claude-md/references/subagent-guidelines.md
```

- [ ] **Step 3: Verify no dangling references**

Search for any remaining references to `subagent-guidelines.md`:

```bash
grep -r "subagent-guidelines" agents/claude/skills/generate-claude-md/
```

Expected: no matches

- [ ] **Step 4: Commit**

```bash
git -C agents commit -m "chore(skills): 마이그레이션 완료된 subagent-guidelines.md를 삭제하다"
```

---

### Task 7: Final Verification

**Files:**
- Verify: all files in `agents/claude/skills/generate-claude-md/`

- [ ] **Step 1: Verify file structure**

```bash
find agents/claude/skills/generate-claude-md/ -type f -name "*.md" | sort
```

Expected output:
```
agents/claude/skills/generate-claude-md/SKILL.md
agents/claude/skills/generate-claude-md/references/SOUL.md
agents/claude/skills/generate-claude-md/references/entry-router-guidelines.md
agents/claude/skills/generate-claude-md/references/karpathy-guidelines.md
agents/claude/skills/generate-claude-md/references/osmani-guidelines.md
agents/claude/skills/generate-claude-md/references/stage1-analyzer.md
agents/claude/skills/generate-claude-md/references/stage3-generator.md
agents/claude/skills/generate-claude-md/references/stage4-verifier.md
agents/claude/skills/generate-claude-md/references/update-mode.md
```

9 files total. `subagent-guidelines.md` must NOT appear.

- [ ] **Step 2: Verify SKILL.md line count**

```bash
wc -l agents/claude/skills/generate-claude-md/SKILL.md
```

Expected: ≤125 lines

- [ ] **Step 3: Verify model and allowed-tools in frontmatter**

```bash
head -6 agents/claude/skills/generate-claude-md/SKILL.md
```

Expected:
```yaml
---
name: generate-claude-md
description: "..."
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Agent, advisor
---
```

- [ ] **Step 4: Verify sub-agent model specifications**

Search for `model:` in all new reference files to confirm sonnet is explicitly specified:

```bash
grep -n "model" agents/claude/skills/generate-claude-md/references/stage{1,3,4}*.md
```

Expected: all sub-agent definitions specify `"sonnet"`

- [ ] **Step 5: Verify advisor escalation points**

```bash
grep -c "advisor" agents/claude/skills/generate-claude-md/SKILL.md
```

Expected: mentions at exactly 3 escalation points (①②③) plus the summary table and the "Do NOT call" note

- [ ] **Step 6: Verify no content loss**

Cross-check that the following critical content exists across all files:
- [ ] Discoverability test mentioned in stage3-generator.md and stage4-verifier.md
- [ ] 10-item checklist in stage4-verifier.md
- [ ] All 5 file types (A-E) in stage3-generator.md
- [ ] U1/U2/U3 complete procedures in update-mode.md
- [ ] Mode detection logic in SKILL.md
- [ ] Stage 2 interview framework in SKILL.md
- [ ] 4 guideline references preserved unchanged

- [ ] **Step 7: Commit submodule pointer update in parent repo**

```bash
cd /Users/ujuc/.config/dotrc
git add agents
git commit -m "chore(agents): 서브모듈을 업데이트하다"
```
