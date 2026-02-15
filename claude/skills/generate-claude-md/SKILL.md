---
name: generate-claude-md
description: CLAUDE.md 및 AGENTS.md 파일을 가이드 원칙에 따라 생성한다. 프로젝트 분석, 인터뷰, 생성, 검증의 4단계 워크플로우를 수행한다.
model: opus
disable-model-invocation: true
---

# CLAUDE.md 및 AGENTS.md 생성 스킬

$ARGUMENTS 가 주어지면 해당 프로젝트 경로를 대상으로 한다. 없으면 현재 작업 디렉토리를 대상으로 한다.

## 생성 철학

이 스킬 전체를 관통하는 4가지 원칙이다 (references/karpathy-guidelines.md 참조).

- **생각 후 행동**: 가정하지 말고 확인한다. 모호함을 감추지 말고 드러낸다.
- **단순함 우선**: 문제를 해결하는 최소한의 내용만 포함한다. 추측을 배제한다.
- **외과적 정밀함**: 프로젝트에서 관찰된 사실만 포함한다. 린터가 할 일은 린터에게 맡긴다.
- **목표 기반 실행**: 모든 지시사항은 검증 가능해야 한다.

아래 4단계를 순서대로 수행한다.

---

## 1단계: 프로젝트 분석 — Think Before Coding

대상 디렉토리에서 다음을 자동 탐지한다:

- 패키지/빌드/테스트/린트·포매터 설정 (package.json, Cargo.toml, pyproject.toml, go.mod, .eslintrc, .prettierrc, biome.json, ruff.toml 등)
- 저장소 구조 및 독립성 (모노레포: workspaces, packages/, apps/; 서브모듈: .gitmodules 파싱으로 경로/URL/독립 저장소 여부; 하위 디렉토리의 자체 패키지 매니저 존재 여부)
- 문서/CI 구성 (기존 CLAUDE.md, AGENTS.md, contributing-docs/, .cursorrules, 하위 CLAUDE.md 위치/내용, CI/CD: .github/workflows, .gitlab-ci.yml)

탐지 결과를 간략히 정리하여 사용자에게 보여준다. 이때:

- 탐지 결과에서 **사실**과 **가정**을 구분하여 표시한다.
- 자동 탐지로 알 수 없는 항목을 명시적으로 나열한다.
- 기존 AGENTS.md가 있으면 구조와 내용을 분석하여 3단계 생성 시 참고한다.
- 중첩 CLAUDE.md 후보 목록을 표로 정리한다:
  | 경로 | 유형 (모노레포 패키지/서브모듈) | 기존 CLAUDE.md 유무 | 생성 권장 여부 |

---

## 2단계: 인터뷰 — Think Before Coding

1단계 자동 탐지로 알 수 없는 항목만 질문한다. WHAT/WHY/HOW 프레임워크를 따른다.

질문은 다음 범위 내에서 필요한 것만 한다:

**WHY (자동 탐지 불가)**:
- 이 프로젝트의 목적/역할은 무엇인가?

**WHAT (자동 탐지로 부족한 부분만)**:
- 모노레포인 경우 각 패키지/앱의 역할
- 서브모듈이 있는 경우 부모 저장소와의 관계 및 독립 운영 여부
- 외부 서비스 의존성 (DB, 메시지 큐, 외부 API 등)

**HOW (자동 탐지로 부족한 부분만)**:
- 특별한 작업 규칙/워크플로우, 브랜치 전략, PR/커밋 규칙, 환경 설정 특이사항이 있는가?
- 중첩 CLAUDE.md 후보 목록을 보여주고, 각 후보에 대해 생성 여부를 확인한다

추가 원칙:
- 모호한 항목은 **가능한 해석을 제시한 뒤** 사용자에게 선택을 요청한다.
- 1단계에서 가정한 내용을 사용자에게 확인한다.

---

## 3단계: 생성 — Simplicity First + Surgical Changes

네 가지를 생성한다:

공통 작성 규칙:
- 코드 스니펫은 직접 포함하지 않고 file:line 참조만 사용한다

### A. 루트 CLAUDE.md

