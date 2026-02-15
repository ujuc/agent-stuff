# CLAUDE.md — agent-stuff

Centralized AI agent configuration repository. Manages settings for Claude Code, Gemini CLI, and Pi agent, deployed via symlinks from the parent [dotrc](https://github.com/ujuc/dotrc) repository.

## Architecture

### Repository Structure

```
agents/
├── CLAUDE.md              # THIS FILE
├── specs/
│   └── SOUL.md            # Shared agent mission and values
├── claude/                # Claude Code config (→ ~/.claude)
│   ├── CLAUDE.md          # Configuration guide
│   ├── system-rules.md    # Critical rules (highest priority)
│   ├── settings.json, mcp.json
│   ├── guides/            # 16 guideline documents
│   ├── skills/            # 6 auto-discovered skills
│   └── scripts/           # Automation scripts
├── gemini/                # Gemini CLI config (→ ~/.gemini)
└── pi/                    # Pi agent config (→ ~/.pi)
```

### Symlink Deployment

This repository is a git submodule of [dotrc](https://github.com/ujuc/dotrc) at `agents/`. Each agent directory symlinks to its expected location:

| Source    | Target      | Description               |
| --------- | ----------- | ------------------------- |
| `claude/` | `~/.claude` | Claude Code global config |
| `gemini/` | `~/.gemini` | Gemini CLI config         |
| `pi/`     | `~/.pi`     | Pi agent config           |

Always edit files in this repository, not at symlink targets.

**Constraint**: Files inside symlinked directories must NOT use relative paths to reference outside their tree — embed shared content inline or reference from this file.

### Shared Specifications

`specs/SOUL.md` defines shared mission and values across all agents. Claude Code embeds this inline in claude/CLAUDE.md:20-47.

## Agent Details

### Claude Code (`claude/`)

See claude/CLAUDE.md for full details: 16 guides, 6 skills, MCP server, priority hierarchy.

### Gemini CLI (`gemini/`) / Pi Agent (`pi/`)

Placeholder directories. Currently empty.

## Conventions

### Language Policy

- **User-facing output** (terminal responses, commit subjects): Korean
- **File content** (code, comments, docs, commit body): English

### Commit Messages

Korean Conventional Commits ending with `-하다`:

```
<type>(<scope>): <Korean subject ending with -하다>
```

**Scopes**: `claude`, `gemini`, `pi`, `specs`, `skills`, `guides`

Examples:

- `feat(claude): 새 MCP 서버 설정을 추가하다`
- `docs(specs): SOUL.md 사명 섹션을 업데이트하다`
- `chore(gemini): Gemini CLI 초기 설정을 추가하다`

## Cross-References

- **[claude/CLAUDE.md](./claude/CLAUDE.md)** — Claude Code configuration, skills, guides, priority hierarchy
- **[specs/SOUL.md](./specs/SOUL.md)** — Shared agent mission and values
- **[dotrc CLAUDE.md](https://github.com/ujuc/dotrc)** — Parent repository documentation
