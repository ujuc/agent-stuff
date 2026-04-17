---
name: commit
description: "한국어 Conventional Commits 규칙에 따라 git 커밋을 생성한다. 서브모듈 변경 감지·우선 커밋, 문서 자동 업데이트, push, 요약까지 포함. /commit, 커밋해줘, commit, 변경사항 커밋, 커밋하고 푸시해줘 요청 시 사용한다."
model: sonnet
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git log:*), Bash(git add:*), Bash(git commit:*), Bash(git push:*), Bash(git -C:*), Bash(git submodule:*), Bash(bash:*), Read, Edit, Glob
---

# Git Commit

Generate commits per the project's Korean Conventional Commits convention.

## Format

`<type>(<scope>): <한국어 제목 -하다>`

- **scope**: follow the scopes defined in the project's CLAUDE.md.
- Subject and body rules (allowed types, length limits, body priority, etc.) follow `references/gitmessage.md`.

## Procedure

### Step 0. Detect submodule changes

1. Run `git status` and check whether any submodule is reported as modified (modified content, new commits).
2. If a submodule has changes, **process the submodule first**:
   - Inspect with `git -C <submodule> status` and `git -C <submodule> diff`.
   - Stage and commit inside the submodule (apply the same rules as Steps 1–7 below).
   - If the user asked for a push, push the submodule first.
3. Once the submodule is done, return to the parent repo and proceed including the updated submodule pointer.

### Steps 1–7. Parent repo commit

1. Read the user's arguments for file paths or instructions.
2. Inspect changes with `git status` and `git diff`.
3. Run `git log --oneline -20` to learn the recent commit style and scope vocabulary.
4. If which files to stage is unclear, ask the user.
5. Stage only the intended files with `git add`.
6. If structural changes are detected, run an incremental doc update (see "Doc updates" below).
7. Commit using a heredoc:

```bash
git commit -m "$(cat <<'EOF'
<type>(<scope>): <한국어 제목>

<body — only when needed>
EOF
)"
```

## Doc updates

After staging, conditionally apply incremental edits to project documentation.

### Triggers (apply if any holds)

1. File or directory added/removed.
2. New scope candidate (a new top-level directory).
3. External tool dependency added.

### Skip conditions

- Content-only changes to existing files (no structural change).
- Submodule pointer updates.
- `style` or `refactor` type internal changes.
- The project root has no AGENTS.md or CLAUDE.md.

### Procedure

1. Use Glob to confirm AGENTS.md / CLAUDE.md exists at project root.
2. If absent, skip doc updates.
3. Read the relevant section (see section mapping below).
4. If a change is needed, describe the edit to the user and get approval.
5. Use Edit to apply the incremental change to that section only.
6. Stage the modified doc with `git add`.

### Section mapping

**AGENTS.md**:

| Trigger | Section to edit |
| ------- | --------------- |
| File / directory add or remove | Repository Structure (tree diagram) |
| Significant file added | Key Files table |
| New top-level directory | Scopes table |

**CLAUDE.md**: Scopes list — only when a new scope candidate appears and CLAUDE.md needs to stay in sync with AGENTS.md.

**README.md**: Installation / Dependencies section — only when an external tool dependency is added.

### Discoverability principle

Don't put information that can be derived by reading code or files into the docs. Docs hold **purpose, deployment target, and relationships** only.

## Push (optional)

If the user explicitly asks to push as part of the request (`커밋하고 푸시해줘`, `commit and push`, ...):

1. **Run in the foreground** (so the SSH passphrase prompt actually reaches the user).
2. If submodules exist, push the submodule first, then the parent repo.
3. On push failure:
   - SSH-related error → suggest the user run `ssh-add`.
   - Other errors → relay the error message verbatim.
4. Do NOT push when push wasn't explicitly requested.

## Summary

After commit (and push, if any), output a concise summary:

```
커밋 완료:
- [submodule] <commit message> (push y/n)
- [parent] <commit message> (push y/n)
파일 N개 변경, +X/-Y줄
```

The summary block is shown to the user, so the labels stay in Korean.

## Prohibitions

- Do NOT add `Co-Authored-By` (the system handles this).
- Do NOT stage files without user confirmation.
- Do NOT modify docs inside a submodule.
- Do NOT create new doc files (incremental edits to existing docs only).
- Do NOT push unless explicitly requested.

## Gemma delegation (optional)

For very large changes (`git diff --cached --shortstat` ≥ 500 lines, ≥ 10 files changed, or the user gives a hint like `큰 diff` / `요약해서 커밋` / `gemma로 정리`), the body draft can be pre-summarized via local Gemma. The subject and the final body are still authored and reviewed by Claude.

Call pattern, fallback rules, and result usage follow `references/gemma-delegation.md`.
