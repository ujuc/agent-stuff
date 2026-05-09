---
name: skill-index
description: "자체 스킬과 플러그인 명령을 8개 그룹과 워크플로우 색인으로 출력하는 메타스킬. 키워드를 잊었을 때 대화창에서 즉시 카탈로그를 본다."
when_to_use: "/skills, /skills <그룹명>, 스킬 목록, 스킬 그룹, 어떤 스킬 있어, 스킬 카탈로그, 스킬 보여줘, 무슨 스킬, list skills, show skills, what skills"
group: meta
model: haiku
allowed-tools: Bash
---

# /skills — 그룹별 스킬 카탈로그

자체 스킬과 플러그인 명령을 8개 그룹으로 분류해 마크다운 표로 출력한다. 사용자가 트리거 키워드를 잊었거나, 어떤 단계에서 어떤 스킬을 쓸지 모를 때 호출한다.

## When to invoke

- 사용자가 `/skills`, `스킬 목록`, `스킬 그룹`, `어떤 스킬 있어`, `스킬 보여줘` 같이 직접 카탈로그를 요청할 때
- 사용자가 작업 흐름 중간에 "다음에 뭐 쓰지" 식으로 막혔을 때
- 그룹 필터링: `/skills 기획`, `/skills planning`, `/skills 검증`

이 메타스킬을 자체적으로 추론·요약하지 말고 **반드시 `bash scripts/skill-index.sh` 를 호출**하여 그 출력을 그대로 사용자에게 보여준다. 스킬 목록의 진실은 frontmatter이며 이 스크립트만이 그 진실을 읽는다.

## Usage

```bash
# 전체 8 그룹 + 워크플로우 색인
bash ~/.claude/skills/skill-index/scripts/skill-index.sh

# 그룹 필터 (한글 라벨 또는 영문 slug)
bash ~/.claude/skills/skill-index/scripts/skill-index.sh 기획
bash ~/.claude/skills/skill-index/scripts/skill-index.sh planning

# 워크플로우 색인만
bash ~/.claude/skills/skill-index/scripts/skill-index.sh --workflow

# README 자동생성용 마크다운
bash ~/.claude/skills/skill-index/scripts/skill-index.sh --markdown
```

## Group definitions

| slug | 한글 라벨 | 포함 스킬 |
|---|---|---|
| `planning` | 🧭 기획·스펙 | spec-planner, sprint-contract-negotiator |
| `analysis` | 📐 분석·계획 | deep-read, annotate-plan |
| `build` | 🛠 구현·실행 | implement-plan, multi-agent-orchestrator |
| `verify` | ✅ 검증·QA | qa-evaluator, frontend-design-evaluator |
| `docs` | 📝 문서·커밋 | commit, generate-claude-md |
| `writing` | ✍️ 글쓰기 | humanizer, prompting-assist |
| `llm` | 🤖 외부 LLM | gemma, codex:* |
| `meta` | 🧪 메타·관리 | generate-skills, skill-improver, autoresearch, eos, maintain, skill-index |

플러그인 명령은 `tools/skill-index/plugin-groups.toml` 에서 동일 8 그룹에 매핑된다.

## Output contract

- stdout 마크다운만 — stderr는 warn(누락된 group 필드 등) 전용
- 종료 코드: 정상 0, 빌드 실패 1
- 출력은 결정적(deterministic) — 같은 입력에 같은 출력. 회사·집 머신에서 재현 가능.

## Adding a new skill

1. `~/.config/dotrc/agents/claude/skills/<new-skill>/SKILL.md` frontmatter에 `group: <slug>` 추가
2. `/skills` 호출 — 해당 그룹에 자동 노출
3. `~/.config/dotrc/agents/claude/CLAUDE.md` Skills 섹션 갱신은 별도 (수동 또는 향후 자동화)

## Plugin command 추가

플러그인 명령(`prefix:command`)은 SKILL.md 수정 권한이 없으므로 `tools/skill-index/plugin-groups.toml` 의 해당 그룹 `commands` 배열에 추가한다.

## References

- Plan: `~/.claude/plans/agents-claude-plans-claude-skills-recur-enchanted-pudding.md`