생성 원칙:

- 모든 세션, 모든 작업에 적용되는 보편적 내용만 포함한다
- 목표 줄 수: 100줄 이하. 절대 300줄을 넘기지 않는다
- 코드 스타일 규칙을 포함하지 않는다 (린터/포매터에 위임)
- AGENTS.md만 참조한다 (contributing-docs/를 직접 참조하지 않는다)
- 1~2단계에서 **확인된 사실만** 포함한다. 추측이나 '있으면 좋을' 항목을 배제한다
- 지시사항 추가 시 자문한다: **"이것 없이 Claude가 실수하는가?"**

구조:

```markdown
# 프로젝트 개요
(WHY: 1-2줄로 프로젝트 목적)

# 기술 스택
(WHAT: 핵심 기술만 나열)

# 개발 명령
(HOW: 빌드, 테스트, 린트 명령)

# 작업 규칙
(HOW: 브랜치, 커밋, PR 등 보편적 규칙)

# 행동 가이드라인
(프로젝트 고유 제약이 있는 경우만. 예: "DB 마이그레이션 시 반드시 확인 후 실행")

# 참조 문서
- **[AGENTS.md](./AGENTS.md)** — 전체 프로젝트 구조, 코드 규약, 상세 가이드
(중첩 CLAUDE.md가 있는 하위 디렉토리도 나열)
```

### B. AGENTS.md

agents.md 표준을 따르는 프로젝트 가이드. CLAUDE.md에서 참조하며, contributing-docs/의 상세 문서를 가리킨다.

생성 원칙:

- 모든 AI 에이전트가 활용할 수 있는 보편적 형식
- contributing-docs/ 참조로 점진적 공개 구현
- 기존 AGENTS.md가 있으면 참고하되 이 스킬 원칙에 맞게 재생성

구조:

- YAML frontmatter (name, description, version, standard)
- Project Overview: 프로젝트 목적, 기술 스택
- Repository Structure: 디렉토리 구조 및 핵심 파일
- Build & Test: 빌드, 테스트, 린트 명령
- Code Style: 코드 규약 (린터로 강제할 수 없는 것만)
- Git Workflow: 브랜치 전략, 커밋 규약
- Boundaries: Always Do / Ask First / Never Do
- Contributing Docs 참조 섹션: contributing-docs/ 내 상세 문서 목록

### C. contributing-docs/ 분리 문서

AGENTS.md에서 참조하는 상세 내용을 별도 문서로 생성한다. 다음 중 프로젝트에 해당하는 것만 생성한다:

- `contributing-docs/architecture.md`: 서비스 구조, 통신 패턴, 데이터 흐름
- `contributing-docs/building_the_project.md`: 상세 빌드/배포 절차
- `contributing-docs/testing.md`: 테스트 전략, 테스트 데이터 설정
- `contributing-docs/database.md`: 스키마 구조, 마이그레이션 방법
- `contributing-docs/conventions.md`: 코드 규약, 네이밍 규칙 (린터로 강제할 수 없는 것만)
- `contributing-docs/behavioral.md`: 프로젝트 고유 행동 제약 (해당하는 경우만)

각 분리 문서도 간결하게 작성하며, 공통 작성 규칙을 따른다.

### D. 중첩 CLAUDE.md (모노레포 패키지 / 서브모듈)

1단계에서 탐지된 후보 중, 2단계에서 사용자가 승인한 디렉토리에 대해 생성한다.

#### 생성 조건

다음 **모두**를 만족하는 디렉토리에만 생성한다:

- 자체 패키지 매니저 파일이 있거나 git 서브모듈이다
- 루트 CLAUDE.md와 다른 기술 스택, 빌드 명령, 또는 작업 규칙이 필요하다
- 사용자가 2단계에서 생성을 승인했다

#### 생성 원칙

Section A의 원칙을 상속하되, 추가로:

