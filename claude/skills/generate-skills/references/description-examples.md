# description 작성 예시 모음

> SKILL.md frontmatter의 `description` 필드 작성법과 예시.

---

## WHAT + WHEN 공식

description은 두 가지를 반드시 포함해야 한다:

1. **WHAT**: 이 스킬이 **무엇을** 하는가
2. **WHEN**: **어떤 상황/요청**에서 사용하는가

```
[WHAT 문장]. [WHEN 문장].
```

시스템은 description을 사용자 입력과 매칭하여 스킬을 자동 로드한다. description의 품질이 트리거 정확도를 결정한다.

---

## 좋은 예시

### 영어

```yaml
description: >-
  Analyzes Figma design files and generates developer handoff documentation.
  Use when user uploads .fig files, asks for "design specs",
  "component documentation", or "design-to-code handoff".
```

```yaml
description: >-
  Sets up a new Next.js project with TypeScript, ESLint, and testing
  configuration. Use when user asks to "create a project",
  "initialize Next.js", or "scaffold a new app".
```

```yaml
description: >-
  Runs TDD workflow: write failing test, implement, refactor.
  Use when implementing features, fixing bugs, or when user mentions
  "TDD", "test first", or "red-green-refactor".
```

### 한국어

```yaml
description: >-
  새로운 Claude 스킬을 폴더 구조, 프론트매터, 지시사항까지 단계적으로 생성한다.
  스킬 만들어줘, 새 스킬 추가, SKILL.md 작성, generate-skills 요청 시 사용한다.
```

```yaml
description: >-
  스프린트 계획, 태스크 생성, 상태 추적 등 Linear 프로젝트 워크플로우를 관리한다.
  "스프린트", "Linear 태스크", "프로젝트 계획", "티켓 생성" 언급 시 사용한다.
```

```yaml
description: >-
  CLAUDE.md 및 AGENTS.md 파일을 가이드 원칙에 따라 생성한다.
  프로젝트 분석, 인터뷰, 생성, 검증의 4단계 워크플로우를 수행한다.
```

---

## 나쁜 예시 → 수정 예시

### 너무 모호함

```yaml
# 나쁨: 무엇을 하는지, 언제 사용하는지 알 수 없음
description: Helps with projects.

# 수정: WHAT + WHEN 명시
description: >-
  Creates project scaffolding with recommended directory structure and configs.
  Use when starting a new project or asking to "set up a project".
```

### 트리거 없음

```yaml
# 나쁨: WHAT만 있고 WHEN이 없음
description: Creates sophisticated multi-page documentation systems.

# 수정: WHEN(트리거 문구) 추가
description: >-
  Creates multi-page documentation from source code and comments.
  Use when user asks for "generate docs", "API documentation",
  or "document this project".
```

### 기술적 내부 용어만 사용

```yaml
# 나쁨: 사용자가 쓸 법한 표현이 아님
description: Implements the Project entity model with hierarchical relationships.

# 수정: 사용자 관점의 표현으로 변경
description: >-
  Sets up project hierarchy with parent-child relationships and permissions.
  Use when user asks to "organize projects", "create project structure",
  or "set up project permissions".
```

### XML 태그 포함

```yaml
# 나쁨: XML 태그 사용 금지
description: Use for <important>project setup</important> tasks.

# 수정: 태그 제거
description: >-
  Handles project setup tasks including directory creation and config files.
  Use when user asks to "set up" or "initialize" a project.
```

---

## 길이 최적화 팁

- **최대**: 1024자 (초과 시 잘림)
- **권장**: 100~300자 (핵심만 전달)
- 불필요한 수식어 제거: "sophisticated", "comprehensive", "advanced" 등
- 구체적 트리거 문구를 우선 배치
- YAML 여러 줄 문법(`>-`) 사용 시 가독성 향상

---

## 트리거 튜닝 전략

### 과소 트리거 (스킬이 로드되지 않음)

증상:
- 사용자가 관련 요청을 해도 스킬이 자동 로드되지 않음
- 매번 수동으로 `/skill-name`을 입력해야 함

해결:
- description에 사용자가 **실제로 말할 법한 표현** 추가
- 동의어, 줄임말, 구어체 표현 포함
- 예: "PR 만들어줘", "풀리퀘 생성", "pull request"

### 과잉 트리거 (무관한 요청에도 로드됨)

증상:
- 관련 없는 작업에도 스킬이 로드됨
- 사용자가 스킬을 비활성화함

해결:
- description에 부정 트리거 추가: "Do NOT use for simple data exploration"
- 범위를 더 구체적으로 제한
- 지나치게 일반적인 단어 제거 (예: "help", "manage")
