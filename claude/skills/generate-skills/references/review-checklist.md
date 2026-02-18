# 스킬 검토 체크리스트

> Review 모드에서 참조하는 상세 검증 기준.

---

## 구조 요구사항

### 폴더 및 파일명

- 폴더명: `kebab-case` (예: `my-skill`, `notion-project-setup`)
- 파일명: 반드시 `SKILL.md` (대소문자 구분, 변형 불가)
- `README.md` 포함 금지 (스킬 폴더 내부)

### YAML Frontmatter

필수 필드:

~~~yaml
---
name: skill-name-in-kebab-case
description: 무엇을 한다. 언제 사용한다.
---
~~~

선택 필드:

~~~yaml
license: MIT
metadata:
  author: 작성자
  version: 1.0.0
  mcp-server: 서버명
~~~

---

## description 작성 기준

### 반드시 포함해야 하는 것

1. **WHAT**: 이 스킬이 무엇을 하는가
2. **WHEN**: 어떤 상황/요청에서 사용하는가 (트리거 문구 포함)

### 좋은 예시

~~~
Analyzes Figma design files and generates developer handoff documentation.
Use when user uploads .fig files, asks for "design specs", "component documentation",
or "design-to-code handoff".
~~~

~~~
스프린트 계획, 태스크 생성, 상태 추적 등 Linear 프로젝트 워크플로우를 관리한다.
"스프린트", "Linear 태스크", "프로젝트 계획", "티켓 생성" 언급 시 사용한다.
~~~

### 나쁜 예시

~~~
# 너무 모호함
Helps with projects.

# 트리거 없음
Creates sophisticated multi-page documentation systems.

# 기술적 내부 용어만 있음
Implements the Project entity model with hierarchical relationships.
~~~

### 제한 사항

- 1024자 이하
- XML 태그(`< >`) 사용 금지
- `claude`, `anthropic` 접두사 스킬명 금지

---

## 트리거 과잉/과소 대응

### 과소 트리거 (로드가 안 됨)

신호:
- 스킬이 자동으로 로드되지 않음
- 사용자가 수동으로 스킬을 호출해야 함

해결:
- description에 트리거 문구 추가
- 사용자가 실제로 말할 법한 표현으로 구체화

### 과잉 트리거 (무관한 요청에도 로드됨)

신호:
- 무관한 작업에 스킬이 로드됨
- 사용자가 스킬을 비활성화함

해결:
- 부정 트리거 추가: "Do NOT use for simple data exploration"
- description을 더 구체적으로 범위 제한

---

## 지시사항 품질 기준

### 구체성

좋은 예:
~~~
`python scripts/validate.py --input {filename}` 실행.
실패 시 일반 원인:
- 필수 필드 누락 (CSV에 추가 필요)
- 날짜 형식 오류 (YYYY-MM-DD 사용)
~~~

나쁜 예:
~~~
진행 전에 데이터를 검증하세요.
~~~

### 에러 처리 포함 여부

~~~markdown
## 에러 처리

| 오류 | 원인 | 해결 |
|------|------|------|
| Connection refused | MCP 서버 미실행 | Settings > Extensions 확인 |
| Invalid API key | 키 만료 또는 권한 없음 | 키 재발급 후 재연결 |
~~~

### 크기 제한

- SKILL.md 본문: 5,000단어 이하 권장
- 상세 문서는 `references/` 폴더로 분리
- 분리 구조 예시:

~~~
my-skill/
├── SKILL.md
└── references/
    ├── api-guide.md
    └── examples/
~~~

---

## 최종 검증 체크리스트

작성 전:
- [ ] 2-3개 구체적 사용 사례 정의
- [ ] 필요 도구 파악 (내장 또는 MCP)

작성 중:
- [ ] 폴더명 kebab-case
- [ ] SKILL.md 정확한 파일명
- [ ] YAML frontmatter `---` 구분자
- [ ] `name`: kebab-case, 공백/대문자 없음
- [ ] `description`: WHAT + WHEN 포함
- [ ] XML 태그 없음
- [ ] 지시사항 구체적이고 실행 가능
- [ ] 에러 처리 포함
- [ ] 예시 포함

업로드 전:
- [ ] 명백한 요청에 트리거 확인
- [ ] 무관한 요청에 비트리거 확인
- [ ] 기능 테스트 통과
- [ ] zip 압축 완료
