# Skill Design Principles

> Three core principles that guide all skill creation.

---

## 1. Concise is Key

The context window is a shared resource. Every token has a cost.

**What to omit:**
- General knowledge Claude already knows (e.g., "Markdown uses # for headings")
- Generic filler ("This skill is helpful for...")
- Excessive preamble and qualifiers
- Supporting documents: README.md, CHANGELOG.md, CONTRIBUTING.md

**The test:** Does this token change Claude's behavior? If not, remove it.

---

## 2. Degrees of Freedom

Match the specificity of instructions to the nature of the task.

| Freedom | Instruction style | Best for |
|---------|-------------------|----------|
| Low | "ALWAYS use this exact format" | Format-critical, high-repetition tasks |
| Medium | "Follow this structure, adapt as needed" | Tasks with patterns but context variance |
| High | "Use your best judgment" | Creative tasks, highly variable context |

**Decision criteria:**

- **Task fragility**: Must the format be exact? → Low freedom
- **Context dependency**: Should results vary by situation? → High freedom
- **Repetition**: Does the same pattern repeat? → Low freedom

---

## 3. Progressive Disclosure

Split information into 3 tiers. Agents load only what they need.

### Tier 1: Metadata (always loaded)

- `name` + `description` fields only
- Used by the system for trigger detection
- Target: ~100 words or fewer

### Tier 2: SKILL.md body (loaded on trigger)

- Core workflow and execution instructions
- Hard limit: **5,000 words / 500 lines**
- Exceed this? Move content to Tier 3

### Tier 3: Bundled resources (loaded on demand)

- `references/`: detailed rules, examples, checklists
- `scripts/`: automation and validation scripts
- `assets/`: images, diagrams, PDFs
- No size limit

---

## What NOT to include

| Type | Example | Reason |
|------|---------|--------|
| Supporting docs | README.md, CHANGELOG.md | Not needed in skill folders |
| General knowledge | "Git is a version control system" | Claude already knows |
| Excessive qualifiers | "This powerful skill will..." | Token waste |
| Self-description | "This skill includes the following:" | Show with instructions instead |
