---
name: implement-plan
description: "주석이 달린 구현 플랜을 지속적 검증·블로커 감지·디버거 연동과 함께 실행한다. 순차/병렬(worktree) 실행 모드를 지원한다. 구현 시작, 플랜 실행해, implement-plan, 다 구현해, /implement-plan 요청 시 사용한다."
group: build
model: sonnet
argument-hint: "[feature-name]"
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, Agent
---

# Implement Plan — Execution Driver

Execute the implementation plan from `.plans/plan-{feature}.md` with continuous verification and progress tracking.

Reads from: `.plans/plan-{feature}.md`, `.plans/.references/{feature}.md`, `.plans/.verify-{slug}.md`, `.plans/.blocker-{slug}.md`, `.plans/.debug-{slug}.md`.

## Composes with

This skill sits at the end of a three-step planning pipeline:

1. `deep-read` → writes `.research/research-*.md` (optional, codebase understanding)
2. `annotate-plan` → writes `.plans/plan-{feature}.md` and `.plans/.references/{feature}.md`
3. `implement-plan` (this skill) → executes the plan

When scope correction is needed, hand back to `annotate-plan` Phase B rather than redesigning inline.

## Workflow

### 1. Load Plan

- Parse `$ARGUMENTS` for an explicit feature name. If provided, open `.plans/plan-{feature}.md` directly.
- Otherwise glob `.plans/plan-*.md`:
  - Exactly one match → use it.
  - Multiple matches → ask the user which one via `AskUserQuestion`.
  - Zero matches → stop and suggest running `annotate-plan` first.
- Parse todo items (`- [ ]` lines) and any explicit dependency notes.
- Load `.plans/.references/{feature}.md` if present (reference implementations from `reference-finder`).
- Create `.plans/.implementing` flag file (activates the `polyglot-typecheck` hook).

### 2. Analyze Dependencies

Classify each todo item:

- **Independent**: no dependency on other items — eligible for parallel execution.
- **Sequential**: depends on completion of another item (explicit note or file-overlap).

File-overlap rule: if two items touch the same file per the plan, treat them as sequential even without an explicit note.

### 3. Choose Execution Mode

Use this heuristic:

| Condition | Mode |
|-----------|------|
| All items sequential OR fewer than 2 independent items | **A** (sequential) |
| 2+ independent items AND items touch disjoint file sets | **B** (parallel worktrees) |
| Mixed (some independent clusters, some sequential chains) | A first for chains, then B for the independent cluster |

**Mode A — Sequential** (default):

Implement items in dependency order in the main context. After each item:

1. Mark `- [x]` in the plan file.
2. Launch a background `verifier` agent:
   ```
   Agent(
     subagent_type="verifier",
     run_in_background=true,
     prompt="verify {item}. Write results to .plans/.verify-{item-slug}.md"
   )
   ```
3. Proceed to the next item immediately. Do not wait.
4. Before marking the NEXT item `- [x]`, poll `.plans/.verify-{prev-slug}.md` until all three lines (`typecheck:`, `lint:`, `tests:`) are present — retry at 1s intervals for up to 60s. Once complete, if any line is FAIL OR `.plans/.blocker-{prev-slug}.md` exists, branch to Step 5a (Blocker / Debug Detection) before continuing.

**Mode B — Parallel worktrees** (2+ independent items):

1. Launch one `implementer` agent per independent item:
   ```
   Agent(
     subagent_type="implementer",
     isolation="worktree",
     run_in_background=true,
     name="implementer-{N}",
     prompt="implement {item}. Plan: {section}. References: {patterns from .plans/.references/}."
   )
   ```
2. While agents run, continue Mode A for any sequential chain in the main context.
3. When each agent completes, it returns a worktree path. Merge reconciliation:
   ```bash
   git fetch <worktree-path>
   git merge --no-ff <branch-name>    # preserves parallel-work boundary
   ```
4. On conflict, surface to the user — do NOT auto-resolve (conflicts indicate dependency misclassification).
5. Mark completed items `- [x]` in the plan.

### 4. Reference-Based Implementation

- Always check `.plans/.references/` before writing new code.
- When the plan says "like X" or references existing code, read that code first and adapt the same patterns.
- When working in main context (Mode A), follow the same mechanical rules as `~/.claude/agents/implementer.md`.

### 5a. Blocker / Debug Detection

Entered from Step 3 Mode A when the previous verifier is complete AND either a blocker file exists OR any check is FAIL. Runs before Step 5 so that diagnostics inform (or replace) the scope-correction decision.

