---
name: generate-skills
description: "Claude 스킬을 생성하거나 기존 스킬을 최신 spec에 맞게 업데이트한다. 스킬 만들어줘, 새 스킬 추가, 스킬 업데이트, 스킬 수정, generate-skills 요청 시 사용한다."
model: opus
disable-model-invocation: true
argument-hint: "[skill-name]"
---

# 스킬 생성/업데이트 워크플로우

## 모드 판별

$ARGUMENTS를 분석하여 모드를 결정한다:

- **업데이트 모드**: $ARGUMENTS에 "업데이트", "수정", "update" 키워드 포함 시
  → 0단계 → U1~U3단계 → 5단계(검증) 순서로 진행
- **생성 모드**: 그 외
  → 0단계 → 1~5단계 순서로 진행

$ARGUMENTS가 없으면 사용자에게 모드와 대상을 확인한다.

---

## 설계 원칙

스킬 생성 전체를 관통하는 원칙. 상세 내용은 references/design-principles.md를 참조한다.

1. **간결함이 핵심**: 컨텍스트 윈도우는 공공재. Claude가 이미 아는 것은 포함하지 않는다.
2. **자유도 조절**: 작업의 취약성에 맞춰 지시 수준을 결정한다 (높음/중간/낮음).
3. **Progressive Disclosure**: 정보를 3단계로 계층 분리한다 (메타데이터 → 본문 → 번들 리소스).
4. **서브에이전트 활용**: 컨텍스트 보호와 병렬 실행이 가능한 지점에서 서브에이전트를 적극 활용한다. 상세 기준은 references/subagent-guidelines.md를 참조한다.

---

## 0단계: 문서 검증 (실행 전)

공식 Skills 문서가 자주 변경되므로, 스킬 생성 전에 최신 상태를 확인한다.

### 절차

1. WebFetch로 https://code.claude.com/docs/en/skills 페이지를 확인한다
2. Frontmatter reference 섹션의 필드 목록을 추출한다
3. references/frontmatter-spec.md의 "Field Reference" 섹션과 비교한다
4. **변경 감지 시**: references/frontmatter-spec.md를 업데이트한 후 1단계로 진행한다
5. **변경 없음**: 바로 1단계로 진행한다

### 비교 대상

- 필드 추가/제거/변경 여부
- String substitutions 변경 여부
- Invocation control matrix 변경 여부

### 주의

- WebFetch 실패 시 (네트워크 오류 등) 기존 references/frontmatter-spec.md를 그대로 사용하고 1단계로 진행한다
- 변경이 대규모인 경우, 사용자에게 변경 요약을 보여주고 확인을 받은 후 업데이트한다

---

## 1단계: 사용 사례 파악

사용자에게 다음 정보를 AskUserQuestion으로 수집한다:

1. **문제/시나리오**: 이 스킬이 해결하려는 구체적 문제는 무엇인가?
2. **대상 도구**: 어떤 도구(내장 도구, MCP 서버, CLI)를 사용하는가?
3. **기대 출력**: 스킬 실행 결과물은 무엇인가? (파일, 메시지, 코드 등)
4. **트리거 상황**: 사용자가 어떤 말을 할 때 이 스킬이 로드되어야 하는가?

수집한 정보를 기반으로 먼저 references/skill-types.md에서 스킬의 도메인 유형을 파악한 후, references/patterns.md에서 적합한 구조 패턴을 선택한다:

| 패턴 | 적합한 상황 | 자유도 |
|------|------------|--------|
| 선형 워크플로우 | 정해진 순서대로 실행 | 낮음~중간 |
| 인터뷰 기반 | 요구사항이 유동적 | 높음 |
| 도구 오케스트레이션 | 여러 도구 조합 | 중간 |
| 템플릿 채우기 | 정형화된 출력물 | 낮음 |
| 검증·리뷰 | 품질 점검 | 중간 |

선택한 패턴과 근거를 사용자에게 확인한다.

### 병렬 탐색 (선택)

AskUserQuestion 발송 후, 사용자 응답 대기 중에 Explore 에이전트를 스폰하여 기존 스킬을 조사한다.
상세 지시사항은 references/subagent-guidelines.md의 "Explore-1"을 따른다.

