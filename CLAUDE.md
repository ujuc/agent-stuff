# CLAUDE.md — agent-stuff

Personal AI agent configuration repository. Git submodule of [dotrc](https://github.com/ujuc/dotrc), deployed via symlinks to each tool's expected system location.

## Technical Stack

- Symlink deployment model (this repo → dotrc submodule → ~/.claude, ~/.pi)

## Architecture

| Source    | Target      | Status      |
| --------- | ----------- | ----------- |
| `claude/` | `~/.claude` | Active      |
| `pi/`     | `~/.pi`     | Placeholder |

**Critical**: Files inside symlinked directories (`claude/`, `pi/`) must NOT use relative paths to reference outside their own tree.

## Development Commands

No build or test toolchain. This is a pure configuration repository.

## Work Rules

- Commit directly to `main` (no branches/PRs)
- Korean Conventional Commits ending with `-하다`:
  ```
  <type>(<scope>): <Korean subject ending with -하다>
  ```
- **Types**: feat, fix, docs, style, refactor, test, chore
- **Scopes**: claude, pi, specs, skills

## Behavioral Guidelines

- `claude/deplicated/` is fully deprecated — do not reference or modify
- `specs/SOUL.md` is the canonical shared mission — keep `claude/CLAUDE.md` Agent Identity in sync
- Always edit files in this repository, not at symlink targets

## References

- **[AGENTS.md](./AGENTS.md)** — Full project structure and detailed guide
