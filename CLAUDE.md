# CLAUDE.md — agent-stuff

Personal AI agent configuration repository. Git submodule of [dotrc](https://github.com/ujuc/dotrc), deployed via symlinks to each tool's expected system location.

## Technical Stack

- Symlink deployment model: `claude/` → `~/.claude` (this repo is a dotrc submodule)

## Development Commands

No build/run toolchain — this is a configuration repository. The one verification
loop is for skill changes:

- Validate a skill: `bash claude/skills/generate-skills/scripts/validate-skill claude/skills/<name>` (first run compiles a Rust workspace, ~6–30s)
- Eval suites live in `claude/evals/<skill>/` — run them via the `waza-runner` agent, never the `waza` CLI directly
- After editing a skill, run `skill-improver` before committing

## Work Rules

- Commit directly to `main` (no branches/PRs)
- Korean Conventional Commits ending with `-하다`, e.g. `feat(skills): 새 스킬을 추가하다`
- **Types**: feat, fix, docs, style, refactor, test, chore
- **Scopes**: claude, rules, skills

## Behavioral Guidelines

- `claude/` is symlinked to `~/.claude`; `claude/CLAUDE.md` and `claude/settings.json` are GLOBAL — they load/apply in every Claude Code session, not just this repo. High blast radius.
- `claude/` mixes tracked config with gitignored runtime state — `.gitignore` is the source of truth for what's runtime. Don't edit gitignored paths (`sessions/`, `cache/`, `file-history/`, `projects/`, …) or `claude/plugins/` (externally installed).
- `claude/deplicated/` is fully deprecated — do not reference or modify
- Always edit files in this repository, not at symlink targets

## Skills

| Skill | Triggers | Model |
| ----- | -------- | ----- |
| `maintain` (project-scoped, `.claude/skills/maintain/`) | /maintain, 정비해줘, 헬스체크, 문서 동기화 | opus |

## References

- **[AGENTS.md](./AGENTS.md)** — Full project structure and detailed guide