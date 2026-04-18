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
│   ├── agents/              # Project-scoped agent definitions (health-checker, doc-syncer, skill-engineer)
│   └── skills/
│       └── maintain/        # Project-scoped maintenance orchestrator skill
├── claude/                  # Symlinked to ~/.claude — global Claude Code config
│   ├── agents/              # Global agent definitions (implementer, researcher, verifier, reference-finder)
│   ├── deplicated/          # Deprecated — do not reference or modify
│   ├── hooks/               # Claude Code hook scripts (rtk-rewrite, polyglot-typecheck, etc.)
│   ├── plugins/             # Externally installed plugins — treat as read-only
│   ├── skills/              # Global skill definitions (15 skills, each with SKILL.md)
│   ├── CLAUDE.md            # Global Claude Code configuration (loaded in every session)
│   ├── RTK.md               # RTK (Rust Token Killer) reference
│   ├── mcp.json             # MCP server configuration
│   └── settings.json        # Claude Code settings (model, permissions, hooks, statusline)
├── docs/
│   ├── plans/               # Implementation plans
│   └── specs/               # Design specifications
├── rules/
│   └── SOUL.md              # Canonical agent mission and values (Korean)
├── AGENTS.md                # This file — project structure and contributor guide
├── CLAUDE.md                # Project-scoped Claude Code overrides
├── LICENSE                  # MIT license
└── README.md                # Repository overview (Korean)
```

### Key Files

| File | Purpose |
| ---- | ------- |
| `rules/SOUL.md` | Canonical shared mission and values (Korean). Source of truth for Agent Identity. |
| `claude/CLAUDE.md` | Global Claude Code configuration — loaded in every session, not just this repo. |
| `claude/settings.json` | Global Claude Code settings (model, permissions, hooks, statusline, etc.). |
| `claude/mcp.json` | MCP (Model Context Protocol) server configuration. Empty by default — servers are typically added via Claude Code UI, not committed here. |
| `claude/skills/` | Global skill definitions — 15 skills, each in `<name>/SKILL.md` with optional `references/` and `scripts/` / `tools/`. |
| `.claude/agents/` | Project-scoped agents for repository maintenance (health-checker, doc-syncer, skill-engineer). |
| `.claude/skills/maintain/` | Project-scoped orchestrator skill that dispatches maintenance agents. |
| `docs/` | Design specs and implementation plans for repository changes. |
| `CLAUDE.md` | Project-scoped Claude Code overrides (commit rules, work rules). |

## Operational Gotchas

- `claude/` is symlinked to `~/.claude` — files inside it must NOT use relative paths to reference outside their own tree
- `claude/CLAUDE.md` is loaded as global config in every Claude Code session, not just within this repo
- `claude/plugins/` contains externally installed plugins — treat as read-only, not project code

## Non-Obvious Conventions

- Commit scopes map to directories: `claude` → `claude/`, `rules` → `rules/`, `skills` → skill definitions
- Agent/skill scope split: `.claude/<type>/` is project-scoped (visible only inside this repo), `claude/<type>/` is global (symlinked to `~/.claude/`) — place repo-maintenance agents in `.claude/agents/`, reusable ones in `claude/agents/`
- Agent Identity in `claude/CLAUDE.md` must stay in sync with `rules/SOUL.md` (marked with `<!-- canonical source -->` comment)
- Add `.gitkeep` when creating new placeholder directories
- Add `.gitignore` entries for agent runtime files

## Boundaries

### Always Do

- Keep `claude/CLAUDE.md` Agent Identity in sync with `rules/SOUL.md`
- Use correct scope in commit messages matching the directory modified

### Ask First

- Modifying `claude/settings.json` or `claude/mcp.json` (affects global Claude Code behavior)
- Adding new agent directories (needs symlink setup in dotrc)
- Changes to `rules/SOUL.md` (affects all agents)

### Never Do

- Reference or modify files in `claude/deplicated/` (fully deprecated)
- Use relative paths in symlinked directories to reference outside their tree
- Add code style rules to CLAUDE.md (delegate to linters)
