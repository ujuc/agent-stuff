---
name: humanizer
argument-hint: "[--strict | redo] [텍스트 또는 파일 경로] [장르: ...] [강도: ...]"
description: |
  AI가 쓴 글의 흔적을 자연스러운 사람의 글로 바꾸는 윤문 오케스트레이터.
  한국어 주력, 영어 부수 지원. Fast 모드(monolith 단일 호출, 디폴트)는 5,000자 이하를
  2~3분에 처리하고, --strict 모드는 4인 파이프라인(탐지→윤문→의미감사+자연성리뷰)
  으로 8,000자+ 정밀 검증. 의미 불변·근거 기반·장르 유지·과윤문 가드(30/50%)·
  Do-NOT list(고유명사/수치/인용)·등급 A~D 자동 채점. 트리거: AI 글 자연스럽게,
  AI 티 제거, ChatGPT 문체, 번역투 고쳐, 사람이 쓴 것처럼 윤문, 휴머나이저, redo, 2차 윤문, --strict.
group: writing
model: sonnet
allowed-tools:
  - Read
  - Write
  - Edit
  - Grep
  - Glob
  - Bash
  - AskUserQuestion
  - Agent
---

# Humanizer — AI 글 흔적 제거 오케스트레이터

AI가 생성한 한국어/영어 텍스트의 흔적을 찾아 자연스러운 사람의 글로 바꾼다.
Fast 모드(디폴트, 단일 호출)와 Strict 모드(4인 파이프라인) 듀얼 트랙.

## Phase 0 — 컨텍스트 확인 및 모드 결정

작업 시작 시 첫 줄로 다음을 출력한다:

```
humanizer v2.0 — {fast|strict|redo} 모드 / run_id: {YYYY-MM-DD-NNN} / 언어: {ko|en|mixed}
```

**모드 결정 우선순위:**

1. 사용자가 `redo` 명시 또는 "특정 카테고리만 다시", "이 문단만", "2차 윤문" → **redo**
2. 사용자가 `--strict` 명시 또는 "정밀 모드", "4인 파이프라인" → **strict**
3. 한국어 입력이 8,000자 초과 → **strict 자동 승급** (사용자에 1줄 고지)
4. 영어 단독 입력 → **fast 강제** (strict는 한국어 전용)
5. 그 외 → **fast (디폴트)**

**옵션 인자 (자연어로 받음):**

- `장르: 칼럼|리포트|블로그|공적` — 미지정 시 입력 첫 300자로 자동 추정
- `강도: 보수|기본|적극` — 기본값 `기본`
- `최소심각도: S1|S2|S3` (strict) 또는 `P1|P2|P3` (fast 영어) — 기본 S2/P2

## Phase 1 — 입력 저장 및 run_id

1. cwd 기준 `_workspace/{YYYY-MM-DD-NNN}/` 디렉토리 생성. NNN은 그날의 시퀀스.
2. 입력 텍스트를 `01_input.txt`에 저장.
3. 첫 300자로 장르 자동 추정 (사용자 명시 시 그쪽 우선).
4. 언어 감지: 한글 비율 70%+ → ko, 영문 비율 70%+ → en, 그 외 mixed.
5. redo 진입이면 가장 최신 `_workspace/` 하위 디렉토리를 재사용 (사용자 발화에 "이전 거 다시" 같은 신호가 있을 때).

## Fast 모드 (디폴트)

### 한국어 입력

`humanize-monolith` 에이전트를 `Agent` 도구로 1회 호출.

호출 인자:
```
input_path: <abs path>/_workspace/{run_id}/01_input.txt
quick_rules_path: ~/.claude/skills/humanizer/references/quick-rules.md
genre_hint: 칼럼 | 리포트 | 블로그 | 공적 | null
```

monolith가 한 호출 안에서 탐지 → 윤문 → 자체검증 → 출력까지 수행하고
`final.md` + `summary.md`를 작성한다. 다른 sub-agent를 호출하지 않는다.

### 영어 입력 (fast 전용 트랙)

monolith는 한국어 전용이므로 영어 입력은 SKILL.md 본문에서 직접 처리한다.

