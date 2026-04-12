---
name: agent-stuff
description: Personal AI agent configuration repository
standard: agents.md/v1
---

## Project Overview

Personal AI agent configuration repository. Manages global settings for Claude Code. Deployed as a git submodule of [dotrc](https://github.com/ujuc/dotrc) via symlinks to each tool's expected system location.

### Key Files

| File | Purpose |
| ---- | ------- |
| `rules/SOUL.md` | Canonical shared mission and values. |
| `claude/settings.json` | Global Claude Code settings (model, permissions, hooks, statusline, etc.). |

## Operational Gotchas

- `claude/` is symlinked to `~/.claude` — files inside it must NOT use relative paths to reference outside their own tree
- `claude/CLAUDE.md` is loaded as global config in every Claude Code session, not just within this repo
- `claude/plugins/` contains externally installed plugins — treat as read-only, not project code

## Non-Obvious Conventions

- Commit scopes map to directories: `claude` → `claude/`, `rules` → `rules/`, `skills` → skill definitions
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
