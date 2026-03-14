---
name: commit
description: "한국어 Conventional Commits 규칙에 따라 git 커밋을 생성한다. /commit, 커밋해줘, commit, 변경사항 커밋 요청 시 사용한다."
model: sonnet
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git log:*), Bash(git add:*), Bash(git commit:*), Read, Edit, Glob
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

## 금지 사항

- Co-Authored-By를 추가하지 않는다 (시스템이 자동 처리)
- `git push`를 실행하지 않는다
- 사용자 확인 없이 파일을 스테이징하지 않는다
- 서브모듈 내부의 문서를 수정하지 않는다
- 새 문서 파일을 생성하지 않는다 (기존 문서의 증분 수정만 수행)

## 참고

커밋 메시지 상세 규칙은 references/gitmessage.md를 따른다.