$ARGUMENTS로 충분한 정보가 제공된 경우 건너뛴다.

---

## 2단계: 구조 생성

references/skill-structure.md를 참조하여 스킬 디렉토리를 생성한다.

### 자동 초기화 (권장)

scripts/init-skill.py를 실행하여 템플릿 구조를 생성한다:

```bash
python3 agents/claude/skills/generate-skills/scripts/init-skill.py <스킬이름> --path <대상경로>
```

기본은 `SKILL.md`만 생성한다. Tier 3 리소스가 필요하면 `--with-references`, `--with-scripts`, `--with-assets` 플래그를 추가한다. 이후 3~4단계에서 내용을 채운다.

### 수동 생성 (init-skill.py 사용 불가 시)

**필수 작업:**

1. 스킬 폴더 생성 (kebab-case)
2. `SKILL.md` 파일 생성 (빈 파일 — 3~4단계에서 채움)

**선택 작업 (1단계 결과에 따라):**

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

1. `name` 작성 (권장): 폴더명과 동일, kebab-case. 생략 시 디렉토리명을 사용한다.
2. `description` 작성 (권장): WHAT + WHEN 공식 적용
   - WHAT: 스킬이 무엇을 하는가 (1단계의 문제/시나리오 기반)
   - WHEN: 어떤 상황에서 사용하는가 (1단계의 트리거 상황 기반)
   - 생략 시 마크다운 본문의 첫 문단을 사용한다.
3. 선택 필드 결정 (카테고리별):

   **호출 제어:**
   - `disable-model-invocation`: 파괴적/비용 높은 스킬이면 `true`
   - `user-invocable`: 배경 지식용 스킬이면 `false` (사용자 `/` 메뉴에서 숨김)

   **실행 환경:**
   - `model`: 복잡한 워크플로우면 `opus`, 단순하면 생략
   - `effort`: 세션 기본값과 다른 수준이 필요하면 지정 (`low`, `medium`, `high`, `max`)
   - `context`: 격리된 서브에이전트에서 실행하려면 `fork`
   - `agent`: `context: fork` 설정 시 사용할 에이전트 타입 (`Explore`, `Plan`, `general-purpose` 등)

   **도구/권한:**
   - `allowed-tools`: 스킬 실행 중 허가 없이 사용할 도구 목록

   **기타:**
   - `argument-hint`: 자동완성 시 표시할 인자 힌트 (예: `[issue-number]`)
   - `hooks`: 스킬 라이프사이클에 연결할 훅

기계적 검증(kebab-case, 길이, 예약 접두사 등)은 5단계의 `validate-skill.sh`에서 자동 처리한다. 여기서는 의미 검증만 한다: description에 WHAT과 WHEN이 모두 포함되었는가.

---

## 4단계: 지시사항 작성

1단계에서 선택한 패턴 구조를 따라 SKILL.md 본문을 작성한다.

### 참조 스킬 분석 (선택)

1단계에서 유사 패턴 스킬이 식별된 경우, Explore 에이전트를 스폰하여 해당 스킬 구조를 심층 분석한다.
본문 초안 작성과 병렬로 실행하며, 결과를 초안에 반영한다.
상세 지시사항은 references/subagent-guidelines.md의 "Explore-2"를 따른다.

### 공통 규칙

- **구체적으로**: 실행 가능한 명령, 정확한 경로, 구체적 기준 포함
- **에러 처리**: 실패 시나리오와 대응 방법 포함
- **예시 포함**: 각 단계의 입력/출력 예시 제공
- **도구 명시**: 사용하는 도구(Read, Write, Bash, AskUserQuestion 등) 명시
- **Gotchas 섹션 포함 권장**: 알려진 실패 포인트를 Gotchas 섹션으로 구축한다. 스킬에서 가장 높은 가치의 콘텐츠다. references/design-principles.md의 원칙 4 참조.

### 출력 패턴 선택

생성할 스킬의 출력 형식이 중요한 경우, references/output-patterns.md를 참조한다:

- **Template Pattern**: 엄격한 출력 형식이 필요할 때
- **Examples Pattern**: 입력/출력 쌍으로 품질 기준을 보여줄 때

### 자유도 적용

references/design-principles.md의 자유도 가이드에 따라 지시사항의 구체성 수준을 결정한다.

