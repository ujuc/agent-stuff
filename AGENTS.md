---
name: agent-stuff
description: Personal AI agent configuration repository
version: "1.0"
standard: agents.md/v1
---

## Project Overview

Personal AI agent configuration repository. Manages global settings for Claude Code, Gemini CLI, and Pi agent. Deployed as a git submodule of [dotrc](https://github.com/ujuc/dotrc) via symlinks to each tool's expected system location.

### Technical Stack

- **Configuration**: Markdown, JSON
- **Deployment**: Git submodule + symlinks
- **CI/CD**: None

## Repository Structure

```
agent-stuff/
├── CLAUDE.md              # Project-level Claude Code instructions
├── AGENTS.md              # This file — project guide for AI agents
├── README.md
├── .claude/               # Project-specific Claude Code settings
│   └── settings.json      # Hooks (entire integration), permissions
├── specs/
│   └── SOUL.md            # Shared agent mission and values (canonical)
├── claude/                # Claude Code global config (→ ~/.claude)
│   ├── CLAUDE.md          # Global Claude Code configuration
│   ├── settings.json      # Model, permissions, sandbox, language
│   ├── mcp.json           # MCP servers (sequential-thinking)
│   ├── statusline-command.sh
│   ├── plugins/config.json
│   ├── memory/MEMORY.md   # Auto memory (persistent across sessions)
│   ├── skills/
│   │   └── generate-claude-md/  # Active skill: CLAUDE.md/AGENTS.md generator
│   └── deplicated/        # DEPRECATED — do not use
├── gemini/                # Gemini CLI config (→ ~/.gemini) — placeholder
│   └── .gitkeep
└── pi/                    # Pi agent config (→ ~/.pi) — placeholder
    └── .gitkeep
```

### Key Files

| File | Purpose |
| ---- | ------- |
| `specs/SOUL.md` | Canonical shared mission and values. Sync changes to `claude/CLAUDE.md` Agent Identity. |
| `claude/CLAUDE.md` | Global Claude Code configuration. Deployed to `~/.claude/CLAUDE.md`. |
| `.claude/settings.json` | Project-specific hooks (entire integration) and permissions. |
| `claude/settings.json` | Global Claude Code settings (model, sandbox, language, etc.). |
| `claude/mcp.json` | MCP server configuration (sequential-thinking via mise + npx). |

## Build & Test

No build or test toolchain. This is a pure configuration repository.

## Git Workflow

- **Branch strategy**: Direct commit to `main`
- **Commit format**: Korean Conventional Commits ending with `-하다`

```
<type>(<scope>): <Korean subject ending with -하다>
```

### Types

feat, fix, docs, style, refactor, test, chore

### Scopes

| Scope    | When to use                    |
| -------- | ------------------------------ |
| `claude` | Changes to `claude/` directory |
| `gemini` | Changes to `gemini/` directory |
| `pi`     | Changes to `pi/` directory     |
| `specs`  | Changes to `specs/` directory  |
| `skills` | Changes to skill definitions   |
| `guides` | Changes to guide documents     |

### Examples

- `feat(claude): 새 MCP 서버 설정을 추가하다`
- `docs(specs): SOUL.md 사명 섹션을 업데이트하다`
- `chore(gemini): Gemini CLI 초기 설정을 추가하다`

## Boundaries

### Always Do

- Edit files in this repository, never at symlink targets
- Keep `claude/CLAUDE.md` Agent Identity in sync with `specs/SOUL.md`
- Use correct scope in commit messages matching the directory modified
- Add `.gitkeep` when creating new placeholder directories
- Add `.gitignore` entries for agent runtime files

### Ask First

- Modifying `claude/settings.json` or `claude/mcp.json` (affects global Claude Code behavior)
- Adding new agent directories (needs symlink setup in dotrc)
- Changes to `specs/SOUL.md` (affects all agents)

### Never Do

- Reference or modify files in `claude/deplicated/` (fully deprecated)
- Use relative paths in symlinked directories to reference outside their tree
- Add code style rules to CLAUDE.md (delegate to linters)
