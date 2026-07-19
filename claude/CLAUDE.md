# CLAUDE.md

@~/.config/dotrc/agents/rules/AGENTS.md

Claude Code global configuration directory. Symlinked as `~/.claude` from the dotrc
repository. Always edit files here, not at the symlink target. Cross-agent guidance
comes from the import above — this file holds Claude-specific configuration only.

## Model Quality Safeguards

When the active model is below the Opus tier (`claude-opus-*`, `claude-fable-*`,
and `claude-mythos-*` are exempt — check the environment block), call `advisor()`
before: commit/push/publish, finalizing substantive analysis, or shipping work the
user will act on. Skip it for trivial reactive tasks or when the user waived it.

## Execution Delegation

Delegation protects the main context as much as it saves cost:

- Multi-file sweeps → `Explore`; run-and-report command sweeps → `haiku` subagent.
- Text-only transforms → local `gemma` skill (`--local`; `GEMMA_NO_FALLBACK=1` for
  sensitive data so it never leaves the machine). gemma cannot execute commands.
- Keep work on the active model when the output drives a decision or edit, and
  verify delegated output before acting. Trivial single commands run inline.

## Workflow Orchestration

Reserve the `Workflow` tool for genuine scale (skill evals, rule-compliance checks,
claim-source cross-verification, bulk triage). Before any large run, gauge cost on
a narrow slice and state the token budget explicitly.

## Context Compaction

When compacting, always preserve: the list of modified files, verification
commands and their latest results, and any pending user approvals or unanswered
questions. (The PreCompact hook already injects `.research/`/`.plans/` file
pointers — this rule covers the rest.)

## Directory Layout

`~/.claude/` mixes user-maintained configuration with runtime state — the repo
`.gitignore` and the `settings.json` deny rules are the source of truth. Runtime
dirs (`sessions/`, `cache/`, `file-history/`, `telemetry/`) and `deplicated/` are
edit-blocked; `projects/` stays writable because auto-memory lives there. Keep
`../rules/AGENTS.md` self-contained and in sync with `../rules/SOUL.md`.

## Skills

The `group:` field in SKILL.md frontmatter is the single source of truth for
classification — view the catalog via `/skills` (the `skill-index` meta-skill);
no static table is duplicated here. Keep skill bodies portable across harnesses:
when a step depends on a Claude-only feature, name a fallback the other harnesses
can follow (see the shared AGENTS.md "Skills (Shared Catalog)" rule).

### Workflow Index

```
[New project]    spec-planner → sprint-contract-negotiator → annotate-plan
                 → implement-plan → qa-evaluator → commit
[Existing code]  deep-read → annotate-plan → implement-plan → commit
[Skill upkeep]   skill-improver → generate-skills → maintain
[Writing]        prompting-assist → humanizer
[Design]         frontend-design-evaluator → multi-agent-orchestrator
```