### 크기 제한

- SKILL.md 본문: 5,000단어 이하 권장
- 초과 시 상세 내용을 `references/`로 분리하고 상대 경로로 참조

### 작성 후 확인

- 모든 references/ 경로가 실제 파일과 일치하는가
- 지시사항이 검증 가능한 형태인가 (모호한 표현 없이)
- 불필요한 내용이 없는가 (린터 역할, 추측, 과도한 설명)

---

## U1단계: 대상 스킬 분석 (업데이트 모드)

1. $ARGUMENTS에서 대상 스킬 경로/이름을 추출한다
2. 대상 스킬의 SKILL.md를 Read로 읽는다
3. frontmatter 필드를 파싱한다 (name, description, 선택 필드)
4. references/, scripts/ 존재 여부를 확인한다
5. SKILL.md 본문 줄 수를 확인한다

대상을 특정할 수 없으면 AskUserQuestion으로 확인한다.

---

## U2단계: 최신 spec과 비교 (업데이트 모드)

0단계에서 확인한 최신 references/frontmatter-spec.md 기준으로 비교한다:

1. **누락 권장 필드**: name, description이 없으면 알림
2. **제거된 필드**: 공식 문서에 없는 필드 (예: license, metadata) 감지
3. **새 필드 활용 가능성**: context, agent, effort, allowed-tools 등 유용할 수 있는 필드 제안
4. **description 품질**: WHAT + WHEN 포함 여부, 트리거 문구 적절성
5. **구조 점검**: SKILL.md 줄 수 (500줄 한도), references/ 분리 필요성

비교 결과를 요약하여 사용자에게 보여주고, 업데이트 범위를 확인받는다.

---

## U3단계: 업데이트 적용 (업데이트 모드)

사용자가 승인한 범위에 따라 Edit 도구로 수정한다:

1. frontmatter 필드 업데이트 (추가/수정/제거)
2. description 개선 (필요 시)
3. 본문 구조 조정 (필요 시)

각 변경 전 변경 내용을 사용자에게 보여주고 확인받는다.
완료 후 5단계(검증)로 진행한다.

---

## 5단계: 검증

### 자동 검증

scripts/validate-skill.sh를 실행한다:

```bash
bash agents/claude/skills/generate-skills/scripts/validate-skill.sh <스킬-디렉토리-경로>
```

실패 항목이 있으면 해당 단계로 돌아가 수정한 후 재실행한다.

### 행동 검증 (선택)

스킬의 출력 품질을 측정할 이진(binary) eval 기준을 정의한다.
references/eval-guide.md를 참조하여 3-6개의 yes/no 체크를 SKILL.md 하단 `## Eval Criteria` 섹션 또는 별도 `evals.md`에 기록한다.
이 기준은 이후 autoresearch 스킬로 자율 최적화할 때 재활용된다.

### 독립 검증 (선택)

생성된 스킬이 references/ 또는 scripts/를 포함하는 경우, general-purpose 에이전트를 스폰하여 맹검 검증을 수행한다.
상세 지시사항은 references/subagent-guidelines.md의 "Reviewer"를 따른다.

스킬이 최소 구조(SKILL.md만)이거나, 사용자가 빠른 생성을 요청한 경우 건너뛴다.

### 트리거 검증 (자동 검증으로 잡을 수 없음)

자동 검증은 형식만 본다. 트리거 정확도는 사람이 본다.

- 사용자가 실제로 쓸 법한 표현이 description에 들어 있는가 (과소 트리거 방지)
- description이 너무 일반적인 단어("help", "manage")로 시작해 무관한 요청에도 매칭될 위험은 없는가 (과잉 트리거 방지)

판단이 필요하면 references/review-checklist.md의 트리거 튜닝 가이드를 참조한다.

### 등록

검증 통과 후, 스킬을 등록할 위치의 CLAUDE.md Skills 테이블에 행을 추가한다:

```markdown
| `skill-name` | 트리거 문구들 | model |
```

### 배포 (선택)

팀 배포가 필요한 경우 references/distribution-guide.md를 참조한다. 레포 체크인 vs 플러그인 마켓플레이스 선택, 스킬 간 조합, 사용량 측정 방법을 안내한다.
