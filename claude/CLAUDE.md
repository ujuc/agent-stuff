# CLAUDE.md

@~/.config/dotrc/agents/rules/AGENTS.md

Claude Code global configuration directory. Symlinked as `~/.claude` from the dotrc
repository. Always edit files here, not at the symlink target. Cross-agent guidance
(identity, rule authoring policy, git, language, interaction principles, tool
language, boundaries) comes from the import above — this file holds Claude-specific
configuration only.

## Interaction Rules (Claude-specific)

- When the user asks for a behavior that must happen "always / every time", wire it
  as a hook in `settings.json` (deterministic) via the `update-config` skill — do not
  add another advisory CLAUDE.md rule
- The shared '업데이트/변경사항' clarification rule is nudged deterministically by a
  `UserPromptSubmit` hook (`hooks/clarify-update-word.sh`) — act on its injected
  reminder when it fires

## Model Quality Safeguards

When the active model is below the Opus tier (Sonnet, Haiku, or other), call
`advisor()` at the following gates:

- **Before commit / push / publish** (git commit, gh pr create, etc.)
- **Before finalizing substantive analysis** (recommendations, root cause
  conclusions, design decisions)
- **Before shipping work the user will act on** (code handed off, configs applied,
  published artifacts)

Exceptions (skip advisor — adds noise without value):

- Trivial reactive tasks: single-line edits, file reads, lookups, mechanical renames
- Tasks where the next action is dictated by tool output you just read
- When the user has explicitly waived advisor for the current task

Detection: identify the active model from the environment block — `claude-opus-*`,
`claude-fable-*`, and `claude-mythos-*` are Opus-tier or above (exempt); anything
else requires advisor.

## Execution Delegation (Cost- & Context-Aware)

When an available agent fits the task, dispatch it instead of working inline —
delegation protects the main context as much as it saves cost. Subagents explore
in their own context window and return only a summary:

- **Broad investigation → `Explore` subagent.** Multi-file sweeps, "how does X
  work" research, and any search that would dump many file reads into the main
  context.
- **Run-and-report → `haiku` subagent.** Running a command (or a sweep of them)
  and capturing/summarizing the result without reasoning over it (Agent tool,
  `model: haiku`).
- **Text-only transforms → local `gemma` skill.** Summarize, translate, classify,
  or draft from text already in hand (`--local`; `GEMMA_NO_FALLBACK=1` for
  sensitive data so it never leaves the machine). `gemma` cannot execute system
  commands — text only.
- **Keep on the active model** when the output drives a decision, edit, or analysis.
  Delegated output is a draft — verify before acting (see Model Quality Safeguards).

Exception: trivial single commands (`ls`, `git status`) — run inline; subagent
spin-up costs more than it saves.

## Workflow Orchestration (Cost-Aware)

The `Workflow` tool runs deterministic multi-agent scripts (fan-out, pipeline,
adversarial verify) and can spawn dozens of agents — reserve it for work that
genuinely needs that scale: skill evals, rule-compliance checks, claim-source
cross-verification, bulk triage. NOT for ordinary coding or single-file edits —
dispatch one Agent or work inline instead. Before any large run, gauge cost on
a narrow slice and state the token budget explicitly; route steps that don't
need a strong model to a smaller one. A per-session effort directive (e.g.
ultracode) can raise this default — honor it when set.

## Context Compaction

When compacting, always preserve: the list of modified files, verification
commands and their latest results, and any pending user approvals or unanswered
questions. (The PreCompact hook already injects `.research/`/`.plans/` file
pointers — this rule covers the rest.)

## Directory Layout

`~/.claude/` mixes user-maintained configuration with runtime state. Edit only these paths:

- `skills/<name>/SKILL.md` — user skill definitions (see [skills/CLAUDE.md](./skills/CLAUDE.md))
- `agents/<name>.md` — custom subagents dispatched by skills and the Agent tool
- `hooks/*.sh` — executable hook scripts wired via `settings.json`
- `output-styles/*.md` — custom output styles referenced by `settings.json`
- `evals/<skill>/` — waza evaluation suites (run only via the `waza-runner` agent)
- `settings.json` — permissions, hooks, env vars
- `../rules/AGENTS.md` — shared cross-agent guidance (imported above; also symlinked
  into Codex/Amp) — keep self-contained and in sync with `../rules/SOUL.md`

`deplicated/` and the runtime dirs (`sessions/`, `cache/`, `file-history/`,
`telemetry/`) are edit-blocked by permission deny rules in `settings.json`;
`projects/` stays writable because auto-memory lives there. Other top-level
subdirectories (`tasks/`, `memory/`, etc.) are runtime-managed and gitignored.

## Priority Hierarchy

When guidelines conflict: **CLAUDE.md** (this file) takes precedence over the
imported shared guidance and project overrides. System rules can NEVER be
overridden without explicit approval.

## Skills

Triggered by natural language; invoke via the Skill tool when a trigger matches. Located in `skills/<skill-name>/SKILL.md`. The `group:` field in SKILL.md frontmatter is the single source of truth for classification — view the full catalog (groups · triggers · models) via `/skills` (or `스킬 목록 보여줘`), which runs the `skill-index` meta-skill to merge `group:` frontmatter with plugin commands; no static table is duplicated here.

Skills are cross-harness assets: Codex and Amp consume the same `SKILL.md` files
(user-global and project `.claude/skills/`) via the shared AGENTS.md "Skills
(Shared Catalog)" rule. Keep skill bodies portable — when a step depends on a
Claude-only feature (Skill/Agent tool, subagents, hooks), name a fallback the
other harnesses can follow.

### Workflow Index

```
[New project]    spec-planner → sprint-contract-negotiator → annotate-plan
                 → implement-plan → qa-evaluator → commit
[Existing code]  deep-read → annotate-plan → implement-plan → commit
[Skill upkeep]   skill-improver → generate-skills → maintain
[Writing]        prompting-assist → humanizer
[Design]         frontend-design-evaluator → multi-agent-orchestrator
```

> The 7-day skill-improver cadence check runs as a SessionStart hook
> (`hooks/skill-improver-cadence.sh`) — follow its injected consent flow when it
> fires; never auto-run skill-improver without consent.
