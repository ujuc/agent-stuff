---
name: annotate-plan
description: "병렬 에이전트로 구현 계획을 생성하고, 사용자 인라인 주석을 반복 처리하여 플랜을 개선한다. 구현 계획 작성, 플랜 만들어줘, annotate-plan, /annotate-plan 요청 시 사용한다."
model: sonnet
allowed-tools: Read, Write, Glob, Grep, Bash, Agent, advisor
---

# Annotate Plan — Annotation Cycle Planning

Create an implementation plan at `.plans/plan-{feature}.md` and support iterative annotation cycles where the user adds inline notes.

## Phase A — Initial Plan Generation

### 1. Gather Context
- Load `.research/research-*.md` if exists (deep-read output)
- Check for `spec.md`, `.sprint/contract.md` (harness artifacts)
- Parse `$ARGUMENTS` for feature name and requirements

### 2. Launch 2 Parallel Agents

| Agent | Type | Role |
|-------|------|------|
| **plan-drafter** | `Plan` (subagent_type) | Draft implementation plan from research + context |
| **reference-finder** | `reference-finder` (subagent_type) | Find reusable patterns, utilities, and reference implementations in the codebase. Output to `.plans/.references/{feature}.md` |

The reference-finder agent follows `~/.claude/agents/reference-finder.md` standards.

### 3. Merge and Write Plan

Combine both agent outputs into `.plans/plan-{feature}.md`:

```markdown
# Plan: {feature}

## Goal
(what and why)

## Approach
(high-level strategy)

## Reference Implementations
(from reference-finder — existing code to reuse/adapt, with file:line citations)

## File Changes
(exact paths, what changes in each)

## Code Snippets
(key implementation details only)

## Dependencies & Ordering
(which items depend on others)

## Risk Assessment
(what could go wrong)

## Open Questions
(unresolved decisions)

## Todo
- [ ] Item 1
- [ ] Item 2
...
```

### 4. Save Baseline
- Copy plan to `.plans/.plan-{feature}.md.prev` (annotation detection baseline)
- Output: "`.plans/plan-{feature}.md` has been created. Review it and add inline notes, then say 'address notes' to start an annotation cycle."

## Phase B — Annotation Cycle

Triggered when user says: "노트 반영해줘", "address notes", "주석 처리해", "annotations"

### 1. Detect Annotations
- Diff `.prev` file against current plan to find user additions
- Also scan for patterns: `> ` blockquotes, `NOTE:`, `TODO:`, `FIXME:`, `<!-- ... -->` comments

### 2. Process Each Annotation
For each detected annotation:
1. Quote the annotation
2. Explain how it will be addressed
3. Update the plan accordingly

### 3. Update Baseline
- Overwrite `.prev` with current plan
- Track cycle count

### 4. Cycle Limit
- After 6 cycles, suggest: "6 annotation cycles complete. Consider moving to implementation with `/implement-plan`."
- This is a suggestion, not a hard stop

## Advisor Escalation

This skill runs on sonnet by default. At the decision points below, call `advisor()` to borrow higher-tier reasoning:

- **Phase A Step 3 — before writing Risk Assessment & Open Questions**: after merging outputs from plan-drafter and reference-finder, when it is unclear which risks are load-bearing or which items should be left as Open Questions.
- **Phase B Step 2 — annotation interpretation**: when a user's blockquote / NOTE / TODO is ambiguous, or when it is unclear whether the change should cut across multiple sections or remain a localized edit.

How to call: invoke `advisor()` with no parameters. The full current conversation context (plan file content and annotations) is automatically forwarded to the higher-tier model. Use this only when **the plan's direction itself needs a structural check** — not for simple Q&A.

## Constraints
- **Do NOT implement code** during this skill
- The plan is a **shared mutable document** — Claude writes, user annotates, Claude incorporates
- Always wait for user confirmation before proceeding to implementation
