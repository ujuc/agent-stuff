---
name: agent-stuff
description: Personal AI agent configuration repository
standard: agents.md/v1
---

## Project Overview

Personal AI agent configuration repository. Manages global settings for Claude Code and Amp. Deployed as a git submodule of [dotrc](https://github.com/ujuc/dotrc) via symlinks to each tool's expected system location.

## Operational Gotchas

- `.githooks/commit-msg` requires the subject to end in a Korean verb declarative — the literal `다` (`.+다$`), so any verb stem passes (`추가하다`, `걷어내다`, `지우다`). Noun endings (`업데이트`, `정리함`), a trailing period, and English subjects fail. It applies only where `core.hooksPath` is set
- `claude/` is symlinked to `~/.claude` — files inside it must NOT use relative paths to reference outside their own tree
- `claude/CLAUDE.md` and `claude/settings.json` are GLOBAL — they load/apply in every Claude Code session, not just within this repo. High blast radius.
- `amp/AGENTS.md` and `amp/settings.json` are individually symlinked into `~/.config/amp` and apply to every local Amp session. Keep runtime state and credentials outside this repository.
- `claude/` mixes tracked config with gitignored runtime state — the repo `.gitignore` is the source of truth for what is runtime. Never edit gitignored runtime paths (`sessions/`, `cache/`, `file-history/`, `telemetry/`, …). `claude/projects/` is the one exception: Claude Code's auto-memory writes to `projects/<project>/memory/`, so that subtree stays writable
- `claude/plugins/` contains externally installed plugins — treat as read-only, not project code
- `claude/mcp.json` is empty by default — MCP servers are normally added through the Claude Code UI (which persists them to `~/.claude.json`), not committed here
- `rules/AGENTS.md` is consumed outside this repo (Claude/Amp `@~/` imports and the `~/.codex/AGENTS.md` symlink) — keep it self-contained (no harness-specific tool references) and under 8 KB

## Non-Obvious Conventions

- Commit scopes map to directories: `amp` → `amp/`, `claude` → `claude/`, `rules` → `rules/`, `skills` → skill definitions
- Agent/skill scope split: `.claude/<type>/` is project-scoped (visible only inside this repo), `claude/<type>/` is global (symlinked to `~/.claude/`) — place repo-maintenance agents in `.claude/agents/`, reusable ones in `claude/agents/`
- Agent Identity in `rules/AGENTS.md` must stay in sync with `rules/SOUL.md` (marked with `<!-- canonical source -->` comment); `claude/CLAUDE.md` imports the shared file instead of duplicating it
- Amp automatically loads `~/.claude/skills/`; keep portable global skills in `claude/skills/` and reserve `amp/` for Amp-native configuration.
- Add `.gitignore` entries when agents introduce new runtime files under `claude/`

## Boundaries

### Always Do

- Keep the Agent Identity in `rules/AGENTS.md` in sync with `rules/SOUL.md`
- Use correct scope in commit messages matching the directory modified
- Edit files in this repository, never at symlink targets (`~/.claude`, `~/.config/amp`)

### Ask First

- Modifying `claude/settings.json`, `claude/mcp.json`, or `amp/settings.json` (affects global agent behavior)
- Adding new agent directories (needs symlink setup in dotrc)
- Changes to `rules/SOUL.md` (affects all agents)

### Never Do

- Reference or modify files in `claude/deplicated/` (fully deprecated)
- Use relative paths in symlinked directories to reference outside their tree
- Add code style rules to CLAUDE.md (delegate to linters)