1. `references/patterns-en.md` + `references/patterns-common.md` 로드.
2. 콘텐츠 유형 매트릭스(아래)로 적용 기준 결정.
3. 패턴 카탈로그 치트시트로 1차 스캔, 심각도 부여 (P1/P2/P3).
4. 수정 — P1 무조건, P2 맥락 판단, P3 선택. 변경률 30/50% 가드 적용.
5. 자체검증 6항 (한국어와 동일) 점검 — 위반 시 해당 edit 롤백 후 재시도.
6. `final.md` + `summary.md` 작성. summary 포맷은 monolith와 동일.

### Mixed 입력

한국어 부분은 monolith로, 영어 부분은 SKILL 본문 처리로 분리하지 않고,
**전체를 영어 fast 트랙처럼 SKILL 본문에서 직접 처리**한다 (monolith가
한국어 전용이라 mixed 처리 시 영어 구간을 망가뜨릴 수 있기 때문).

### Fast 응답 형식

산출물 작성 후 다음 4가지를 사용자에게 짧게 반환:

1. 한 줄 상태: `완료. 변경률 X% / 등급 Y / 자체검증 N/6 통과`
2. 윤문본 본문 (final.md 내용을 마크다운 블록으로)
3. summary.md의 핵심 표 (메트릭 + 카테고리 탐지 + 자체검증)
4. 등급 B 이하면 "정밀 검증이 필요하면 `--strict`로 4인 파이프라인 실행 가능" 안내

**디폴트 wall-clock 목표:** 5,000자 이하 2~3분, 8,000자 5~7분.

## Strict 모드 (`--strict` 또는 8,000자+ 자동 승급)

**한국어 전용.** 영어 입력에 `--strict`가 들어오면 fast 강제 + 안내:
"strict 모드는 한국어 전용입니다. 영어는 fast 모드만 지원합니다."

### Phase A — 탐지

`humanize-detector` 에이전트 호출 → `02_detection.json` 생성.
입력: `01_input.txt`, `genre_hint`, `min_severity` 옵션.

### Phase B — 윤문 (최대 3회 루프)

`humanize-rewriter` 에이전트 호출 → `03_rewrite.md` + `03_rewrite_diff.json`.
입력: `01_input.txt`, `02_detection.json`, `preserve_formatting` 옵션.

### Phase C — 병렬 검증 (에이전트 팀)

`Agent` 도구를 병렬로 두 번 호출 (단일 메시지에 두 tool call):

- `humanize-fidelity-auditor` → `04_fidelity_audit.json` (의미 동등성 13항)
- `humanize-naturalness-reviewer` → `05_naturalness_review.json` (잔존 + 과윤문)

### Phase C 종합 판정

| fidelity | naturalness | 종합 | 후속 |
|---|---|---|---|
| full_pass | accept / accept_with_note | **최종 승인** | Phase D |
| full_pass | rewrite_round_2 | **2차 윤문** | Phase B 재호출 (target finding) |
| full_pass | rollback_and_rewrite | **롤백 후 재윤문** | rewriter에 edit 롤백 지시 |
| conditional_pass | - | **롤백된 edit만 재시도** | Phase B 재호출 |
| fail | - | **전면 재작업** | Phase B 전면 재호출 |

2차/3차 윤문 진입 시 `03_rewrite_v2.md`·`v3.md`로 버전 분리.
**최대 3회 후 미해결이면 `hold_and_report`** — 사용자에게 사람 검토 권고.

### Phase D — 최종 출력

1. `03_rewrite_vN.md` 또는 `03_rewrite.md`를 `final.md`로 복사.
2. `summary.md` 생성 (fast 모드와 동일 포맷).
3. 사용자에게 응답 4종 (fast와 동일).

## Redo 모드 (`/humanizer redo [지시]`)

가장 최신 `_workspace/{run_id}/` 식별 (없으면 안내 후 종료).

**사용자 지시 파싱:**

| 사용자 발화 | 처리 |
|---|---|
| "특정 카테고리만 다시" / "번역투만" / "관용구만" | strict + 해당 카테고리 finding만 Phase B 재실행 |
| "이 문단만" / "두 번째 문단만" | strict + 해당 범위 finding만 처리 |
| "2차 윤문" / 지시 없음 | 잔존 finding 전체 대상 round 2 |
| "강도 낮춰" / "보수적으로" | min_severity = S1만 |
| "강도 높여" | min_severity = S1+S2+S3 |
| "장르 바꿔서 X" | 새 run_id + genre_hint 변경 후 Phase A부터 |
| "이 변경 되돌려줘" | fidelity-auditor 롤백 명령으로 처리 |

