---
name: gemma
description: "로컬 Ollama에서 실행 중인 Google Gemma 모델(최신 설치 버전 자동 감지)에 프롬프트를 전달하고 응답을 받아온다. gemma, gemma4, gemma로 요약해줘, gemma로 번역해, 로컬 LLM으로 처리해, 오프라인 AI, Gemma 호출 요청 시 사용한다. 민감 정보 오프라인 처리, 긴 컨텍스트 요약, 다국어 번역, 초안 생성 등에 적합."
model: sonnet
allowed-tools: Bash(bash:*), Bash(ollama:*), Bash(curl:*)
argument-hint: "[variant] <prompt>"
---

# Gemma (로컬 Ollama)

로컬 Ollama로 실행 중인 Google Gemma 모델에 프롬프트를 전달하고 응답을 반환한다. Ollama에 설치된 최신 gemma 버전을 자동으로 감지하여 사용한다.

## 사용 방식

`scripts/query.sh`를 Bash로 호출한다. 첫 인자가 짧은 variant 태그(예: `e2b`, `e4b`, `26b`, `31b`)이면 모델 변형으로 사용하고, 아니면 전체를 프롬프트로 간주한다. 변형 미지정 시 기본값은 `latest`다.

```bash
# 기본 (자동 감지된 최신 버전의 latest 태그)
bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "한국어로 자기소개 해줘"

# variant 명시
bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh e4b "이 문단을 3줄로 요약해줘: ..."
```

스크립트가 stdout으로 Gemma의 응답만 출력하므로 결과를 그대로 사용자에게 전달하거나 후속 작업의 입력으로 사용한다.

## 모델 자동 감지

스크립트는 Ollama에 설치된 gemma 계열 모델 중 최신 버전을 자동으로 선택한다.

- `gemma3`, `gemma4`, `gemma5` 등 `gemma[0-9]+` 패턴의 모델을 탐지
- 버전 번호가 가장 높은 모델을 기본으로 사용
- `GEMMA_MODEL` 환경변수로 전체 모델명 직접 지정 가능 (예: `gemma3:2b`)
- variant 인자를 주면 감지된 최신 버전의 해당 variant를 사용

## 절차

1. 사용자 요청에서 프롬프트 본문과 (선택) variant를 식별한다
2. 긴 입력(코드, 노트, 문서)이 포함된 경우 프롬프트에 명확한 지시문을 먼저 쓰고 본문을 heredoc이나 단일 문자열로 전달한다
3. `Bash` 도구로 `scripts/query.sh`를 호출한다
4. stdout 응답을 사용자에게 전달한다 — Gemma 응답은 Claude의 답변이 아님을 명확히 표시한다 (예: `Gemma (gemma4:e4b) 응답:` 헤더, stderr의 info 메시지에서 실제 모델명 확인)
5. 에러가 발생하면 스크립트가 안내한 조치(ollama serve, ollama pull 등)를 사용자에게 전달한다

## 환경변수

| 변수 | 기본값 | 설명 |
|------|--------|------|
| `GEMMA_HOST` | `http://localhost:11434` | Ollama 서버 주소 |
| `GEMMA_TIMEOUT` | `120` | 요청 타임아웃(초) |
| `GEMMA_VARIANT` | `latest` | 기본 variant 태그 |
| `GEMMA_MODEL` | (없음) | 전체 모델명 직접 지정, 자동 감지 우회 |

## 적절한 활용 사례

- 민감 데이터 요약·분류 (네트워크 송출 없이 로컬 처리)
- Obsidian 노트 초안 작성·정리
- 다국어 번역(140개 이상 지원)
- 긴 로그/문서 요약 (128K+ 컨텍스트)
- 함수 호출·JSON 구조화 출력이 필요한 에이전틱 태스크

## 부적절한 사례 (Claude로 처리 권장)

- 고난도 수학·추론 (AIME, GPQA Hard) — Claude 우위
- 대규모 코드베이스 탐색 — Claude Code의 Agent/Explore가 적합
- 음악·비음성 오디오 이해 — Gemma 미지원

## 에러 처리

스크립트는 다음 경우에 명확한 에러 메시지를 출력하고 non-zero exit한다:

- Ollama 서버 미실행 (`localhost:11434` 응답 없음) → `ollama serve`
- gemma 모델 미설치 → `ollama pull gemma4`
- 지정 variant 미설치 → `ollama pull {version}:{variant}`
- 타임아웃(기본 120초) → 프롬프트 단축 또는 소형 variant 권장
- 필수 의존(`curl`, `jq`) 누락 → 설치 안내
