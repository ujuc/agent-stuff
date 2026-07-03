# CLAUDE.md — agent-stuff

@AGENTS.md

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

## Skills

| Skill | Triggers | Model |
| ----- | -------- | ----- |
| `maintain` (project-scoped, `.claude/skills/maintain/`) | /maintain, 정비해줘, 헬스체크, 문서 동기화 | opus |
