# Skill folder & file structure rules

> Detailed rules for laying out a skill directory.

---

## Minimum structure

```
my-skill/
└── SKILL.md
```

Every skill must contain a single `SKILL.md`. It is the entry point.

## Extended structure

```
my-skill/
├── SKILL.md              # required: frontmatter + instructions
├── references/           # optional: detailed docs, examples, checklists
│   ├── api-guide.md
│   └── examples/
│       └── basic.md
├── scripts/              # optional: validation / utility scripts
│   └── validate.sh
└── assets/               # optional: images, diagrams, PDFs
    └── architecture.png
```

---

## Folder-name rules

- **kebab-case** required: `my-skill`, `notion-project-setup`, `generate-skills`.
- No uppercase, underscores, or spaces.
- Folder name **must match** the `name` field in SKILL.md frontmatter.
- Use a meaningful name: a reader should be able to guess what the skill does.

---

## Subfolder usage

### `references/`

Hold detailed reference docs that SKILL.md links to.

- Checklists, specs, example collections, ...
- **Required** when SKILL.md body exceeds 5,000 words.
- SKILL.md links via relative paths: `references/api-guide.md`.
- If a skill depends on a file outside the skill tree (project-root doc, another submodule), copy it into `references/` so the skill is self-contained.

### `scripts/`

Hold automation / validation scripts.

- Shell scripts, Python scripts, etc.
- Make them executable (`chmod +x`).
- When SKILL.md references a script, include the full invocation.

### `assets/`

Hold images, diagrams, PDFs, and other binary/media files.

- Visuals referenced from SKILL.md or references/.
- Prefer text when text can do the job.

---

## Progressive disclosure

A skill splits content across three tiers. Don't make the agent load everything at once.

### Tier 1: Metadata (always loaded)

- Frontmatter `name` + `description` only.
- Used by the system for trigger detection.
- Target: ~100 words or fewer.

### Tier 2: SKILL.md body (loaded on trigger)

- Core workflow and instructions.
- Hard limit: **5,000 words / 500 lines**.
- Over the limit → move into Tier 3.

### Tier 3: Bundled resources (loaded on demand)

- `references/` for detailed rules, examples, checklists.
- `scripts/` for automation / validation.
- `assets/` for images, diagrams, PDFs.
- No size limit.

---

## Forbidden items

| Item | Reason |
|------|--------|
| Including a `README.md` | Skill folders must not contain README.md. SKILL.md is the only entry point. |
| `claude` prefix (`claude-my-skill`) | Reserved namespace. |
| `anthropic` prefix (`anthropic-helper`) | Reserved namespace. |
| Folder name not matching `name` | Causes skill load failure. |
| Entry points other than SKILL.md | `main.md`, `index.md`, etc. are not allowed. |
