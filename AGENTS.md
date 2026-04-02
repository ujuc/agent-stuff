---
name: agent-stuff
description: Personal AI agent configuration repository
version: "1.0"
standard: agents.md/v1
---

## Project Overview

Personal AI agent configuration repository. Manages global settings for Claude Code. Deployed as a git submodule of [dotrc](https://github.com/ujuc/dotrc) via symlinks to each tool's expected system location.

### Technical Stack

- **Deployment**: Git submodule of [dotrc](https://github.com/ujuc/dotrc) + symlinks

## Repository Structure

```
agent-stuff/
├── CLAUDE.md              # Project-level Claude Code instructions
├── AGENTS.md              # This file — project guide for AI agents
├── README.md
├── .claude/               # Project-specific settings and harness
│   ├── agents/            # Harness agent definitions
│   │   ├── health-checker.md   # Structure validation specialist
│   │   ├── doc-syncer.md       # Documentation sync specialist
│   │   └── skill-engineer.md   # Skill lifecycle specialist
│   └── skills/
│       └── maintain/            # Orchestrator: repo maintenance
├── rules/
│   └── SOUL.md            # Shared agent mission and values (canonical)
├── claude/                # Claude Code global config (→ ~/.claude)
│   ├── CLAUDE.md          # Global Claude Code configuration
│   ├── RTK.md             # RTK (Rust Token Killer) reference
│   ├── settings.json      # Global settings (model, permissions, hooks, etc.)
│   ├── mcp.json           # MCP server configuration (placeholder)
│   ├── plugins/config.json
│   ├── skills/
│   │   ├── autoresearch/                  # Active skill: Autonomous skill optimizer
│   │   ├── commit/                        # Active skill: Korean Conventional Commits
│   │   ├── frontend-design-evaluator/     # Active skill: Frontend design quality evaluator
│   │   ├── generate-claude-md/            # Active skill: CLAUDE.md/AGENTS.md generator
│   │   ├── generate-skills/              # Active skill: Skill generator
│   │   ├── multi-agent-orchestrator/      # Active skill: Multi-agent pipeline orchestrator
│   │   ├── qa-evaluator/                  # Active skill: Web app QA tester via Chrome
│   │   ├── spec-planner/                  # Active skill: Prompt-to-spec expander
│   │   └── sprint-contract-negotiator/    # Active skill: Done-criteria negotiation protocol
│   └── deplicated/        # DEPRECATED — do not use
```

### Key Files

| File | Purpose |
| ---- | ------- |
| `rules/SOUL.md` | Canonical shared mission and values. Sync changes to `claude/CLAUDE.md` Agent Identity. |
| `claude/settings.json` | Global Claude Code settings (model, permissions, hooks, statusline, etc.). |

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
| `rules`  | Changes to `rules/` directory  |
| `skills` | Changes to skill definitions   |

### Examples

- `feat(claude): 새 MCP 서버 설정을 추가하다`
- `docs(rules): SOUL.md 사명 섹션을 업데이트하다`

## Boundaries

### Always Do

- Edit files in this repository, never at symlink targets
- Keep `claude/CLAUDE.md` Agent Identity in sync with `rules/SOUL.md`
- Use correct scope in commit messages matching the directory modified
- Add `.gitkeep` when creating new placeholder directories
- Add `.gitignore` entries for agent runtime files

### Ask First

- Modifying `claude/settings.json` or `claude/mcp.json` (affects global Claude Code behavior)
- Adding new agent directories (needs symlink setup in dotrc)
- Changes to `rules/SOUL.md` (affects all agents)

### Never Do

- Reference or modify files in `claude/deplicated/` (fully deprecated)
- Use relative paths in symlinked directories to reference outside their tree
- Add code style rules to CLAUDE.md (delegate to linters)
