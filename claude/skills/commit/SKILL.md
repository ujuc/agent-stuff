---
name: commit
description: "한국어 Conventional Commits 규칙에 따라 git 커밋을 생성한다. 서브모듈 변경 감지·우선 커밋, 문서 자동 업데이트, push, 요약까지 포함. /commit, 커밋해줘, commit, 변경사항 커밋, 커밋하고 푸시해줘 요청 시 사용한다."
model: sonnet
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git log:*), Bash(git add:*), Bash(git commit:*), Bash(git push:*), Bash(git -C:*), Bash(git submodule:*), Bash(bash:*), Read, Edit, Glob
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

### 0. 서브모듈 변경 감지

1. `git status`로 서브모듈 변경 여부를 확인한다 (modified content, new commits)
2. 변경된 서브모듈이 있으면 **서브모듈 내부를 먼저** 처리한다:
   - `git -C <submodule> status`와 `git -C <submodule> diff`로 변경 내용 파악
   - 서브모듈 내부에서 스테이징 → 커밋 (아래 1~7단계와 동일한 규칙 적용)
   - 사용자가 push를 요청한 경우 서브모듈부터 push
3. 서브모듈 처리가 끝나면 부모 저장소로 돌아와 서브모듈 포인터를 포함하여 진행한다

### 1~7. 부모 저장소 커밋

1. 사용자 인자에서 파일 경로나 지시사항을 확인한다
2. `git status`와 `git diff`로 변경사항을 파악한다
3. `git log --oneline -20`으로 최근 커밋 스타일과 scope를 확인한다
4. 스테이징할 파일이 불명확하면 사용자에게 확인한다
5. 해당 파일만 `git add`로 스테이징한다
6. 구조적 변경이 감지되면 문서 증분 업데이트를 수행한다 (아래 "문서 업데이트" 참조)
7. heredoc으로 커밋한다:

```bash
git commit -m "$(cat <<'EOF'
<type>(<scope>): <한국어 제목>

<본문 — 필요한 경우만>
EOF
)"
```

## 문서 업데이트

스테이징 완료 후, 아래 조건에 따라 프로젝트 문서를 증분 수정한다.

### 트리거 조건 (하나라도 해당하면 수행)

1. 파일/디렉토리 추가 또는 삭제
2. 새 scope 후보 등장 (새 최상위 디렉토리)
3. 외부 도구 의존성 추가

### 적용 제외 (수행하지 않음)

- 기존 파일 내용만 변경 (구조 변경 없음)
- 서브모듈 포인터 업데이트
- style, refactor 타입의 내부 변경
- 프로젝트 루트에 AGENTS.md/CLAUDE.md가 없는 경우

### 수행 절차

1. Glob으로 프로젝트 루트에 AGENTS.md, CLAUDE.md 존재를 확인한다
2. 없으면 문서 업데이트를 건너뛴다
3. Read로 해당 문서의 관련 섹션을 확인한다 (아래 섹션 매핑 참조)
4. 변경이 필요하면 사용자에게 수정 내용을 설명하고 승인을 받는다
5. Edit로 해당 섹션만 증분 수정한다
6. 수정된 문서를 `git add`로 스테이징한다

### 섹션 매핑

**AGENTS.md**:

| 트리거 | 수정 대상 섹션 |
| ------ | -------------- |
| 파일/디렉토리 추가·삭제 | Repository Structure (트리 다이어그램) |
| 주요 파일 추가 | Key Files 테이블 |
| 새 최상위 디렉토리 추가 | Scopes 테이블 |

**CLAUDE.md**: 새 scope 후보 시 Scopes 목록 (AGENTS.md와 동기화 필요 시만)

**README.md**: 외부 도구 의존성 추가 시 설치/의존성 섹션만

### 발견 가능성 원칙

코드나 파일을 읽으면 알 수 있는 정보는 문서에 넣지 않는다. 문서에는 **목적, 배포 대상, 관계**만 기술한다.

## Push (선택)

사용자가 push를 함께 요청한 경우 (`커밋하고 푸시해줘`, `commit and push` 등):

1. **반드시 foreground에서 실행**한다 (SSH passphrase 프롬프트 대응)
2. 서브모듈이 있으면 서브모듈 push를 먼저 완료한 뒤 부모 저장소를 push한다
3. push 실패 시:
   - SSH 관련 에러 → `ssh-add` 실행을 사용자에게 제안한다
   - 기타 에러 → 에러 메시지를 그대로 전달한다
4. push를 명시적으로 요청하지 않았으면 push하지 않는다

## 요약

커밋(및 push) 완료 후 간결한 요약을 출력한다:

```
커밋 완료:
- [서브모듈명] <commit message> (push 여부)
- [부모 저장소] <commit message> (push 여부)
파일 N개 변경, +X/-Y줄
```

## 금지 사항

