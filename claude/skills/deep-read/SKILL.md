---
name: deep-read
description: "Deeply analyze a codebase area and produce structured research documents. Spawns 3 parallel Explore agents for structure, data flow, and risk analysis. Triggers: 코드 분석해줘, 깊이 읽어봐, deep-read, /deep-read"
model: opus
allowed-tools: Read, Glob, Grep, Bash, Agent
---

# Deep Read — Codebase Research

Deeply analyze a code area and produce a structured research document at `.research/research-{topic}.md`.

## Workflow

### 1. Determine Target
- Parse `$ARGUMENTS` for the target (directory, module, or feature area)
- If no target specified, ask the user what to analyze

### 2. Launch 3 Parallel Explore Agents

Spawn 3 agents simultaneously using `Agent` tool with `run_in_background: true`:

| Agent | Role | Output |
|-------|------|--------|
| **structure-explorer** | File structure, entry points, type/interface mapping | `.research/.partial/structure.md` |
| **flow-explorer** | Data flow tracing, function call chains, state changes | `.research/.partial/dataflow.md` |
| **risk-explorer** | External/internal dependencies, vulnerabilities, implicit contracts, tech debt | `.research/.partial/risks.md` |

Each agent uses `subagent_type: "Explore"` with thoroughness "very thorough".

Agent prompt template:
```
You are a codebase researcher (see ~/.claude/agents/researcher.md for standards).
Your focus: {role description}.
Target scope: {target path}.
Write findings to: {output path}.
Read EVERY file in scope. Cite exact file:line ranges.
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

## Constraints
- **NO code modifications** — observation and documentation only
- **NO improvement suggestions** — report what IS, not what should be
- Create `.research/` directory if it does not exist
