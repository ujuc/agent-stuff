---
name: generate-claude-md
description: "CLAUDE.md, AGENTS.md, contributing-docs/, .claude/rules/ 파일을 가이드 원칙에 따라 생성하거나 업데이트한다. /generate-claude-md, CLAUDE.md 업데이트, AGENTS.md 갱신 요청 시 사용한다."
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Agent, advisor
---

# CLAUDE.md Generator — Orchestrator

## Mode Detection

$ARGUMENTS를 분석하여 모드를 결정한다:

- **업데이트 모드**: "업데이트", "수정", "갱신", "update", "refresh" 포함 시
  → Stage 1(재분석) → U1(감사) → U2(비교) → U3(적용) → Stage 4(검증)
- **생성 모드**: 그 외
  → Stage 1 → Stage 2 → Stage 3 → Stage 4

### Target Identification

| Keyword | Target |
|---------|--------|
| "CLAUDE.md" (단독) | Root CLAUDE.md만 |
| "AGENTS.md" | AGENTS.md + contributing-docs/ |
| "rules" | .claude/rules/만 |
| 키워드 없이 "업데이트" | 5종 전체 |

$ARGUMENTS가 없으면 현재 작업 디렉토리를 대상으로 생성 모드.

---

## Generation Philosophy

이 스킬 전체를 관통하는 원칙이다.

**설계 원칙** (references/karpathy-guidelines.md): 생각 후 행동 / 단순함 우선 / 외과적 정밀함 / 목표 기반 실행

**콘텐츠 원칙** (references/osmani-guidelines.md): 발견 불가능한 정보만 포함. AGENTS.md는 코드로 해결하지 못한 문제의 진단 목록이다.

**성능 근거**: 자동 생성 컨텍스트 → 성공률 -2~3%, 비용 +20%. 수동 작성 gotcha → 성공률 +4% (ETH Zurich). 모든 줄은 존재 이유를 증명해야 한다.

**거버넌스 원칙** (references/entry-router-guidelines.md): 자율 에이전트 안전장치가 필요한 경우, Entry Router 패턴의 CORE 규칙을 AGENTS.md Boundaries와 CLAUDE.md 행동 가이드라인에 반영한다.

**영혼** (references/SOUL.md): 에이전트 정체성과 태도의 기저.

**LLM 컨텍스트**: LLM은 인컨텍스트 학습자 — 코드 패턴을 검색하면 스타일을 따라가므로 스타일 규칙 불필요. 상위 레벨 오류는 하류로 기하급수적 증폭. 지시사항을 검증 가능한 목표로 작성한다.

---

## Stage 1: Project Analysis

**참조**: references/stage1-analyzer.md (전체 절차, 에이전트 프롬프트 포함)

대상 디렉토리에서 패키지/빌드/테스트/린트 설정, 저장소 구조(모노레포/서브모듈), 문서/CI 구성, 기존 `.claude/rules/`를 자동 탐지한다.

**복잡도 판정**: 설정 파일 3종 이상, 모노레포, 서브모듈 존재 시 → 복잡 프로젝트.

- **복잡 프로젝트**: 3개 Explore 에이전트(model: sonnet) 스폰 — Explore-Config, Explore-Structure, Explore-Docs
- **단순 프로젝트**: 서브에이전트 없이 직접 탐지

탐지 결과를 발견 가능/불가능으로 분류하여 사용자에게 보여준다. 사실과 가정을 구분 표시한다.

**① advisor() 호출 조건**: 1단계 결과에서 모노레포 5+ 패키지, 서브모듈 3+ 개, 또는 기존 CLAUDE.md가 복잡한 구조인 경우 → advisor()로 분석 전략 검증.

---

## Stage 2: Interview (직접 실행 — 서브에이전트 위임 불가)

1단계 자동 탐지로 알 수 없는 항목만 질문한다:

- **WHY**: 프로젝트 목적/역할
- **WHAT**: 모노레포 패키지 역할, 서브모듈 관계, 외부 서비스 의존성
- **HOW**: 작업 규칙/워크플로우, 에이전트 반복 실수 여부, 중첩 CLAUDE.md 생성 승인

모호한 항목은 가능한 해석을 제시한 뒤 선택을 요청한다. 1단계 가정을 사용자에게 확인한다.

**심층 탐색 (선택)**: AskUserQuestion 대기 중, 대규모 모노레포(5+ 패키지)에서 미해결 질문이 있으면 Explore-Deep 에이전트(model: sonnet)를 백그라운드 스폰. 1단계 결과가 충분하면 스킵.

**업데이트 모드**: references/update-mode.md의 U1(감사)과 U2(비교)를 이 단계에서 통합 실행. U2 비교 보고서를 사용자에게 제시하고 업데이트 범위를 확인한다.

**② advisor() 호출 조건**: 사용자 답변이 1단계 탐지 결과와 모순되거나, 업데이트 모드에서 드리프트 항목이 10+개인 경우.

---

## Stage 3: Generation

**참조**: references/stage3-generator.md (A~E 파일별 생성 규칙, 공통 작성 규칙 포함)

1개 general-purpose 에이전트(model: sonnet)를 스폰하여 파일을 생성한다.

**전달할 것**: Stage 1 요약, Stage 2 답변, 대상 파일 목록, 4개 가이드라인 파일의 핵심 원칙(karpathy, osmani, entry-router, SOUL).

**생성 대상 5종**: Root CLAUDE.md, AGENTS.md, contributing-docs/, 중첩 CLAUDE.md, .claude/rules/ — 해당하는 것만 생성.

**업데이트 모드**: references/update-mode.md의 U3(적용)을 실행. Edit 도구로 외과적 수정만. 전체 재생성하지 않는다.

---

## Stage 4: Verification

**참조**: references/stage4-verifier.md (10항목 체크리스트, 안티패턴, Reviewer 프롬프트 포함)

3단계 파이프라인:

1. **Verifier**: 10항목 체크리스트를 줄 단위 적용 (model: sonnet)
2. **Iterative Fix**: 탈락 항목을 수정하고 재검증 (최대 2회 반복)
3. **Reviewer**: 생성물이 CLAUDE.md 1개를 넘어서면, 맹검 독립 검증 에이전트(model: sonnet) 스폰. 1~2단계 결과를 전달하지 않는다.

검증 결과를 사용자에게 보고한다. 탈락 항목은 줄과 사유를 함께 표시.

**③ advisor() 호출 조건**: Reviewer가 FAIL 판정을 내리고, 주 에이전트의 수정으로도 2회 반복 후 PASS되지 않는 경우.

---

## Advisor Escalation Summary

| # | When | Trigger |
|---|------|---------|
| ① | Stage 1 완료 후 | 모노레포 5+ 패키지, 서브모듈 3+, 또는 복잡한 기존 CLAUDE.md |
| ② | Stage 2 중 | 사용자 답변 ↔ 탐지 결과 모순, 또는 업데이트 드리프트 10+ |
| ③ | Stage 4 중 | Reviewer FAIL 후 2회 반복 수정으로도 PASS 불가 |

**advisor()를 호출하지 않는 경우**: 단순 프로젝트 생성, 파일 1~2개만 대상, 검증 1회차 PASS, 사용자가 명확한 지시를 준 경우.
