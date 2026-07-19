---
name: skill-index
description: "자체 스킬과 플러그인 명령을 8개 그룹과 워크플로우 색인으로 출력하는 메타스킬. 키워드를 잊었을 때 대화창에서 즉시 카탈로그를 본다."
when_to_use: "/skills, /skills <그룹명>, 스킬 목록, 스킬 그룹, 어떤 스킬 있어, 스킬 카탈로그, 스킬 보여줘, 무슨 스킬, list skills, show skills, what skills"
group: meta
model: haiku
allowed-tools: Bash
---

# /skills — Skill Catalog by Group

Renders self-authored skills and plugin commands as a markdown table across 8
groups. Invoked when the user has forgotten trigger keywords or does not know
which skill fits the current stage of work.

## When to invoke

- The user directly requests the catalog: `/skills`, `스킬 목록`, `스킬 그룹`, `어떤 스킬 있어`, `스킬 보여줘`
- The user stalls mid-workflow with a "다음에 뭐 쓰지"-style question
- Group filtering: `/skills 기획`, `/skills planning`, `/skills 검증`

Do not reason about or summarize this catalog yourself — **always run
`bash scripts/skill-index.sh`** and show its output to the user verbatim. The
frontmatter is the source of truth for the skill list, and this script is the
only reader of that truth.

## Usage

```bash
# Full 8 groups + workflow index
bash ~/.claude/skills/skill-index/scripts/skill-index.sh

# Group filter (Korean label or English slug)
bash ~/.claude/skills/skill-index/scripts/skill-index.sh 기획
bash ~/.claude/skills/skill-index/scripts/skill-index.sh planning

# Workflow index only
bash ~/.claude/skills/skill-index/scripts/skill-index.sh --workflow

# Markdown for README auto-generation
bash ~/.claude/skills/skill-index/scripts/skill-index.sh --markdown
```

## Group definitions

| slug | Korean label | skills |
|---|---|---|
| `planning` | 🧭 기획·스펙 | spec-planner, sprint-contract-negotiator |
| `analysis` | 📐 분석·계획 | deep-read, annotate-plan |
| `build` | 🛠 구현·실행 | implement-plan, multi-agent-orchestrator |
| `verify` | ✅ 검증·QA | qa-evaluator, frontend-design-evaluator |
| `docs` | 📝 문서·커밋 | commit, generate-agent-docs |
| `writing` | ✍️ 글쓰기 | humanizer, prompting-assist |
| `llm` | 🤖 외부 LLM | gemma, codex:* |
| `meta` | 🧪 메타·관리 | generate-skills, skill-improver, autoresearch, maintain, skill-index |

Plugin commands are mapped to the same 8 groups in
`tools/skill-index/plugin-groups.toml`.

## Output contract

- stdout carries markdown only — stderr is reserved for warnings (missing `group` fields, etc.)
- Exit codes: 0 on success, 1 on build failure
- Output is deterministic — same input, same output; reproducible across machines.

## Adding a new skill

1. Add `group: <slug>` to the frontmatter of `~/.config/dotrc/agents/claude/skills/<new-skill>/SKILL.md`
2. Run `/skills` — the skill appears under its group automatically
3. Updating the Skills section of `~/.config/dotrc/agents/claude/CLAUDE.md` is a separate step (manual, or future automation)

## Adding a plugin command

Plugin commands (`prefix:command`) expose no SKILL.md to edit, so add them to
the group's `commands` array in `tools/skill-index/plugin-groups.toml`.

## References

- Group taxonomy & catalog: `../README.md`
- Plugin command groups: `tools/skill-index/plugin-groups.toml`
