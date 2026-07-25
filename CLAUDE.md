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
- **Scopes**: claude, rules, skills
- Subject format and the allowed type list are enforced by `.githooks/commit-msg` — read the regex there instead of restating it
