---
name: implement-plan
description: "Execute annotated plans with continuous verification. Supports sequential and parallel (worktree) execution modes. Triggers: 구현 시작, 플랜 실행해, implement-plan, 다 구현해, /implement-plan"
model: sonnet
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, Agent
---

# Implement Plan — Execution Driver

Execute the implementation plan from `.plans/plan-{feature}.md` with continuous verification and progress tracking.

## Workflow

### 1. Load Plan
- Find active plan in `.plans/plan-*.md` (if multiple, ask user which one)
- Parse todo items and their dependencies
- Load `.plans/.references/{feature}.md` if exists (reference implementations)
- Create `.plans/.implementing` flag file (activates polyglot-typecheck hook)

### 2. Analyze Dependencies
Classify each todo item:
- **Independent**: no dependency on other items (can run in parallel)
- **Sequential**: depends on completion of another item

### 3. Choose Execution Mode

**Mode A — Sequential** (default, when items have dependencies):
- Implement items in dependency order in the main context
- After each item completion:
  1. Mark `- [x]` in the plan file
  2. Launch `verifier` agent in background:
     ```
     Agent(name="verifier", run_in_background=true)
     Task: verify {item}. Check types, lint, tests.
     Write results to .plans/.verify-{item-slug}.md
     ```
  3. Proceed to next item immediately (do not wait for verifier)
  4. If verifier reports errors, fix before continuing

**Mode B — Parallel** (when 2+ independent items exist):
- Launch `implementer` agents in isolated worktrees:
  ```
  Agent(name="implementer-{N}", isolation="worktree", run_in_background=true)
  Task: implement {todo items}
  Plan context: {relevant plan sections}
  References: {from .plans/.references/}
  ```
- Each agent follows `~/.claude/agents/implementer.md` standards
- After all agents complete, review worktree changes
- If merge conflicts, notify user
- Mark completed items `- [x]` in the plan

### 4. Reference-Based Implementation
- Before writing new code, always check `.plans/.references/` for existing patterns
- Prefer adapting reference implementations over writing from scratch
- When the plan says "like X" or references existing code, read that code first and follow the same patterns

### 5. Scope Correction
If an approach is failing (verifier reports repeated errors, or implementation diverges from plan):
1. Stop implementation of the current item
2. Revert changes for that item (`git checkout -- {files}`)
3. Mark the todo item as `- [ ] (RESET)` in the plan
4. Notify user: "Item {X} needs scope correction. The approach in the plan may need revision."
5. Wait for user to annotate the plan with corrections before retrying

### 6. Completion
When all items are done or a blocking error occurs:
1. Run final `verifier` agent (full build + test suite)
2. Delete `.plans/.implementing` flag
3. Output summary:
   - Items completed / total
   - Verification results
   - Any items marked (RESET)
4. If uncommitted changes exist, suggest running `/commit`

## Implementation Rules
- Implementation should be **mechanical**, not creative — all decisions were made in planning
- Do NOT add comments, docstrings, or type annotations beyond what the plan specifies
- Do NOT refactor surrounding code
- Run typecheck continuously (polyglot-typecheck hook handles this automatically)
- Do NOT stop until all items are completed or explicitly blocked
