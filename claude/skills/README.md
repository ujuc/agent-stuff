# Skills

## Overview

| Skill | Purpose | Model |
|-------|---------|-------|
| annotate-plan | Implementation plan with annotation cycles | sonnet + advisor |
| autoresearch | Autonomous skill optimizer via eval loops | opus |
| commit | Korean Conventional Commits | sonnet |
| deep-read | Deep codebase analysis with parallel agents | sonnet + advisor |
| frontend-design-evaluator | GAN-style frontend design quality evaluation | sonnet + advisor |
| gemma | Local Gemma model query via Ollama (auto-detects latest version) | sonnet |
| generate-claude-md | CLAUDE.md/AGENTS.md generator | opus |
| generate-skills | Skill generator | opus |
| implement-plan | Plan executor with continuous verification | sonnet |
| skill-improver | Test-driven skill improvement loop | sonnet + advisor |
| multi-agent-orchestrator | Planner-Generator-Evaluator pipeline | opus |
| qa-evaluator | Web app QA testing via Chrome | sonnet + advisor |
| spec-planner | Prompt-to-spec expansion | opus |
| sprint-contract-negotiator | Done-criteria negotiation protocol | opus |

## Harness Pipeline

The 5 harness skills form a connected pipeline:

```
[User Prompt]
    |
    v
[spec-planner] --- Product spec
    |
    v
[sprint-contract-negotiator] --- Done criteria
    |
    v
[Generator] --- Implementation
    |
    +-- [qa-evaluator] --- Functional QA
    +-- [frontend-design-evaluator] --- Design QA
    |
    v
[multi-agent-orchestrator] orchestrates the full pipeline
```

## Quick Start

- **Individual skill**: invoke by name or trigger phrase (e.g., `/spec-planner` or "기획서 만들어줘")
- **Full pipeline**: invoke `multi-agent-orchestrator` with `--chrome` enabled
- **Chrome-dependent skills** (`qa-evaluator`, `frontend-design-evaluator`) require `--chrome` flag or `/chrome` command

## Skill Structure Convention

Each skill follows the `SKILLNAME/SKILL.md` pattern with an optional `references/` directory for supplementary material. See [generate-skills/references/frontmatter-spec.md](generate-skills/references/frontmatter-spec.md) for the full field reference.
