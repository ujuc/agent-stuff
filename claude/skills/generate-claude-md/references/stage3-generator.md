# Stage 3: Generator

> Defines file generation rules for CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, and .claude/rules/. Adopts a single sub-agent execution model.
> Tier 2 reference — loaded during Stage 3 execution.

---

## Agent Definition

| Parameter         | Value           |
| ----------------- | --------------- |
| subagent_type     | general-purpose |
| model             | sonnet          |
| run_in_background | false           |

---

## What the Orchestrator Provides

The orchestrator passes the following inputs to this agent:

- **Stage 1 summary**: Detected project facts (tech stack, monorepo structure, submodules, existing files)
- **Stage 2 answers**: User decisions (which nested CLAUDE.md to generate, scope boundaries)
- **Target files**: List of files to create or update (Root CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, .claude/rules/)
- **Generation principles**: Condensed from the guideline files (SOUL, entry-router-guidelines)

---

## Common Writing Rules

Apply to all generated files. The authoritative ✅ include / ❌ exclude table and
the prune test live in `claude-code-best-practices.md` (live-fetched) — defer to
the freshest copy of it; the rules below are the operative shorthand.

- Do not include code snippets directly — use `file:line` references only
- **Discoverability test**: For every line, ask "Can an agent discover this by reading the code?" If yes, omit it
- **Prune test** (authoritative gate): ask "Would removing this cause Claude to make mistakes?" If not, cut it. Bloated files cause Claude to ignore real instructions
- No auto-generated summaries: Do not include LLM-generated summaries of code as-is
- A must-run-every-time rule (e.g., lint before commit) belongs in a **hook**, not a CLAUDE.md line — recommend the hook instead

---

## Section A: Root CLAUDE.md

### Generation Principles

- Include only universal content that applies to every session and every task
- Target line count: ~100 lines (soft). **Hard ceiling: 200 lines** — the official limit (claude-code-best-practices.md): files over 200 lines consume more context and reduce adherence. Past the ceiling, split into `.claude/rules/` or `@`-imports rather than letting CLAUDE.md sprawl
- Do not include code style rules (delegate to linters/formatters)
- Reference AGENTS.md only (do not reference contributing-docs/ directly)
- Include only **confirmed facts** from Stages 1 and 2. Exclude assumptions or "nice to have" items
- Before adding any instruction, ask: **"Will Claude make a mistake without this?"**

### Structure Template

```markdown
# Project Overview
(WHY: 1-2 lines describing project purpose — only if not already in README)

# Tech Stack
(WHAT: List only core technologies — only if not obvious from package.json/go.mod/etc. Omit section if obvious)

# Development Commands
(HOW: Build, test, lint commands — only if not in README/Makefile)

# Work Rules
(HOW: Universal rules for branching, commits, PRs)

# Behavioral Guidelines
(Undiscoverable project-specific constraints. E.g., "Always confirm before running DB migrations")

# References
- **[AGENTS.md](./AGENTS.md)** — Undiscoverable operational info, detailed guides
(Also list any subdirectories with nested CLAUDE.md)
```

**Pointer vs. `@import`** (claude-code-best-practices.md): Claude Code reads
CLAUDE.md, **not** AGENTS.md. Keep the AGENTS.md reference a markdown **link**
(read on demand) — do **not** use `@AGENTS.md`, which would load AGENTS.md in full
every session and defeat progressive disclosure. Use `@import` only for content
that genuinely belongs in every session. If the project relies on Claude Code
auto-reading AGENTS.md, surface the tradeoff and let the user choose.

### Do-NOT-Include List

- Code examples or syntax demonstrations
- Information directly readable from config files (package.json, go.mod, etc.)
- Rules that a linter or formatter already enforces
- Content duplicated in AGENTS.md or contributing-docs/

---

## Section B: AGENTS.md

### Overview

Project guide following the agents.md standard. Referenced from CLAUDE.md; points to detailed documents in contributing-docs/.

### Generation Principles

- Use a universal format accessible to all AI agents
- Implement progressive disclosure by referencing contributing-docs/
- **Treat AGENTS.md as a codesmell list**: each entry should ideally be resolved via code, linters, or CI. Remove entries when the underlying code improves
- If an existing AGENTS.md exists, re-apply the discoverability test to all entries and identify candidates for removal

### Structure

