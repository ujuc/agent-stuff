# CLAUDE.md — agent-stuff

Personal AI agent configuration repository. Git submodule of [dotrc](https://github.com/ujuc/dotrc), deployed via symlinks to each tool's expected system location.

## Technical Stack

- Symlink deployment model (this repo → dotrc submodule → ~/.claude)

## Architecture

| Source    | Target      | Status      |
| --------- | ----------- | ----------- |
| `claude/` | `~/.claude` | Active      |

## Development Commands

No build or test toolchain. This is a pure configuration repository.

## Work Rules

- Commit directly to `main` (no branches/PRs)
- Korean Conventional Commits ending with `-하다`, e.g. `feat(skills): 새 스킬을 추가하다`
- **Types**: feat, fix, docs, style, refactor, test, chore
- **Scopes**: claude, rules, skills

## Behavioral Guidelines

- `claude/deplicated/` is fully deprecated — do not reference or modify
- Always edit files in this repository, not at symlink targets

## Skills

| Skill | Triggers | Model |
| ----- | -------- | ----- |
| `maintain` | /maintain, 정비해줘, 헬스체크, 문서 동기화 | opus |

## References

- **[AGENTS.md](./AGENTS.md)** — Full project structure and detailed guide