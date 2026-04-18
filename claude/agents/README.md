# agents/

Reference for the five subagents that power the planning-pipeline skills
(`deep-read` → `annotate-plan` → `implement-plan`).

Agents here are pipeline workers, not general-purpose assistants. They are
invoked via the `Agent` tool with `subagent_type: "<name>"` by exactly one
calling skill each.

For policy ("how to edit agents here"), see `../CLAUDE.md`.
This file is the **reference** for callers and contributors.

## Role Matrix

| Agent              | Calling skill       | Model  | Tools                                     | Writes code | Output path                       | Advisor |
|--------------------|---------------------|--------|-------------------------------------------|-------------|-----------------------------------|---------|
| `reference-finder` | `annotate-plan`     | sonnet | Read, Glob, Grep, advisor                 | no          | `.plans/.partial/references.md`   | ≤1      |
| `researcher`       | `deep-read` (×3)    | sonnet | Read, Glob, Grep, Bash, advisor           | no          | `.research/.partial/{role}.md`    | ≤1      |
| `verifier`         | `implement-plan`    | haiku  | Read, Glob, Grep, Bash, advisor           | no          | `.plans/.verify-{item-slug}.md`   | emergency only |
| `implementer`      | `implement-plan`    | sonnet | Read, Write, Edit, Glob, Grep, Bash, advisor | **yes**  | source files + `.plans/.blocker-{item-slug}.md` on failure | ≤1 (pre-blocker) |
| `debugger`         | `implement-plan`    | sonnet | Read, Grep, Glob, Bash, advisor           | no          | `.plans/.debug-{item-slug}.md`    | ≤1      |

Model selection: `haiku` for mechanical / high-volume parallel work, `sonnet`
for anything that requires reasoning or synthesis.

Tool minimalism: each agent gets the smallest tool set that lets it do its
job. `implementer` is the only one with `Write` / `Edit` for a reason.

## I/O Contract

Every agent is invoked with an output file path in its prompt. Rules that
apply to every agent:

1. **Write the output file even on partial success.** A missing file is
   indistinguishable from a silent crash to the caller.
2. **Cite sources with `file:line` or `file:start-end`.** Plain prose
   claims without citations are rejected by downstream skills.
3. **Markdown headings are part of the contract.** Callers grep for specific
   headings to split and merge partials — do not rename or drop headings.
4. **On partial or degraded output, prepend `<!-- PARTIAL: {reason} -->`.**
   `deep-read` and `annotate-plan` preserve this marker through their merge
   logic so the user can decide whether to retry.
5. **Never modify files outside the output path**, except `implementer`,
   which modifies source files scoped to its assigned todo item.

## Dependency Graph

```
          deep-read
              │
              ▼
 ┌──────── researcher ×3 (structure/dataflow/risks)
 │            │
 │            ▼
 │   .research/research-{feature}.md
 │            │
 │            ▼
 │       annotate-plan ─────────── reference-finder
 │            │                          │
 │            ▼                          ▼
 │   .plans/plan-{feature}.md   .plans/.references/{feature}.md
 │            │
 │            ▼
 │       implement-plan
 │            │
 │   ┌────────┼─────────┬─────────┐
 │   ▼        ▼         ▼         ▼
 │ implementer verifier debugger  (back to annotate-plan Phase B on RESET)
 │   │        │         │
 │   ▼        ▼         ▼
 │  source   .verify-   .debug-
 │  edits    {slug}.md  {slug}.md
 │   │
 │   ▼
 │  .plans/.blocker-{slug}.md (on failure, read by implement-plan Step 5a)
```

Artifact paths read by multiple skills:

- `.research/research-*.md` — produced by `deep-read`, consumed by
  `annotate-plan` Phase A.
- `.plans/.references/{feature}.md` — produced by `reference-finder` during
  `annotate-plan` Phase A, consumed by `implementer`.
- `.plans/plan-{feature}.md` — produced by `annotate-plan`, consumed by
  `implement-plan`.
- `.plans/.verify-{slug}.md` — produced by `verifier`, polled by
  `implement-plan` Step 3 Mode A.
- `.plans/.blocker-{slug}.md` — produced by `implementer`, consumed by
  `implement-plan` Step 5a and `annotate-plan` Phase B.
- `.plans/.debug-{slug}.md` — produced by `debugger`, consumed by
  `implement-plan` Step 5a and `annotate-plan` Phase B.

## Advisor Common Guide

All five agents have `advisor` in their `tools:` frontmatter, but the call
budget is deliberately tight:

- `advisor()` takes **no parameters** — the agent's full execution context
  is forwarded automatically.
- Call advisor AT MOST ONCE per run, at the standard point: **after
  orientation, before substantive work** (before writing the output file,
  before deep reading, before implementing).
- Do NOT call advisor for deterministic / mechanical work. It is not a
  sanity check; it is a judgment aid.
- The `verifier` is haiku-model and treats advisor as **emergency-only** —
  see its SKILL-side policy for the narrow exception.
- When advisor conflicts with tool output (files, test results), trust the
  tool output. You are allowed ONE reconcile call to surface the conflict
  explicitly; beyond that, record a blocker (implementer) or proceed with
  the primary evidence.

## Adding a New Agent — Checklist

1. `name` in frontmatter matches the filename (kebab-case).
2. `description` is one sentence, ending with "Used by {skill} skill."
3. `tools:` contains the minimum set. Do not copy `implementer`'s tool list
   by default.
4. `model:` — `haiku` only if the work is mechanical and cost-sensitive;
   otherwise `sonnet`.
5. Decide advisor policy: `≤1`, `emergency only`, or `no advisor`.
6. Document Input, Output, and any Failure Policy explicitly in the body.
7. Add a row to the Role Matrix above and, if the agent produces a new
   artifact path, to the Dependency Graph.
8. Wire the dispatch into exactly one calling skill. Do not share agents
   across skills — that is what makes these pipeline workers instead of
   general-purpose assistants.
