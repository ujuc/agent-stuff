---
name: agent-stuff
description: Personal AI agent configuration repository
standard: agents.md/v1
---

## Project Overview

Personal AI agent configuration repository. Manages global settings for Claude Code. Deployed as a git submodule of [dotrc](https://github.com/ujuc/dotrc) via symlinks to each tool's expected system location.

### Repository Structure

```
agent-stuff/
├── .claude/
│   ├── agents/              # Project-scoped agent definitions
│   └── skills/
│       └── maintain/        # Project-scoped maintenance orchestrator skill
├── claude/                  # Symlinked to ~/.claude — global Claude Code config
│   ├── agents/              # Global agent definitions
│   ├── deplicated/          # Deprecated — do not reference or modify
│   ├── hooks/               # Claude Code hook scripts (polyglot-typecheck, etc.)
│   ├── output-styles/       # Custom output styles referenced by settings.json
│   ├── plugins/             # Externally installed plugins
│   ├── evals/               # waza evaluation suites — one subdirectory per evaluated skill
│   ├── skills/              # Global skill definitions (each with SKILL.md)
│   │   └── CLAUDE.md        # Nested config — skill work rules and model-assignment conventions
│   ├── CLAUDE.md            # Global Claude Code configuration (loaded in every session)
│   ├── mcp.json             # MCP server configuration
│   └── settings.json        # Claude Code settings (model, permissions, hooks, statusline)
├── rules/
│   ├── AGENTS.md            # Canonical cross-agent guidance (Claude import + Codex/Amp symlinks)
│   └── SOUL.md              # Canonical agent mission and values (Korean)
├── AGENTS.md                # This file — project structure and contributor guide
├── CLAUDE.md                # Project-scoped Claude Code overrides
└── README.md                # Repository overview (Korean)
```

### Key Files

| File | Purpose |
| ---- | ------- |
| `rules/SOUL.md` | Canonical shared mission and values (Korean). Source of truth for Agent Identity. |
| `rules/AGENTS.md` | Canonical cross-agent guidance. Imported by `claude/CLAUDE.md` (`@~/` path); symlinked as `~/.codex/AGENTS.md` and `~/.config/amp/AGENTS.md`. Self-contained, < 8 KB. |
| `claude/CLAUDE.md` | Global Claude Code configuration — loaded in every session, not just this repo. |
| `claude/settings.json` | Global Claude Code settings (model, permissions, hooks, statusline, etc.). |
| `claude/mcp.json` | MCP (Model Context Protocol) server configuration. Empty by default — servers are typically added via Claude Code UI, not committed here. |
| `claude/skills/` | Global skill definitions — each in `<name>/SKILL.md` with optional `references/` and `scripts/` / `tools/`. |
| `CLAUDE.md` | Project-scoped Claude Code overrides (commit rules, work rules). |

## Operational Gotchas

- `.githooks/commit-msg` requires the subject to end in a Korean verb declarative — the literal `다` (`.+다$`), so any verb stem passes (`추가하다`, `걷어내다`, `지우다`). Noun endings (`업데이트`, `정리함`), a trailing period, and English subjects fail. It applies only where `core.hooksPath` is set
- `claude/` is symlinked to `~/.claude` — files inside it must NOT use relative paths to reference outside their own tree
- `claude/CLAUDE.md` and `claude/settings.json` are GLOBAL — they load/apply in every Claude Code session, not just within this repo. High blast radius.
- `claude/` mixes tracked config with gitignored runtime state — the repo `.gitignore` is the source of truth for what is runtime. Never edit gitignored paths (`sessions/`, `cache/`, `file-history/`, `projects/`, …)
- `claude/plugins/` contains externally installed plugins — treat as read-only, not project code
- `rules/AGENTS.md` is consumed outside this repo (Claude `@~/` import, `~/.codex/AGENTS.md` and `~/.config/amp/AGENTS.md` symlinks) — keep it self-contained (no Claude-only tool references) and under 8 KB

## Non-Obvious Conventions

- Commit scopes map to directories: `claude` → `claude/`, `rules` → `rules/`, `skills` → skill definitions
- Agent/skill scope split: `.claude/<type>/` is project-scoped (visible only inside this repo), `claude/<type>/` is global (symlinked to `~/.claude/`) — place repo-maintenance agents in `.claude/agents/`, reusable ones in `claude/agents/`
- Agent Identity in `rules/AGENTS.md` must stay in sync with `rules/SOUL.md` (marked with `<!-- canonical source -->` comment); `claude/CLAUDE.md` imports the shared file instead of duplicating it
- Add `.gitignore` entries when agents introduce new runtime files under `claude/`

## Boundaries

### Always Do

- Keep `claude/CLAUDE.md` Agent Identity in sync with `rules/SOUL.md`
- Use correct scope in commit messages matching the directory modified
- Edit files in this repository, never at symlink targets (`~/.claude`)

### Ask First

- Modifying `claude/settings.json` or `claude/mcp.json` (affects global Claude Code behavior)
- Adding new agent directories (needs symlink setup in dotrc)
- Changes to `rules/SOUL.md` (affects all agents)

### Never Do

- Reference or modify files in `claude/deplicated/` (fully deprecated)
- Use relative paths in symlinked directories to reference outside their tree
- Add code style rules to CLAUDE.md (delegate to linters)
