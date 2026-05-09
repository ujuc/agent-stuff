# Skills

이 디렉토리는 Claude Code 자체 스킬을 담는다. 스킬 카탈로그는 SKILL.md frontmatter의 `group:` 필드를 단일 진실 소스로 삼아 [`skill-index/`](skill-index/) 메타스킬이 자동 생성한다.

전체 카탈로그(플러그인 명령 포함)는 대화창에서 `/skills` / `스킬 목록` / `스킬 그룹` / `어떤 스킬 있어` 호출, 또는:

```bash
bash skill-index/scripts/skill-index.sh
bash skill-index/scripts/skill-index.sh 기획        # 그룹 필터 (한글 라벨)
bash skill-index/scripts/skill-index.sh planning    # 그룹 필터 (영문 slug)
bash skill-index/scripts/skill-index.sh --workflow  # 워크플로우 색인만
```

## 그룹

| slug | 한글 라벨 | 자체 스킬 |
| --- | --- | --- |
| `planning` | 🧭 기획·스펙 | `spec-planner`, `sprint-contract-negotiator` |
| `analysis` | 📐 분석·계획 | `deep-read`, `annotate-plan` |
| `build` | 🛠 구현·실행 | `implement-plan`, `multi-agent-orchestrator` |
| `verify` | ✅ 검증·QA | `qa-evaluator`, `frontend-design-evaluator` |
| `docs` | 📝 문서·커밋 | `commit`, `generate-claude-md` |
| `writing` | ✍️ 글쓰기 | `humanizer`, `prompting-assist` |
| `llm` | 🤖 외부 LLM | `gemma`, `codex:setup`, `codex:rescue` |
| `meta` | 🧪 메타·관리 | `skill-index`, `generate-skills`, `skill-improver`, `autoresearch`, `eos` |

플러그인 명령 매핑은 [`skill-index/tools/skill-index/plugin-groups.toml`](skill-index/tools/skill-index/plugin-groups.toml).

## 워크플로우 색인

```
[새 프로젝트]   spec-planner → sprint-contract-negotiator → annotate-plan
                → implement-plan → qa-evaluator → commit
[기존 코드]     deep-read → annotate-plan → implement-plan → commit
[스킬 정비]     skill-improver → generate-skills → maintain → eos
[글쓰기]        prompting-assist → humanizer
[디자인]        frontend-design-evaluator → multi-agent-orchestrator
```

`maintain`은 project-scoped (`agent-stuff/.claude/skills/maintain/`)이라 user-scope 카탈로그에는 포함되지 않으나, 워크플로우 단계로는 등장한다.

## Harness Pipeline

5개 harness 스킬이 단일 파이프라인을 구성한다 (`build` / `verify` / `planning`에 분포):

```
[User Prompt]
    │
    ▼
[spec-planner] ── Product spec
    │
    ▼
[sprint-contract-negotiator] ── Done criteria
    │
    ▼
[Generator] ── Implementation
    │
    ├── [qa-evaluator] ── Functional QA
    └── [frontend-design-evaluator] ── Design QA
    │
    ▼
[multi-agent-orchestrator] orchestrates the full pipeline
```

Chrome 의존 스킬(`qa-evaluator`, `frontend-design-evaluator`)은 `--chrome` 플래그 또는 `/chrome` 명령으로 활성화한다.

## Skill Structure Convention

각 스킬은 `<skill>/SKILL.md` 패턴이며 선택적으로 `references/`, `scripts/`, `tools/` 디렉토리를 둔다. 전체 frontmatter 스펙은 [`generate-skills/references/frontmatter-spec.md`](generate-skills/references/frontmatter-spec.md).

신규 스킬 추가 시 frontmatter에 **반드시** `group: <slug>` 필드를 둔다 — 위 8개 슬러그 중 선택. 누락하면 `/skills` 출력의 "❓ 미분류" 섹션에 stderr warn과 함께 표시된다.

## References

- [`CLAUDE.md`](./CLAUDE.md) — skill 작성 규약과 conventions
- [`../CLAUDE.md`](../CLAUDE.md) — Claude Code global configuration
- [`skill-index/`](./skill-index/) — `/skills` 메타스킬 (Rust + bash launcher)
