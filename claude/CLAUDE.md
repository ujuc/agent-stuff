@~/.config/dotrc/agents/rules/AGENTS.md

## Model Quality

- When `advisorModel` in `settings.json` is stronger than the active model, call `advisor()` before commit, push, publish, substantive analysis, or other consequential work.
- Skip `advisor()` for trivial tasks or when the user waives it.

## Delegation

- Delegate only when a child will read substantially more than it reports; verify delegated output before acting.
- Use `Explore` for multi-file discovery and a `haiku` subagent for mechanical command sweeps.
- Use the local `gemma` skill for text-only transforms. Set `GEMMA_NO_FALLBACK=1` for sensitive data.
- Keep decision-driving analysis and edits on the active model.
- Reserve `Workflow` for large evals, compliance checks, cross-verification, or bulk triage. Test a narrow slice and state the token budget first.

## Compaction

- Preserve modified files, latest verification results, pending approvals, and unanswered questions. The `PreCompact` hook already preserves `.research/` and `.plans/` pointers.
