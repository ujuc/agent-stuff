# Worked Example — One Negotiation Round-Trip

This is a single, end-to-end example of one round-trip for a small sprint. It exists to anchor the format rules from `file-format.md` in something concrete.

## Input Spec

> "Add a tile-based map editor to the level designer. It needs a fill tool and a way to delete entities."

## `.sprint/contract-draft-1.md` (Generator)

```markdown
# Contract Draft 1 — Tile Editor v1

## Sprint Goal
Ship the level designer's first usable tile editor with fill and delete primitives.

## Implementation Scope
1. Rectangle fill tool — paint a rectangular area with the selected tile
2. Entity delete — remove a selected entity from the canvas
3. Tile palette — choose the active tile

## Verification Criteria

| # | Criterion | Expected Behavior | Test Method |
|---|-----------|-------------------|-------------|
| 1 | Fill tool works correctly | Tiles get painted | Try it in the editor |
| 2 | Delete removes entities | Entities disappear | Click delete |
| 3 | Tile palette shows all available tiles | All tiles visible | Open the palette |
| 4 | Performance is acceptable | Fast enough | Try a big map |

## Exclusions
- Multi-layer editing
- Undo/redo history

## Changes from Previous Draft
Initial draft.
```

## `.sprint/contract-review-1.md` (Evaluator)

```markdown
# Contract Review 1 — Tile Editor v1

Reviewed: contract-draft-1.md

## Verdicts

| # | Verdict | Reason |
|---|---------|--------|
| 1 | REJECT | Missing observable result and verification method: "works correctly" and "Try it" are not externally testable. Specify the input gesture and the expected canvas state. |
| 2 | REJECT | Missing subject specificity and observable result: which entity, deleted from where, what UI confirms removal? |
| 3 | ACCEPT | (subject + verb + result + method all present) |
| 4 | REJECT | Missing observable result and verification method: "fast enough" has no threshold. Pick a latency in ms and a map size. |

## Summary
- Accepted: 1
- Rejected: 3
- Escalation triggered: no
```

## `.sprint/contract-draft-2.md` (Generator, addressing review)

```markdown
# Contract Draft 2 — Tile Editor v1

## Sprint Goal
Ship the level designer's first usable tile editor with fill and delete primitives.

## Implementation Scope
(unchanged)

## Verification Criteria

| # | Criterion | Expected Behavior | Test Method |
|---|-----------|-------------------|-------------|
| 1 | Rectangle fill tool allows click-drag to paint a rectangular region with the selected tile | Every cell inside the dragged rectangle shows the selected tile after mouseUp | Manual: pick tile, click-drag from (1,1) to (5,5), confirm 25 cells changed |
| 2 | Clicking the trash icon on a selected entity removes it from canvas and entity-list panel | Selected entity vanishes from both views; selection clears | Manual: select entity E, click trash icon, confirm E is gone from canvas and list |
| 3 | Tile palette shows all available tiles | All tile types in the assets folder appear in the palette grid | Manual: count palette icons, compare with `assets/tiles/*.png` count |
| 4 | Map loads within 500 ms on a 100×100 grid | Time from "open map" action to first paint ≤ 500 ms | Manual + perf log: open `fixtures/100x100.map`, read timestamp delta |

## Exclusions
(unchanged)

## Changes from Previous Draft
- #1: rewrote per review-1#1 — added gesture, observable result, and exact verification steps
- #2: rewrote per review-1#2 — named the trigger (trash icon), the two views, and the selection-clear side effect
- #4: rewrote per review-1#4 — added 500 ms threshold and 100×100 fixture
```

## `.sprint/contract-review-2.md` (Evaluator)

```markdown
# Contract Review 2 — Tile Editor v1

Reviewed: contract-draft-2.md

## Verdicts

| # | Verdict | Reason |
|---|---------|--------|
| 1 | ACCEPT |  |
| 2 | ACCEPT |  |
| 3 | ACCEPT |  |
| 4 | ACCEPT |  |

## Summary
- Accepted: 4
- Rejected: 0
- Escalation triggered: no
```

## `.sprint/contract.md` (final)

```markdown
# Sprint Contract — Tile Editor v1

## Sprint Goal
Ship the level designer's first usable tile editor with fill and delete primitives.

## Implementation Scope
1. Rectangle fill tool — paint a rectangular area with the selected tile
2. Entity delete — remove a selected entity from the canvas
3. Tile palette — choose the active tile

## Verification Criteria
(copied from draft-2 verbatim)

## Exclusions
- Multi-layer editing
- Undo/redo history

## Negotiation History
- Draft 1 (2026-04-18): 4 proposed, 1 accepted, 3 rejected
- Draft 2 (2026-04-18): 3 revised, 4 accepted, 0 rejected
- Final (2026-04-18): 4 criteria agreed in 2 rounds
```

## What This Example Illustrates

1. **The Evaluator's reasons name a missing part of the four-part rule.** "Missing verification method" is actionable; "too vague" alone would not be.
2. **The Generator's `Changes from Previous Draft` cites review IDs.** This is what makes the audit trail useful.
3. **Every criterion in draft 2 has all four parts.** Subject + verb + observable result + verification method.
4. **No round-trip waste.** Round 2 fixed every rejection; the third round was unnecessary.