`humanize-rewriter` 재호출 + 재검증. 산출물은 `03_rewrite_v2.md` (또는 v3).
**최대 round 3.** 그 이상은 `hold_and_report`로 사람 검토 권고.

## 운영 안전망 (모든 모드 공통)

### 변경률 가드

- 변경률 = 레벤슈타인 거리 / 원문 길이.
- **30% 초과 → 경고** (summary.md에 기록).
- **50% 초과 → 강제 중단·마지막 안전 버전으로 롤백** (`over_polish_aborted: true`).

### Do-NOT list (탐지·윤문 모두 제외)

- 고유명사·제품명·모델명·기관명 (GPT-4, Claude 3, Gemini 등).
- 수치·날짜·단위·백분율.
- 큰따옴표 안 직접 인용문.
- 법률·규정 조문.
- 수학·화학·통계 표기.
- 영어 약어 업계 표준 (LLM, GPU, MCP, API, SDK 등).

### 자체검증 6항 (윤문 직후 자가 점검)

1. 고유명사·수치·날짜·인용 100% 보존
2. 변경률 30% 이하 (50% 초과는 작업 중단)
3. 장르 이탈 없음 (칼럼이 에세이로 변하지 않음)
4. register 보존 (격식체→격식체)
5. 잔존 S1 패턴 0건 (한국어) 또는 P1 패턴 0건 (영어)
6. 인공 표현 자제 (원문에 없던 비유·수사 임의 추가 금지)

위반 시: edit 롤백 → 다시 윤문 → 재점검. 자체 루프 최대 1회.
이상 미해결이면 결과 출력하되 summary.md에 "자가검증 미통과 N건" 표기.

### 등급 자동 채점 (A~D)

- **A**: S1/P1 잔존 0, S2/P2 잔존 2 이하, 변경률 10~25%, 자체검증 6/6
- **B**: S1/P1 잔존 0, S2/P2 잔존 4 이하, 자체검증 5+/6
- **C**: S1/P1 잔존 1~2 또는 자체검증 4/6 이하 → strict 권고
- **D**: S1/P1 잔존 3+ 또는 변경률 50% 초과 → 작업 중단 권고

## 콘텐츠 유형 매트릭스

장르별 적용 기준이 다르다. 다음 순서로 판단:

1. 사용자가 명시한 경우 → 그대로 따름
2. 파일 확장자/경로로 추론 (예: `README.md` → 기술 문서, `blog/` 하위 → 블로그)
3. 본문 내용으로 추론 (코드 블록 비율, 어조, 형식)
4. 판단이 어려우면 AskUserQuestion으로 확인

| 유형 | 적용 기준 | "숨결 주입" |
|------|-----------|-------------|
| **블로그/에세이** | 모든 패턴 적용 | O — 의견, 1인칭, 개성 적극 권장 |
| **기술 문서** | 명확성 우선. 수식어/filler 제거 | X — 감정/개성 주입 금지. 정확하고 건조하게 |
| **마케팅/카피** | 과장은 줄이되 설득력 유지. 구체적 수치로 대체 | 제한적 — 브랜드 보이스에 맞춰 |
| **학술/리포트** | 정확성과 출처 중심. weasel word 제거 | X — 객관적 톤 유지 |
| **코드 주석** | 간결성 우선. 불필요한 설명 제거 | X |
| **SNS/캐주얼** | 과도한 형식성 제거. 구어체 허용 | O — 자유롭게 |

## 패턴 카탈로그 참조

대상 텍스트의 언어와 모드에 따라 다음 파일을 Read한다:

| 모드 | 언어 | 카탈로그 |
|---|---|---|
| Fast | ko | `references/quick-rules.md` (monolith가 직접 로드) |
| Fast | en | `references/patterns-en.md` + `references/patterns-common.md` |
| Fast | mixed | 위 영어 카탈로그 + `references/patterns-ko.md` |
| Strict | ko | `references/taxonomy-ko.md` (detector가 SSOT로 로드) + `references/playbook-ko.md` (rewriter가 처방으로 로드) |