- Co-Authored-By를 추가하지 않는다 (시스템이 자동 처리)
- 사용자 확인 없이 파일을 스테이징하지 않는다
- 서브모듈 내부의 문서를 수정하지 않는다
- 새 문서 파일을 생성하지 않는다 (기존 문서의 증분 수정만 수행)
- push를 명시적으로 요청받지 않았으면 push하지 않는다

## Gemma 위임 (선택)

매우 큰 변경에서는 커밋 **본문** 작성 전에 로컬 Gemma로 diff를 1차 요약할 수 있다. 제목(`<type>(<scope>): <한국어 제목>`)은 여전히 Claude가 작성한다. Gemma는 본문의 사실 나열을 가볍게 돕는 역할만 한다. Gemma 호출 규약·폴백 규칙·결과 표시 원칙은 `../gemma/references/delegation-guide.md`를 따른다.

### 트리거 조건

다음 조건 중 하나 이상이면 위임을 고려한다:

- `git diff --cached --shortstat`가 500줄 이상 변경을 보고
- 변경 파일 수가 10개 이상
- 사용자가 `큰 diff`, `요약해서 커밋`, `gemma로 정리` 같은 힌트를 제공

작은 변경에서는 Claude가 직접 본문을 작성하는 편이 더 빠르고 정확하므로 이 단계를 건너뛴다.

### 호출 방식

스테이징 완료 후, commit 명령(7단계) 전에 한 번 호출한다. 아래 패턴은 두 가지 흔한 함정을 미리 피한다:

- **Quoted heredoc 함정**: `<<'EOF'`는 내부의 `$()` command substitution을 literal로 취급한다. 따라서 `$(git diff --cached)`를 quoted heredoc 안에 넣으면 실제 diff가 아니라 문자열 `$(git diff --cached)` 가 gemma에 전달된다. diff를 먼저 변수로 캡처하고 double-quoted string으로 보간한다.
- **zsh noclobber 함정**: `set -o noclobber`가 켜진 zsh에서 고정 경로로 `2>/tmp/foo.log`를 반복하면 두 번째 호출부터 `file exists`로 실패한다. PID 기반 이름 + `rm -f` 선행으로 회피한다.

```bash
# 로그 파일 — zsh noclobber 회피
LOG=/tmp/gemma-commit-$$.log
rm -f "$LOG"

# 스테이지된 diff를 변수로 먼저 캡처 (quoted heredoc 함정 회피)
DIFF=$(git diff --cached)

# 프롬프트 조립 — double-quoted string 안의 $DIFF 는 값으로 보간되지만
# diff 내부의 $·백틱 등은 재해석되지 않는다
PROMPT="다음 git diff를 5개 이하의 글머리 기호로 요약해줘. 각 변경의 *의도*에 집중하고, 코드 인용은 하지 마. 한국어로 출력해.

---
$DIFF"

# 호출 + 조용한 폴백
gemma_summary=$(GEMMA_TIMEOUT=300 bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "$PROMPT" 2>"$LOG") || gemma_summary=""
```

- stdout은 `$gemma_summary`로 캡처, stderr는 로그 파일로 분리
- `|| gemma_summary=""`가 폴백 트리거 — query.sh가 non-zero 종료 시 빈 문자열
- 큰 diff는 기본 120초를 초과할 수 있으므로 `GEMMA_TIMEOUT=300` 같은 값으로 여유를 준다

### 폴백 규칙

**Gemma 가용성은 전제가 아니다.** Ollama가 꺼져 있거나 gemma 모델이 설치돼 있지 않거나 타임아웃이 나도 commit 절차 자체는 정상 진행돼야 한다:

1. `$gemma_summary`가 비어 있으면 gemma 단계를 건너뛰고 Claude가 직접 본문을 작성한다
2. 사용자에게는 세션당 한 번만 간단히 알린다 (예: `note: gemma 사전 요약을 건너뛰었습니다 — Ollama 미가동`)
3. 이 때문에 커밋이 실패하면 안 된다 — gemma 실패는 에러가 아니라 **정상 경로**다

### 결과 사용

1. `$gemma_summary`를 얻었어도 그대로 본문에 붙여넣지 말고 Claude가 읽고 **정확성을 검토**한다
2. 커밋 본문 작성 시 참고용 초안으로만 사용 — 최종 본문은 Claude가 책임진다
3. Conventional Commits 형식, 한국어 `-하다` 종결 규칙은 여전히 Claude가 적용한다
4. 사용자에게 커밋 diff를 설명할 때, gemma가 요약한 부분은 라벨을 붙여 구분한다 (예: `gemma 사전 요약에 따르면: ...`). 제목과 최종 커밋 메시지는 Claude의 voice로 유지한다

## 참고

커밋 메시지 상세 규칙은 references/gitmessage.md를 따른다.
