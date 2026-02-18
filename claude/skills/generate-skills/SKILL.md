---
name: generate-skills
description: 새로운 Claude 스킬을 폴더 구조, 프론트매터, 지시사항까지 단계적으로 생성한다.
  스킬 만들어줘, 새 스킬 추가, SKILL.md 작성, generate-skills 요청 시 사용한다.
model: opus
disable-model-invocation: true
---

# 스킬 생성 워크플로우

새 Claude 스킬을 5단계로 생성한다. 각 단계를 순서대로 수행하며, 이전 단계가 완료되어야 다음 단계로 진행한다.

$ARGUMENTS 가 주어지면 해당 경로/이름을 대상으로 한다. 없으면 사용자에게 확인한다.

---

## 1단계: 사용 사례 파악

사용자에게 다음 정보를 AskUserQuestion으로 수집한다:

1. **문제/시나리오**: 이 스킬이 해결하려는 구체적 문제는 무엇인가?
2. **대상 도구**: 어떤 도구(내장 도구, MCP 서버, CLI)를 사용하는가?
3. **기대 출력**: 스킬 실행 결과물은 무엇인가? (파일, 메시지, 코드 등)
4. **트리거 상황**: 사용자가 어떤 말을 할 때 이 스킬이 로드되어야 하는가?

수집한 정보를 기반으로 references/patterns.md에서 적합한 패턴을 선택한다:

| 패턴 | 적합한 상황 |
|------|------------|
| 선형 워크플로우 | 정해진 순서대로 실행 |
| 인터뷰 기반 | 요구사항이 유동적 |
| 도구 오케스트레이션 | 여러 도구 조합 |
| 템플릿 채우기 | 정형화된 출력물 |
| 검증·리뷰 | 품질 점검 |

선택한 패턴과 근거를 사용자에게 확인한다.

---

## 2단계: 구조 생성

references/skill-structure.md를 참조하여 스킬 디렉토리를 생성한다.

### 필수 작업

1. 스킬 폴더 생성 (kebab-case)
2. `SKILL.md` 파일 생성 (빈 파일 — 3~4단계에서 채움)

### 선택 작업 (1단계 결과에 따라)

3. `references/` 폴더 + 참조 문서 생성
4. `scripts/` 폴더 + 유틸리티 스크립트 생성
5. `assets/` 폴더 생성

### 확인 사항

- 폴더명이 kebab-case인가
- `README.md`를 생성하지 않았는가
- 폴더명이 `claude` 또는 `anthropic`으로 시작하지 않는가

---

## 3단계: 프론트매터 작성

references/frontmatter-spec.md와 references/description-examples.md를 참조하여 YAML frontmatter를 작성한다.

### 작성 절차

1. `name` 작성: 폴더명과 동일, kebab-case
2. `description` 작성: WHAT + WHEN 공식 적용
   - WHAT: 스킬이 무엇을 하는가 (1단계의 문제/시나리오 기반)
   - WHEN: 어떤 상황에서 사용하는가 (1단계의 트리거 상황 기반)
3. 선택 필드 결정:
   - `model`: 복잡한 워크플로우면 `opus`, 단순하면 생략
   - `disable-model-invocation`: 파괴적/비용 높은 스킬이면 `true`

### 자기 검증

- `name`이 kebab-case 정규식 `^[a-z0-9]+(-[a-z0-9]+)*$`에 매칭하는가
- `description`이 1024자 이하인가
- `description`에 XML 태그가 없는가
- `description`에 WHAT과 WHEN이 모두 포함되어 있는가

---

## 4단계: 지시사항 작성

1단계에서 선택한 패턴 구조를 따라 SKILL.md 본문을 작성한다.

### 공통 규칙

- **구체적으로**: 실행 가능한 명령, 정확한 경로, 구체적 기준 포함
- **에러 처리**: 실패 시나리오와 대응 방법 포함
- **예시 포함**: 각 단계의 입력/출력 예시 제공
- **도구 명시**: 사용하는 도구(Read, Write, Bash, AskUserQuestion 등) 명시

### 크기 제한

- SKILL.md 본문: 5,000단어 이하 권장
- 초과 시 상세 내용을 `references/`로 분리하고 상대 경로로 참조

### 작성 후 확인

- 모든 references/ 경로가 실제 파일과 일치하는가
- 지시사항이 검증 가능한 형태인가 (모호한 표현 없이)
- 불필요한 내용이 없는가 (린터 역할, 추측, 과도한 설명)

---

## 5단계: 검증

### 자동 검증

scripts/validate-skill.sh를 실행한다:

```bash
bash agents/claude/skills/generate-skills/scripts/validate-skill.sh <스킬-디렉토리-경로>
```

실패 항목이 있으면 해당 단계로 돌아가 수정한 후 재실행한다.

### 수동 검증

references/review-checklist.md의 최종 검증 체크리스트를 적용한다:

**작성 전 확인:**
- [ ] 2-3개 구체적 사용 사례가 정의되었는가
- [ ] 필요 도구가 파악되었는가

**작성 중 확인:**
- [ ] YAML frontmatter가 올바른가 (자동 검증으로 확인)
- [ ] 지시사항이 구체적이고 실행 가능한가
- [ ] 에러 처리가 포함되었는가
- [ ] 예시가 포함되었는가

**작성 후 확인:**
- [ ] 명백한 요청에 트리거가 작동하는가 (description 검토)
- [ ] 무관한 요청에 트리거가 작동하지 않는가 (과잉 트리거 검토)

### 등록

검증 통과 후, 스킬을 등록할 위치의 CLAUDE.md Skills 테이블에 행을 추가한다:

```markdown
| `skill-name` | 트리거 문구들 | model |
```
