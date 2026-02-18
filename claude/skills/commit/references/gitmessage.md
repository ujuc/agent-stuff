# Git Commit Message Guide

> Synthesized from [mitsuhiko/agent-stuff](references/mitsuhiko-commit.md) and
> [antigravity-awesome-skills (Sentry)](references/antigravity-commit.md),
> adapted for Korean Conventional Commits with `-하다` endings.

## Principles

- Each commit should be a **single, stable change** — the repository must work after every commit.
- Commits should be **independently reviewable**.
- Reveal *why* you changed something, not just *what* changed.
- Priority: **why > what > how**.

## Format

```
<type>(<scope>): <한국어 제목 -하다>

<본문>

<이슈 참조>
```

### Subject line

- Written in Korean, ending with `-하다` (e.g., `추가하다`, `수정하다`, `제거하다`)
- 50 characters or fewer (including type prefix)
- No trailing period
- Choose the most specific `type` — avoid defaulting to `chore`

### Body (optional)

Include a body only when the change's intent is non-obvious.

- Explain **what** and **why**, not *how*
- Contrast with previous behavior when relevant
- 72 characters per line, `-` for bullet points
- Separate from subject with a blank line

### Footer (optional)

- Related issue number or URL: `Closes #42`, `Refs #13`

## Commit Types

| Type       | Description                          |
|------------|--------------------------------------|
| `feat`     | New feature                          |
| `fix`      | Bug fix                              |
| `refactor` | Code refactoring (no behavior change)|
| `style`    | Formatting only (no logic change)    |
| `docs`     | Documentation changes                |
| `test`     | Add or refactor tests                |
| `chore`    | Build, tooling, package manager, etc.|

## Argument Handling

Caller-provided arguments influence the commit:

- **File paths/globs** → limit which files to stage and commit
- **Freeform instructions** → influence scope, summary, and body
- **Combined** → honor both file selection and instructions
- **Ambiguous files** → always ask the user before staging

## Prohibitions

- Do NOT add `Co-Authored-By` (system handles this automatically)
- Do NOT add sign-offs (`Signed-off-by`)
- Do NOT include breaking-change markers
- Do NOT push — only commit

## Examples

```
feat(skills): commit 스킬을 추가하다

한국어 Conventional Commits 규칙에 따른 커밋 생성 스킬.
mitsuhiko, antigravity 레퍼런스와 gitmessage 가이드를 포함한다.
```

```
fix(zshrc): 플러그인 로드 순서 오류를 수정하다

zimfw 초기화 전에 PATH 설정이 선행되어야 하는데 순서가 뒤바뀌어
일부 툴(pyenv, rbenv)이 정상 동작하지 않던 문제를 수정한다.
```

```
chore(agents): 서브모듈을 업데이트하다
```

## Notes for AI Generation

1. Analyze the diff to understand *intent*, not just line changes.
2. Write the subject in Korean with `-하다` ending.
3. If the change has a non-obvious reason, always include a body.
4. Keep subject ≤ 50 chars; body lines ≤ 72 chars.
5. Stage only the intended files — if unclear, ask.
