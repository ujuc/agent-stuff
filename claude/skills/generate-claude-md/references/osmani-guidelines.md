# Osmani AGENTS.md Guidelines

Source: https://addyosmani.com/blog/agents-md/

## Core Philosophy

Include only information that is **undiscoverable and operationally critical**.

The goal is to capture what agents genuinely cannot figure out from reading code — not
to document everything, but to document only what would otherwise cause repeated failure.

## Performance Research (ETH Zurich)

Evidence base for the "less is more" principle:

| Context Type          | Success Rate Impact | Cost Impact |
| --------------------- | ------------------- | ----------- |
| Auto-generated        | −2 to −3%           | +20%+       |
| Human-written gotchas | +4%                 | minimal     |

**Implication**: Every line must justify its existence. Auto-summarized content actively
harms agent performance. Only manually curated, operationally specific information helps.

## Discoverability Test

Before including any line, ask: **"Can an agent discover this by reading the code?"**

**Include** (undiscoverable):
- External system behaviors (third-party API quirks, cloud resource constraints)
- Tribal knowledge not encoded in code (why a workaround exists, history of a decision)
- Non-obvious ordering requirements (must run X before Y, never run A and B in same session)
- Environmental gotchas (specific version conflicts, OS-level dependencies)
- Business rules that look arbitrary but have regulatory/legal reasons

**Exclude** (discoverable):
- Directory structure (agent can `ls`/`tree`)
- Code style patterns (agent reads existing code and infers)
- Technology stack (visible in package.json, go.mod, pyproject.toml, etc.)
- Standard build/test commands already in README or Makefile
- Linter-enforced rules (agent sees them fail and learns)

## Maintenance-First Philosophy

When an agent repeatedly fails at a task, prefer fixing the **root cause in code** over
adding more documentation:

1. **Can the code be restructured** so the mistake is impossible? → Refactor first
2. **Can a linter or CI check** catch and enforce the constraint? → Add the check
3. **Only if neither is feasible**: add a targeted AGENTS.md entry

AGENTS.md items are a **diagnostic list of unsolved problems**, not permanent features.
Each entry should eventually be resolved by improving the codebase itself.

## Anti-Patterns

Five patterns that reduce agent performance:

### 1. Static Instructions for Dynamic Contexts
Unconditional rules that apply regardless of task type.
- Bad: "Always use TypeScript strict mode"
- Better: "When adding new files: use TypeScript strict mode. When editing legacy JS: match existing style."

### 2. Information Redundancy
Content already present in README, CONTRIBUTING.md, or CI configuration.
Duplicated information creates maintenance debt and confusion when sources diverge.

### 3. Over-reliance on Auto-generation
LLM-summarized content of the codebase inserted verbatim.
Agents already read code — summarizing it back wastes context and hurts performance.

### 4. Stale Documentation
Instructions referencing old tool versions, deprecated APIs, or removed patterns.
Stale context is worse than no context: agents follow instructions that lead to failure.

### 5. Single-File Overloading
Cramming architecture, conventions, build details, and behavioral rules into one file.
Use the AGENTS.md → contributing-docs/ progressive disclosure pattern instead.
