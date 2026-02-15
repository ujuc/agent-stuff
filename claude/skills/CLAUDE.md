# CLAUDE.md — claude/skills

Guide for creating and maintaining Claude Code skills. Each skill is auto-discovered from `skills/<skill-name>/SKILL.md` when trigger phrases appear in conversation.

## Creating a New Skill

1. Create directory: `skills/<skill-name>/`
2. Copy template: `cp skill-template.md <skill-name>/SKILL.md`
3. Replace all `[bracket]` placeholders with actual values
4. Test activation by using trigger phrases in Claude

See skill-template.md for YAML frontmatter structure and required fields.

## Required Sections

| Section              | Purpose                                          |
| -------------------- | ------------------------------------------------ |
| `# Display Name`    | H1 title + one-sentence summary                  |
| `## Source of Truth` | Links to authoritative references                |
| `## When to Activate`| Trigger phrases (English + Korean) and contexts  |
| `## Instructions`   | Step-by-step features with **When** and **Steps** |
| `## Response Language`| Korean for user communication, English for files |
| `## See Also`        | Related documents and references                 |

## Conventions

- Trigger phrases: always include both English and Korean variants
- Steps: use numbered lists with bold step names
- Examples: include concrete input/output for each feature
- Supporting files: place additional resources in the skill directory

## Existing Skills

| Skill                | Triggers                             | Model  |
| -------------------- | ------------------------------------ | ------ |
| `agents`             | "에이전트해줘", "AGENTS.md 만들어줘" | haiku  |
| `commit`             | "커밋해줘", "commit changes"         | haiku  |
| `interview`          | "인터뷰해줘", "스펙 작성해줘"        | sonnet |
| `review`             | "리뷰해줘", "이거 괜찮아?"           | sonnet |
| `refactor`           | "리팩토링 해줘", "정리해줘"          | opus   |
| `troubleshoot`       | "왜 안돼?", "에러 났어"              | opus   |
| `generate-claude-md` | `/generate-claude-md` (manual)       | opus   |

## See Also

- skill-template.md — Base template for new skills
- ../CLAUDE.md — Skills table and priority hierarchy
