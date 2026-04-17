---
name: prompting-assist
description: "사용자가 LLM에 보낼 프롬프트를 개선·리뷰·피드백받고 싶어할 때 사용. Anthropic 공식 프롬프팅 모범 사례(semantics 참조 001)에 근거한 체크리스트로 진단하고 개선안을 제시한다. '프롬프트 개선해줘', '이 프롬프트 리뷰해줘', '프롬프팅 팁', '/prompting' 등 명시적 어구에만 발동하며, 일반 대화 속 '프롬프트'라는 단어만으로는 발동하지 않는다."
model: sonnet
allowed-tools: Read, Grep, Edit
---

# Prompting Assist — 프롬프트 개선 도우미

## Purpose

사용자가 LLM(특히 Claude 계열)에 보내려는 프롬프트를 Anthropic 공식 모범 사례에 비추어 진단하고 개선안을 제공한다. 이 스킬은 **프롬프트 작성·개선이 주제인 발화**에만 반응하며, 일반적인 대화 속에 프롬프트라는 단어가 등장한다고 해서 발동하지 않는다.

## Trigger Policy

**발동**:
- "프롬프트 개선해줘"
- "이 프롬프트 리뷰해줘" / "이 프롬프트 피드백 줘"
- "프롬프팅 팁 알려줘"
- `/prompting`
- "system prompt 개선해줘"

**발동하지 않음**:
- "프롬프트가 너무 길어서..." (단어 등장 ≠ 개선 요청)
- "프롬프트 엔지니어링이 뭐야?" (개념 질문, 개선 아님)
- "이 프롬프트 의미가 뭐야?" (해석 요청, 개선 아님)

판별이 모호하면 1문장 확인 질문을 먼저 한다: "이 프롬프트를 개선해드릴까요, 아니면 의미를 설명해드릴까요?"

## Workflow

### Stage 1: Context Collection

1. **프롬프트 원문 확보**
   - 이미 채팅에 붙여넣어져 있으면 범위를 확인한다.
   - 파일 경로가 제시되면 Read.
   - 누락이면 한 번만 요청한다 ("어떤 프롬프트를 보고 싶으신가요?").

2. **필요 최소 정보 수집** (AskUserQuestion, 한꺼번에 묻지 않음)
   - 대상 모델: Claude 4.x 계열 / 다른 LLM / 불명
   - 주 용도: 단발 응답 / 에이전틱 / 도구 호출 / 긴 컨텍스트 / 코딩
   - 강한 제약: 응답 길이 / 비용 / 레이턴시 / 특정 포맷

모델이 불명이면 Claude 4.6/4.7 기준을 기본 가정으로 삼고, 가정을 명시한다.

### Stage 2: Reference Load

Read: `$GYEOL_HOME/memory/semantics/summary/001-anthropic-prompting-best-practices.md`

특히 "Prompt Authoring Checklist" 섹션을 진단 기준으로 사용한다. "Detailed Notes"의 코드 스니펫은 개선안 예시로 재사용 가능.

원문 전체(`source/001-...source.md`)는 체크리스트로 해결되지 않는 드문 경우에만 Read한다.

### Stage 3: Diagnosis

체크리스트 범주별로 합격/미달을 판정한다:

| 범주 | 핵심 질문 |
|------|-----------|
| Clarity & specificity | 원하는 결과가 명시적인가? 스코프와 예외가 분명한가? |
| Context & motivation | 제약의 이유가 설명되었는가? |
| Examples | few-shot이 필요한 과제에 3-5개 예시가 있는가? `<example>` 래핑되었는가? |
| Structure | 내용 유형별 XML 태그로 구분되었는가? 긴 컨텍스트가 위 / 쿼리가 아래인가? |
| Role & identity | 시스템 프롬프트에 역할이 부여되었는가? |
| Output control | 금지형 대신 수행형으로 쓰였는가? 마지막 턴 prefill 의존이 없는가? |
| Thinking & effort | effort 설정이 난이도에 맞는가? 공격적 언어로 과트리거 유도가 없는가? |
| Tool use & agentic | 행동 vs 제안 의도가 명확한가? 병렬 의도가 표시되었는가? |
| Long-horizon | 상태가 구조화된 파일로 유지되는가? 완료 기준이 검증 가능한가? |
| Anti-patterns | 테스트 하드코딩, 과대 방어 코딩, 불필요 추상화 유도가 없는가? |

미달 항목마다 **짧은 근거 + 개선 방향**을 기록한다. 근거는 체크리스트 항목 또는 Detailed Notes 섹션을 인용한다.

### Stage 4: Proposal

수정 규모에 맞춰 포맷을 선택한다:

- **소폭 수정** (3개 이하 항목): 구간별 diff
  ```
  Before: "Make it better"
  After:  "Refactor the loop to use parallel tool calls (see Parallel tool-call prompt)."
  Why:    Clarity & specificity (§Stage 3), Tool use (§parallel)
  ```
- **전면 개선** (다수 미달): 개선된 프롬프트 전문 + 변경 요점 목록

**옵션 제시** 선호: 사용자 결정 여지가 있을 때 "A안(간결 우선)"/"B안(엄격 우선)" 두 안 나열.

마지막에 체크리스트 커버리지를 한 줄로 보고: "10개 범주 중 7개 합격, 3개 개선 반영."

## Constraints

- **원본 의도 유지**: 사용자 의도를 바꾸지 않는다. 품질만 올린다.
- **근거 기반**: Anthropic 문서에 없는 주장을 하지 않는다. 항상 체크리스트 항목·Detailed Notes로 매핑.
- **영어/한국어**: 사용자 프롬프트의 원 언어를 유지한다. 진단·설명은 대화 언어(기본 한국어).
- **간결성**: 진단 보고는 범주당 1-2줄. 불필요한 해설을 배제한다.
- **모델 버전 주의**: Claude 4.5 → 4.6 → 4.7 사이 차이가 크다. 대상 모델이 불명이면 가정을 명시하고 진행한다.

## References

- `$GYEOL_HOME/memory/semantics/summary/001-anthropic-prompting-best-practices.md` — 주 참조 (요약·체크리스트·코드 스니펫)
- `$GYEOL_HOME/memory/semantics/source/001-anthropic-prompting-best-practices.source.md` — 원문 아카이브 (특수한 경우에만 Read)
- `$GYEOL_HOME/memory/semantics/_index.md` — 향후 관련 참조 축적 시 여기서 확장
