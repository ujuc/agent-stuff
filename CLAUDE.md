# CLAUDE.md — agent-stuff

Centralized AI agent configuration repository. Manages settings for Claude Code, Gemini CLI, and Pi agent, deployed via symlinks from the parent [dotrc](https://github.com/ujuc/dotrc) repository.

## Key Commands

```bash
# Validate markdown links in Claude guides
claude/scripts/lint-docs.sh

# Check Claude Code settings syntax
python3 -m json.tool claude/settings.json > /dev/null

# Check MCP server configuration syntax
python3 -m json.tool claude/mcp.json > /dev/null
```

## Architecture

### Repository Structure

```
agents/
├── CLAUDE.md              # THIS FILE - Repository entry point
├── README.md              # Brief repository description
├── LICENSE                 # MIT License
├── specs/
│   └── SOUL.md            # Shared agent mission and values
├── claude/                # Claude Code config (→ ~/.claude)
│   ├── CLAUDE.md          # Claude Code configuration guide
│   ├── system-rules.md    # Critical rules (highest priority)
│   ├── settings.json      # CLI settings
│   ├── mcp.json           # MCP server configuration
│   ├── statusline-command.sh
│   ├── guides/            # 16 guideline documents
│   ├── skills/            # Auto-discovered skills (6 skills)
│   └── scripts/           # Automation scripts
├── gemini/                # Gemini CLI config (→ ~/.gemini)
│   └── .gitkeep
├── pi/                    # Pi agent config (→ ~/.pi)
│   └── .gitkeep
├── .claude/
│   └── settings.json      # Claude Code hooks for this repo
└── .entire/
    └── settings.json      # Entire tool settings
```

### Symlink Deployment

This repository is a git submodule of [dotrc](https://github.com/ujuc/dotrc) at `agents/`. Each agent directory symlinks to its expected location:

| Source            | Target       | Description              |
| ----------------- | ------------ | ------------------------ |
| `claude/`         | `~/.claude`  | Claude Code global config |
| `gemini/`         | `~/.gemini`  | Gemini CLI config         |
| `pi/`             | `~/.pi`      | Pi agent config           |

Always edit files in this repository, not at symlink targets.

### Symlink Path Constraints

Symlinked directories resolve relative paths from the **link target**, not the source. For example, `claude/` is deployed as `~/.claude`, so a relative path `../specs/SOUL.md` inside `claude/CLAUDE.md` would resolve to `~/../specs/SOUL.md` — which does not exist.

**Rule**: Files inside symlinked agent directories (`claude/`, `gemini/`, `pi/`) must NOT use relative paths to reference files outside their own directory tree (e.g., `specs/`, root-level files).

**Correct approaches**:
- Embed shared content directly (inline) with a comment noting the canonical source
- Use the repository-level `CLAUDE.md` (this file) for cross-directory references, as it is not affected by symlinks

### Shared Specifications

`specs/SOUL.md` defines the shared mission and values that apply across all agents:

- Accuracy over speed
- Simplest, safest solutions
- Maintainable, testable code
- Clear risk assessment

## Agent Details

### Claude Code (`claude/`)

The most comprehensive configuration. See [claude/CLAUDE.md](./claude/CLAUDE.md) for full details.

Key components:
- **16 guides** covering philosophy, technical standards, security, performance, and more
- **6 skills**: agents, commit, interview, refactor, review, troubleshoot
- **MCP server**: sequential-thinking for structured reasoning
- **Priority hierarchy**: system-rules.md > CLAUDE.md > domain guides

### Gemini CLI (`gemini/`)

Placeholder for Gemini CLI agent configuration. Currently empty (`.gitkeep`).

### Pi Agent (`pi/`)

Placeholder for Pi agent configuration. Currently empty (`.gitkeep`).

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

### Adding a New Agent

1. Create directory: `<agent-name>/`
2. Add configuration files for the agent
3. Set up symlink deployment in dotrc
4. Update this CLAUDE.md structure section
5. Add `.gitignore` rules for runtime files if needed

## Cross-References

- **[claude/CLAUDE.md](./claude/CLAUDE.md)** — Claude Code configuration, skills, guides, priority hierarchy
- **[specs/SOUL.md](./specs/SOUL.md)** — Shared agent mission and values
- **[dotrc CLAUDE.md](https://github.com/ujuc/dotrc)** — Parent repository documentation
