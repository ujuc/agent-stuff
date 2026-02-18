---
name: commit
description: "한국어 Conventional Commits 규칙에 따라 git 커밋을 생성한다. /commit, 커밋해줘, commit, 변경사항 커밋 요청 시 사용한다."
model: sonnet
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git log:*), Bash(git add:*), Bash(git commit:*)
---

# Git Commit

프로젝트의 한국어 Conventional Commits 규칙에 따라 커밋을 생성한다.

## 형식

`<type>(<scope>): <한국어 제목 -하다>`

- **type** (필수): feat, fix, docs, style, refactor, test, chore
- **scope** (선택): 프로젝트 CLAUDE.md에 정의된 scope를 따른다
- **제목** (필수): 한국어, `-하다` 종결 어미, 50자 이내, 마침표 없음
- **본문** (선택): 변경 의도가 불명확할 때만 포함한다. why > what > how 우선순위, 72자 줄바꿈

## 절차

1. 사용자 인자에서 파일 경로나 지시사항을 확인한다
2. `git status`와 `git diff`로 변경사항을 파악한다
3. `git log --oneline -20`으로 최근 커밋 스타일과 scope를 확인한다
4. 스테이징할 파일이 불명확하면 사용자에게 확인한다
5. 해당 파일만 `git add`로 스테이징한다
6. heredoc으로 커밋한다:

```bash
git commit -m "$(cat <<'EOF'
<type>(<scope>): <한국어 제목>

<본문 — 필요한 경우만>
EOF
)"
```

## 금지 사항

- Co-Authored-By를 추가하지 않는다 (시스템이 자동 처리)
- `git push`를 실행하지 않는다
- 사용자 확인 없이 파일을 스테이징하지 않는다

## 참고

커밋 메시지 상세 규칙은 references/gitmessage.md를 따른다.
