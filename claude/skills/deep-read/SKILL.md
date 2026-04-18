---
name: deep-read
description: "코드베이스 영역을 깊이 분석하여 구조화된 리서치 문서를 생성한다. 구조, 데이터 흐름, 리스크 분석을 위해 3개 병렬 researcher 에이전트를 디스패치한다. 코드 분석해줘, 깊이 읽어봐, deep-read, /deep-read 요청 시 사용한다."
model: sonnet
argument-hint: "[target-path]"
allowed-tools: Read, Glob, Grep, Bash, Agent, advisor
---

# Deep Read — Codebase Research

Deeply analyze a code area and produce a structured research document at `.research/research-{topic}.md`.

## Workflow

### 1. Determine Target
- Parse `$ARGUMENTS` for the target (directory, module, or feature area)
- If no target specified, ask the user what to analyze

### 2. Launch 3 Parallel Researcher Agents

Spawn 3 agents simultaneously using `Agent` tool with `subagent_type: "researcher"` and `run_in_background: true`. The researcher agent definition (`~/.claude/agents/researcher.md`) enforces citation rules, exploration depth, and no-modification constraints automatically.

| Agent | Role | Output |
|-------|------|--------|
| **structure-explorer** | File structure, entry points, type/interface mapping | `.research/.partial/structure.md` |
| **flow-explorer** | Data flow tracing, function call chains, state changes | `.research/.partial/dataflow.md` |
| **risk-explorer** | External/internal dependencies, vulnerabilities, implicit contracts, tech debt | `.research/.partial/risks.md` |

Agent prompt template:
```
Focus: {role description}.
Target: {target path}.
Output: {output path}.
```

### 3. Merge Results

After all 3 agents complete, read `.partial/` files and merge into `.research/research-{topic}.md`:

```markdown
# Research: {topic}

## Architecture Overview
(from structure-explorer)

## Key Files & Responsibilities
(from structure-explorer)

## Data Flow
(from flow-explorer)

## Dependencies
(from risk-explorer)

## Patterns & Conventions
(cross-reference: structure + flow)

## Gotchas & Risks
(from risk-explorer)

## Integration Points
(cross-reference: flow + risk)
```

### 4. Cleanup
- Delete `.research/.partial/` directory
- Output: "`.research/research-{topic}.md` has been created. Please review it for accuracy before proceeding to planning."

## Advisor Escalation

This skill runs on sonnet by default. At the decision points below, call `advisor()` to borrow higher-tier reasoning:

- **Before Step 3 merge**: when the 3 researcher agents (structure / dataflow / risk) report contradictory findings, or when synthesizing the Architecture Overview is ambiguous.
- **When risk-explorer reports a Critical-level risk**: to judge whether that risk is load-bearing and how firmly to state it in the "Gotchas & Risks" section.

How to call: invoke `advisor()` with no parameters. The full current conversation context (including the `.partial/` outputs from all three agents) is automatically forwarded to the higher-tier model. Use this only when **the merge direction itself needs a structural check** — not for simple Q&A.

## Constraints
- **NO code modifications during merge** — observation and documentation only (per-agent rules are enforced by researcher.md)
- Create `.research/` directory if it does not exist