- **범위 한정**: 해당 디렉토리 안의 컨텍스트만 다룬다
- **중복 금지**: 상위 CLAUDE.md에 있는 내용을 반복하지 않는다. 차이점만 기술한다
- **상위 참조**: 공통 규칙은 상위 CLAUDE.md를 구체적 경로로 참조한다
- **목표 줄 수**: 50줄 이하. 절대 100줄을 넘기지 않는다
- **자기 완결적 제목**: `# CLAUDE.md — {패키지/서브모듈 이름}`으로 시작한다

#### 구조

```markdown
# CLAUDE.md — {이름}

(1줄: 이 디렉토리의 목적/역할)

## 기술 스택
(상위와 다른 부분만. 동일하면 섹션 생략)

## 개발 명령
(이 디렉토리 고유의 빌드/테스트/린트 명령)

## 작업 규칙
(상위와 다른 규칙이 있는 경우만. 없으면 생략)

## 참조 문서
- **[../CLAUDE.md](../CLAUDE.md)** — 프로젝트 공통 규칙
(하위에 AGENTS.md가 있으면 참조, 없으면 생략)
```

#### 참조 경로 규칙

- 상위 CLAUDE.md: 항상 상대 경로 (`../CLAUDE.md`)
- 서브모듈: 부모 저장소 CLAUDE.md를 URL 또는 상대 경로로 참조
- 형제 디렉토리: 직접 참조하지 않는다 (상위를 경유)

---

## 4단계: 검증 — Goal-Driven Execution

생성된 CLAUDE.md와 AGENTS.md에 대해 다음 체크리스트를 줄 단위로 적용한다:

1. **보편성/필수성/중복**: 모든 작업에 적용되는가? 없으면 실수하는가? 코드를 읽으면 자명한가? → 아니면 삭제 또는 AGENTS.md/contributing-docs/로 이동
2. **린터 역할**: 코드 스타일 규칙인가? → 삭제하고 린터/Hook으로 대체 권장
3. **추측 배제**: 1~2단계에서 확인되지 않은 내용이 포함되는가? → 있으면 삭제
4. **검증 가능성**: 각 지시사항을 따랐는지 판별 가능한가? → 아니면 구체화
5. **분량 제약**: 100줄 이하, 개별 지시사항 50개 이하인가? → 초과하면 통합 또는 삭제
6. **계층/범위**: CLAUDE.md가 contributing-docs/를 직접 참조하는가? AGENTS.md가 Claude 전용 내용만 담는가? 중첩 CLAUDE.md가 자기 디렉토리 밖을 포함하거나 상위 내용을 반복하는가? → 있으면 이동/삭제 또는 상위 참조로 대체
7. **참조 정합성**: 중첩 CLAUDE.md의 상위 참조 경로가 올바른가? → 상대 경로 검증

자기 테스트 질문:
- "시니어 엔지니어가 이 CLAUDE.md를 보고 '과하다'고 할까?"
- "CLAUDE.md → AGENTS.md → contributing-docs/ 계층이 명확하게 분리되어 있는가?"
- "중첩 CLAUDE.md를 제거해도 상위 CLAUDE.md만으로 해당 디렉토리 작업이 충분한가?" → 충분하면 중첩 파일 삭제 권장
- "중첩 CLAUDE.md 간에 내용이 서로 모순되는가?"

검증 결과를 사용자에게 보고한다. 탈락 항목이 있으면 해당 줄과 사유를 함께 표시하고, 정제된 최종본을 제시한다.

---

## LLM 컨텍스트 원칙

- LLM은 인컨텍스트 학습자다. 기존 코드 패턴을 검색하면 스타일을 자연스럽게 따라가므로, 스타일 규칙을 CLAUDE.md에 포함할 필요가 없다.
- Claude Code 시스템 프롬프트가 이미 ~50개 지시사항을 포함하며, 보편적이지 않은 내용이 많을수록 전체가 무시될 확률이 높아진다.
- 상위 레벨(CLAUDE.md)의 오류는 하위(계획 → 코드)로 기하급수적으로 증폭된다.
- LLM은 명확한 목표가 있을 때 반복 실행에 탁월하다 (references/karpathy-guidelines.md). 지시사항을 **검증 가능한 목표**로 작성해야 Claude가 자율적으로 올바른 결과에 도달한다.
