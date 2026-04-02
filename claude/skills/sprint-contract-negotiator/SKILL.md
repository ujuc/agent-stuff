---
name: sprint-contract-negotiator
description: "Generator-Evaluator 간 done 기준을 파일 기반 프로토콜로 협상하여 sprint contract를 생성한다. sprint contract 협상, done 기준 정의, 완료 조건 합의, acceptance criteria, sprint-contract-negotiator 요청 시 사용한다."
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep
---

# Sprint Contract Negotiator

Negotiate a "definition of done" between Generator and Evaluator agents via file-based communication protocol. Inspired by the GAN-inspired multi-agent pattern from Anthropic's harness design blog, where the Evaluator's specificity is the key quality driver.

## Purpose

Before implementation begins, Generator and Evaluator must agree on what "done" looks like. This skill produces a `contract.md` file that both agents can reference during the sprint. The contract prevents scope drift and ensures every criterion is externally testable.

## Input

A high-level spec or user story. NOT detailed technical implementation — the contract defines WHAT to verify, not HOW to build it.

Example input:
- "Build a tile-based map editor with rectangle fill, entity placement, and animation preview"
- "Add user authentication with OAuth2 and role-based access control"

## Negotiation Protocol

### Roles

**Generator** proposes:
- Implementation plan (feature list with brief descriptions)
- Testable criteria for each feature (what can be verified from the outside)

**Evaluator** reviews each criterion and either:
- ACCEPTS — the criterion is specific, externally testable, and unambiguous
- REJECTS — with a concrete reason explaining why the criterion fails testability

### Iteration Rules

1. Generator writes `contract-draft-{n}.md` with proposed criteria
2. Evaluator reviews and writes `contract-review-{n}.md` with ACCEPT/REJECT per criterion
3. Generator incorporates feedback and writes next draft
4. **Maximum 3 round-trips** — if no agreement after 3 rounds, escalate to user
5. Final agreed version is written as `contract.md`

### File Exchange Location

All negotiation files are placed in the project root under `.sprint/` directory:
```
.sprint/
  contract-draft-1.md
  contract-review-1.md
  contract-draft-2.md
  contract-review-2.md
  contract.md          # Final agreed contract
```

## Output: contract.md Structure

The final contract follows this structure:

```markdown
# Sprint Contract — [Sprint Name]

## Sprint Goal
[One sentence describing what this sprint delivers]

## Implementation Scope
1. [Feature A] — [brief description]
2. [Feature B] — [brief description]
...

## Verification Criteria

| # | Criterion | Expected Behavior | Test Method |
|---|-----------|-------------------|-------------|
| 1 | [Subject + Verb + Result] | [Observable outcome] | [How to verify] |
| 2 | ... | ... | ... |

## Exclusions
- [Explicitly out-of-scope item 1]
- [Explicitly out-of-scope item 2]
```

## Criteria Quality Standards

### The Specificity Requirement

The blog's key insight: when the Evaluator found issues, findings were highly specific. This level of specificity is REQUIRED in both criteria and evaluator responses.

Bad criterion:
> "The fill tool works correctly"

Good criterion:
> "Rectangle fill tool allows click-drag to fill rectangular area with selected tile"

Bad evaluator feedback:
> "FAIL — fill tool has bugs"

Good evaluator feedback:
> "FAIL — Rectangle fill tool only places tiles at drag start/end. fillRectangle exists but not triggered on mouseUp"

### Criteria Writing Rule

Every criterion MUST follow the pattern: **Subject + Verb + Expected Result + Verification Method**

See [references/contract-template.md](references/contract-template.md) for the full template and real examples.

## Procedure

1. Read the input spec or user story
2. Use Glob and Grep to understand the existing codebase context (if any)
3. **Round 1 — Generator proposes**: Write `contract-draft-1.md` with sprint goal, feature list, and initial criteria table
4. **Round 1 — Evaluator reviews**: Read draft, check each criterion for external testability, write `contract-review-1.md`
5. Repeat until agreement or 3 rounds exhausted
6. Write final `contract.md`
7. Report summary to user: number of criteria, rounds needed, any escalated items

## When to Use This Skill

- At the START of a sprint, before any implementation
- When translating user stories into verifiable acceptance criteria
- When Generator and Evaluator agents need a shared definition of done
- When past sprints had scope disagreements or unclear completion criteria

## Gotchas

- **Do not include implementation details in criteria.** Criteria define WHAT to verify, not HOW to build. Technical implementation is the Generator's domain.
- **Max 3 round-trips is a hard limit.** If criteria cannot be agreed upon, the problem is likely ambiguous input — escalate to the user rather than looping endlessly.
- **Criteria count matters.** The blog's Sprint 3 had 27 criteria. Aim for thorough coverage — too few criteria means gaps will be discovered late.
- **Evaluator must reject vague criteria.** If a criterion cannot be tested by an external observer without reading source code, it is vague. "Works correctly" is always vague.
- **File-based protocol is non-negotiable.** All communication happens through files in `.sprint/`, not through conversational back-and-forth. This creates an audit trail.
- **Do not confuse this with a test plan.** The contract defines acceptance criteria at the product level. Unit test plans are a separate concern that the Generator handles during implementation.