- YAML frontmatter: `name`, `description`, `version`, `standard`
- **Project Overview**: Project purpose (only if not in README)
- **Operational Gotchas**: Traps agents cannot discover from code (external system behavior, non-obvious ordering requirements, environment-specific constraints)
- **Non-Obvious Conventions**: Conventions not inferable from code patterns (only what linters do not enforce)
- **Build & Test Gotchas**: Non-obvious build/test requirements only (exclude standard commands)
- **Git Workflow**: Branch strategy, commit conventions (only if not in CONTRIBUTING.md)
- **Boundaries**: Always Do / Ask First / Never Do
- **Contributing Docs**: Reference section listing detailed documents in contributing-docs/

---

## Section C: contributing-docs/ Separate Documents

Generate as separate documents the detailed content referenced from AGENTS.md. Create only the documents applicable to the project:

- `contributing-docs/architecture.md`: Service structure, communication patterns, data flows
- `contributing-docs/building_the_project.md`: Detailed build/deployment procedures
- `contributing-docs/testing.md`: Test strategy, test data setup
- `contributing-docs/database.md`: Schema structure, migration procedures
- `contributing-docs/conventions.md`: Code conventions, naming rules (only what linters cannot enforce)
- `contributing-docs/behavioral.md`: Project-specific behavioral constraints (only if applicable)

Each separate document must also be concise and follow the common writing rules.

---

## Section D: Nested CLAUDE.md (Monorepo Packages / Submodules)

Generate for directories detected in Stage 1 and approved by the user in Stage 2.

### Generation Conditions

Generate **only** for directories that satisfy **all** of the following:

- Has its own package manager file, or is a git submodule
- Requires a different tech stack, build commands, or work rules from the root CLAUDE.md
- The user approved generation in Stage 2

### Generation Principles

Inherit principles from Section A, with the following additions:

- **Scope restriction**: Cover only context within this directory
- **No duplication**: Do not repeat content from the parent CLAUDE.md. Describe only differences
- **Parent reference**: Reference the parent CLAUDE.md by explicit relative path for shared rules
- **Target line count**: 50 lines or fewer. Hard limit: 100 lines
- **Self-contained title**: Begin with `# CLAUDE.md — {package/submodule name}`

### Structure Template

```markdown
# CLAUDE.md — {name}

(1 line: purpose/role of this directory)

## Tech Stack
(Only differences from parent. Omit section if identical)

## Development Commands
(Build/test/lint commands unique to this directory)

## Work Rules
(Only if there are rules different from the parent. Omit if none)

## References
- **[../CLAUDE.md](../CLAUDE.md)** — Project-wide common rules
(Reference local AGENTS.md if present; omit otherwise)
```

### Reference Path Rules

- Parent CLAUDE.md: Always use relative path (`../CLAUDE.md`)
- Submodule: Reference parent repository CLAUDE.md via URL or relative path
- Sibling directories: Do not reference directly (route through parent)

---

## Section E: .claude/rules/ Rule Files

Auto-injected path-scoped rule files loaded by Claude Code each session. If contributing-docs/ serves as detailed documentation for all AI agents and human developers, rules/ serves as behavior rules exclusive to Claude Code.

### Generation Conditions

Generate **only** when one or more of the following is found in Stages 1–2:

1. **Path scoping needed**: Rules exist that apply only to specific directories
2. **CLAUDE.md exceeds size limit**: Non-universal rules need extraction because 100 lines will be exceeded
3. **3+ independent concerns**: 3 or more unrelated rule groups are identified

### Generation Principles

- **Path scoping first**: Rules that can specify `globs` must always include globs
- **Minimize alwaysApply**: Rules needed in every session go in CLAUDE.md first. In rules/, `alwaysApply: true` only when CLAUDE.md size limit is exceeded
- **One concern per file**: Do not mix multiple concerns in a single file
- **File naming**: `{concern}.md` (e.g., `api-conventions.md`, `testing.md`, `database-safety.md`)
- **Size limit**: 50 lines or fewer per file
- **Discoverability test**: Inherit from common writing rules

### File Format

```markdown
---
description: (One-line description of this rule)
globs: ["src/api/**/*.ts"]    # Optional: path scoping
alwaysApply: false            # If true, always loaded in every session
---

(Rule content — undiscoverable information only)
```

### Role Distinction: contributing-docs/ vs rules/

| Dimension        | contributing-docs/                         | rules/                          |
| ---------------- | ------------------------------------------ | ------------------------------- |
| Audience         | All AI agents + human developers           | Claude Code only                |
| Load mechanism   | Referenced from AGENTS.md, read on demand  | Auto-injected each session      |
| Path scoping     | Not possible                               | Possible via globs              |
| Content          | Detailed documents (architecture, testing) | Short behavior rules            |
