---
name: maintain
description: "agent-stuff 저장소의 구조 정합성, 문서 동기화, 스킬 품질을 점검하고 유지보수한다. /maintain, 정비해줘, 헬스체크, 문서 동기화, 스킬 점검, 스킬 감사 요청 시 반드시 이 스킬을 사용할 것."
model: opus
allowed-tools: Agent, Read, Glob, Grep, Bash, Edit, Write, TaskCreate, TaskUpdate
user-invocable: true
argument-hint: "[full|skill|docs|health]"
---

# Maintain — Agent-stuff Repository Maintenance Orchestrator

Dispatches specialist agents to verify and maintain the agent-stuff configuration repository.

## Modes

| Mode | Agents Dispatched | Purpose |
|------|-------------------|---------|
| `health` (default) | health-checker | Structure validation report |
| `docs` | health-checker → doc-syncer | Detect issues then fix documentation |
| `skill` | skill-engineer | Skill audit or lifecycle task |
| `full` | health-checker → doc-syncer → skill-engineer | Complete maintenance pass |

## Workflow

### 1. Parse Mode

```
mode = $ARGUMENTS or "health"
valid modes: health, docs, skill, full
```

If `$ARGUMENTS` does not match a valid mode, treat the entire argument as a task description and route to the most relevant agent.

### 2. Dispatch Agents

All agents run as sub-agents (not agent teams). Each agent uses its own model from frontmatter.

**health mode:**
```
Agent(subagent_type: "health-checker")
→ Return health report to user
```

**docs mode:**
```
Agent(subagent_type: "health-checker")
→ Read health report
→ Agent(subagent_type: "doc-syncer",
        prompt: include health report findings)
→ Summarize changes made
```

**skill mode:**
```
Agent(subagent_type: "skill-engineer",
      prompt: include $ARGUMENTS context if provided)
→ Return audit results or delegate to generate-skills/autoresearch
```

**full mode:**
```
health → docs → skill (sequential)
→ Comprehensive summary of all findings and changes
```

### 3. Present Results

Summarize findings in Korean. Do NOT auto-commit — let the user invoke `/commit` when ready.

Format:
```
## 정비 결과 ({mode} 모드)

### 검증 결과
- PASS: N건
- WARN: N건
- FAIL: N건

### 수행한 변경
- [변경 목록]

### 미해결 항목
- [수동 처리 필요 항목]
```
