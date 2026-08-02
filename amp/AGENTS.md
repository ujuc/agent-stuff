# Amp Global Configuration

@~/.config/dotrc/agents/rules/AGENTS.md

This directory is the canonical source for user-maintained Amp configuration.
Files are deployed individually into `~/.config/amp`; always edit this repository,
not the symlink targets. The imported file above provides shared cross-agent
guidance, while this file contains Amp-specific configuration only.

## Configuration Boundaries

- Keep stable, secret-free user preferences in `settings.json`.
- Amp loads `~/.claude/skills/` automatically; keep reusable skills in the shared
  Claude skill catalog instead of duplicating them under this directory.
- Add Amp-only plugins, checks, or skills only when an Amp-native implementation is
  required. Do not mirror Claude agents or hooks that Amp cannot consume directly.
- Prefer skill-bundled `mcp.json` files over global MCP servers so tools are loaded
  only when needed. Reference credentials through environment variables; never
  commit tokens, OAuth credentials, or machine-specific secrets.

## Managed Files

- `AGENTS.md` — Amp-specific global guidance plus the shared guidance import.
- `settings.json` — Amp CLI user settings deployed as
  `~/.config/amp/settings.json`.