`patterns-ko.md` 상단에는 K↔A-J 매핑 표가 있어 fast/strict 트랙 사이의
ID 상호 참조가 가능하다.

## 글에 숨결 불어넣기 (블로그/SNS 전용)

> **적용 대상: 블로그/에세이, SNS/캐주얼만.** 기술 문서, 학술, 코드 주석에는 적용하지 않는다.

AI 패턴 제거는 절반. 깨끗하지만 무미건조한 글도 AI처럼 보인다.

### 영혼 없는 글의 징후

- 모든 문장이 비슷한 길이와 구조
- 의견 없이 사실만 나열
- 불확실함이나 복잡한 감정에 대한 인정 없음
- 적절한 곳에서도 1인칭 회피
- 유머, 날카로움, 개성 부재
- 보도자료나 백과사전처럼 읽힘

### 숨결을 넣는 법

**의견을 가져라.** 사실을 보고하는 데 그치지 말고 반응하라. "솔직히 이건 좀 애매하다"가 장단점을 중립적으로 나열하는 것보다 낫다.

**리듬을 바꿔라.** 짧은 문장. 그리고 좀 더 천천히 가는 긴 문장. 섞어 써라.

**복잡함을 인정하라.** 사람은 복잡한 감정을 가진다. "인상적인데 동시에 좀 불편하다"가 "인상적이다"보다 사람답다.

**'나'를 쓸 때는 써라.** 1인칭이 비전문적인 게 아니다. "계속 생각나는 건..."이나 "내가 걸리는 부분은..."은 실제로 생각하는 사람의 표현이다.

**약간의 지저분함을 허용하라.** 완벽한 구조는 알고리즘 냄새가 난다. 곁가지, 여담, 반쯤 정리된 생각은 사람의 것이다.

**감정을 구체적으로.** "우려된다"가 아니라 "새벽 3시에 아무도 안 보는데 에이전트가 혼자 돌아가는 거 생각하면 좀 소름 돋는다."

## 주의 사항

- **의미 불변이 최상위 불문율.** monolith·strict 모두에서 위반 즉시 롤백.
- **장르 이탈 금지.** 칼럼이 에세이로, 에세이가 문학으로 옮겨가지 않음.
- **register 보존.** 격식체 입력 → 격식체 출력. AI 티는 문법·수사이지 격식 자체가 아님.
- **자동 로드 금지.** 프로젝트 CLAUDE.md 등 다른 파일을 자동 파싱해 옵션을 추론하지 않음.
- **이전 산출물 백업.** `final.md`·`summary.md`가 이미 존재하면 `_prev` 접미로 백업 후 새로 작성.

## 참고 자료

- 한국어 fast 룰북: `references/quick-rules.md` (S1·S2 핵심 + 자체검증 6항 + 등급 기준)
- 한국어 strict SSOT: `references/taxonomy-ko.md` (10대분류 × 40+ 패턴)
- 한국어 strict 처방: `references/playbook-ko.md` (카테고리별 치환 레시피)
- 한국어 fast 카탈로그: `references/patterns-ko.md` (K1~K19 + 매핑 표)
- 영어 fast 카탈로그: `references/patterns-en.md` (E1~E19)
- 공통 패턴: `references/patterns-common.md` (C1~C6)

## Sub-agents (strict / fast 모두 사용)

- `humanize-monolith` — Fast 한국어 단일 호출
- `humanize-detector` — Strict Phase A
- `humanize-rewriter` — Strict Phase B (redo도 사용)
- `humanize-fidelity-auditor` — Strict Phase C-1
- `humanize-naturalness-reviewer` — Strict Phase C-2

이들은 `~/.config/dotrc/agents/claude/agents/` (= `~/.claude/agents/`)에 위치.

## Acknowledgements

Strict 트랙(5인 → 4인 파이프라인)과 fast 트랙(monolith)의 설계 및 quick-rules,
taxonomy-ko, playbook-ko, sub-agent 정의는 [`epoko77-ai/im-not-ai`](https://github.com/epoko77-ai/im-not-ai)
v1.5의 `humanize-korean` 스킬에서 가져왔다 (MIT License). 자세한 출처와 변경
사항은 [`LICENSE-THIRD-PARTY`](./LICENSE-THIRD-PARTY)에 기록.