1. **Blocker path.** If `.plans/.blocker-{prev-slug}.md` exists:
   - Read the file and display its three sections (`## Problem`, `## Attempts`, `## Proposal`) verbatim to the user.
   - Direct the user to run `annotate-plan` Phase B (`address notes`) so the blocker feeds the next annotation cycle.
   - Do NOT continue to the next todo item. Do NOT run debugger (the implementer already determined this is not a diagnosable failure).

2. **Debugger dispatch on verifier FAIL.** Otherwise, if the previous verifier reports FAIL:
   ```
   Agent(
     subagent_type="debugger",
     prompt="diagnose {item}. Read .plans/.verify-{prev-slug}.md and the affected files. Write findings to .plans/.debug-{prev-slug}.md"
   )
   ```
   Wait for `.plans/.debug-{prev-slug}.md` to appear, then display its four sections (`## Symptom`, `## Hypotheses`, `## Reproduction`, `## Suggested Fix`) to the user. Ask whether to apply the suggested fix inline or proceed to Step 5 (Scope Correction).

3. **Clean path.** If neither file exists and every check is PASS/SKIP, continue to the next item normally — do NOT enter Step 5.

### 5. Scope Correction

If verifier reports repeated errors OR implementation diverges from the plan:

1. Stop the current item.
2. Revert changes for that item: `git checkout -- {files}` (or drop the worktree in Mode B).
3. Mark the todo as `- [ ] (RESET)` in the plan.
4. Notify user: "Item {X} needs scope correction. Run `annotate-plan` Phase B with notes."
5. Wait for user to annotate before retrying.

### 6. Completion

When all items are done or a blocking error occurs:

1. Launch one final `verifier` agent (full build + test suite) and wait for its report.
2. Delete `.plans/.implementing` flag.
3. Output summary:
   - Items completed / total.
   - Verification results (pass/fail per check).
   - Any items marked `(RESET)`.
4. If uncommitted changes exist, suggest running `/commit`.

## Gotchas

1. **`.plans/.implementing` flag is not auto-cleaned on crash.** If the previous run crashed mid-execution, the hook stays active. Detect a stale flag at Step 1 by checking file mtime — if older than 24h or no active plan file exists, delete and warn the user.

2. **Worktree cleanup requires explicit branch removal.** `isolation="worktree"` creates a branch per agent. After merge, run `git worktree remove <path>` and `git branch -d <branch>`; otherwise branches accumulate.

3. **Verifier race in Mode A.** Launching verifier `run_in_background=true` and immediately starting the next item means the verifier may still be running when the next completes. Always poll the previous verifier's output file BEFORE marking the next item `- [x]`, not after.

4. **File-overlap classification beats explicit dependency notes.** A plan may say "independent" but if two items both edit `src/auth.ts`, parallel execution will produce merge conflicts. Static file-overlap analysis from `.plans/.references/` overrides plan metadata.

5. **`verifier` uses the `haiku` model.** It is cheap and fast, but cannot diagnose subtle logic errors. Treat its PASS as "compiles and tests exit 0" — not "correct." The main-context Claude is responsible for correctness.

6. **Multiple active plans require disambiguation.** Never pick the first glob result silently — always ask via `AskUserQuestion` when `glob('.plans/plan-*.md')` returns more than one match, because selecting the wrong plan corrupts its `- [x]` state.

## Eval Criteria

```
EVAL 1: Plan state integrity
  Question: After execution, does the plan file have exactly one of
            [x], [ ], [ ] (RESET) per original todo line?
  Pass: All todos accounted for, no duplicates or orphans.
  Fail: Any todo line lost, duplicated, or in an unknown state.

EVAL 2: Flag lifecycle
  Question: After completion, is .plans/.implementing removed?
  Pass: File does not exist.
  Fail: File still present.

EVAL 3: Verification coverage
  Question: Did every completed [x] item have a corresponding
            .plans/.verify-*.md artifact?
  Pass: 1:1 mapping between [x] items and verify files.
  Fail: Any [x] item lacking a verification record.

EVAL 4: Mode selection correctness
  Question: If Mode B was used, did parallel items touch disjoint
            file sets (no merge conflicts during reconciliation)?
  Pass: Merge completed without conflicts.
  Fail: Conflicts occurred — items were misclassified as independent.

EVAL 5: Composition hygiene
  Question: On (RESET), was the handoff to annotate-plan Phase B
            made explicit to the user rather than inline redesign?
  Pass: User was directed to annotate-plan with reset notes.
  Fail: The skill attempted to redesign the failed item inline.
```
