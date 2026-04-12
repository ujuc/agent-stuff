# Update Mode Reference: U1–U3 Procedures

This file defines the three update-mode stages that replace the standard generation flow when existing documentation files are detected. Integration points:

- **U1** runs after Stage 1 (project re-analysis)
- **U2** runs during Stage 2 (planning)
- **U3** replaces Stage 3 (generation)

Stage 4 (verification) runs normally after U3 completes.

---

## U1: Audit (after Stage 1)

Determine the current state of existing generated files in the target directory.

### Procedure

1. **Detect existing files** using Glob/Read. For each file type, check:

   | File | Details to Check |
   |------|-----------------|
   | Root `CLAUDE.md` | Line count, section list (parse `#`/`##` headers), validity of reference paths |
   | `AGENTS.md` | Frontmatter fields, section list, validity of `contributing-docs/` references, presence of Boundaries section |
   | `contributing-docs/` | File list, line count per file, whether each file is referenced from `AGENTS.md` |
   | Nested `CLAUDE.md` | Location, line count, validity of parent reference paths, content overlap with parent `CLAUDE.md` |
   | `.claude/rules/` | File list, `description`/`globs`/`alwaysApply` per file, content overlap with `CLAUDE.md` |

2. **Present audit summary table** to the user:

   | File | Lines | Status | Notes |
   |------|-------|--------|-------|
   | `CLAUDE.md` | N | exists | N sections |
   | `AGENTS.md` | N | exists | N sections, contributing-docs refs valid |
   | `contributing-docs/architecture.md` | N | exists | referenced from AGENTS.md |
   | `.claude/rules/typescript.md` | N | exists | globs: `**/*.ts` |
   | … | … | … | … |

3. **If no files are found**: ask the user via AskUserQuestion:
   > "No existing documentation files were found. Would you like to switch to generation mode?"
   Stop and wait for the response.

### Parallelization

The Stage 1 Explore agent and the U1 file reads can run in parallel. However, if there are 3 or fewer target files, reading them directly is more efficient than spawning parallel sub-agents.

---

## U2: Drift Comparison (during Stage 2)

Cross-reference Stage 1 re-analysis results against U1 audit results using three axes.

### Axis 1: Codebase Drift

Compare the current codebase state (Stage 1 results) against existing documentation content:

| File Type | Compare Against | Drift Criteria |
|-----------|----------------|----------------|
| Root `CLAUDE.md` | Tech stack vs. actual config files | Technology added, removed, or version changed |
| Root `CLAUDE.md` | Dev commands vs. actual `scripts/`/`Makefile` | Commands changed, added, or removed |
| `AGENTS.md` | Operational Gotchas vs. current code | Resolved gotchas still present; new gotchas needed |
| `contributing-docs/` | Each doc's content vs. actual structure/config | Structure or strategy changed |
| Nested `CLAUDE.md` | Directory-specific tech/commands vs. documentation | Subdirectory changed |
| `.claude/rules/` | `globs` patterns vs. actual file paths | Scoped paths no longer exist |

### Axis 2: Principle Re-application

Re-apply the Stage 3 generation philosophy to every line of existing files:

- **Discoverability test**: Identify items that are now discoverable through improved code (and therefore should be removed)
- **Size constraint**: Flag if Root `CLAUDE.md` exceeds 100 lines or any nested `CLAUDE.md` exceeds 50 lines
- **Staleness risk**: Flag specific version numbers, tool names, or dependency names that no longer match the current state
- **Redundancy check**: Identify content duplicated across files (`CLAUDE.md` ↔ `rules/`, parent ↔ nested, `AGENTS.md` ↔ `contributing-docs/`)

### Axis 3: Structural Integrity

Validate cross-file reference relationships:

- `CLAUDE.md` → `AGENTS.md` reference path resolves correctly
- `AGENTS.md` → `contributing-docs/` references match actual files present
- Nested `CLAUDE.md` → parent reference path is correct
- `.claude/rules/` `globs` patterns point to paths that actually exist

### Comparison Report Format

Present comparison results grouped by category:

```
## Drift Comparison Results

### Change Required
| # | File | Item | Reason | Recommended Action |
|---|------|------|--------|--------------------|
| 1 | CLAUDE.md | Tech stack section | Bun added, Node removed | Update stack entry |

### Remove Recommended
| # | File | Item | Reason |
|---|------|------|--------|
| 1 | .claude/rules/legacy.md | Entire file | globs target paths no longer exist |

### Add Candidate
| # | File | Item | Reason |
|---|------|------|--------|
| 1 | contributing-docs/ | testing-strategy.md | New test framework detected (Vitest) |

### No Change
(N items require no changes)
```

After presenting the report, ask the user via AskUserQuestion:

> "Which of the above items would you like to apply? (all / select / by file)"

Wait for the user's selection before proceeding to U3.

---

## U3: Apply (instead of Stage 3)

Apply user-approved changes using surgical edits.

### Apply Principles

- **Surgical changes only**: Use the Edit tool. Modify only lines that require changes. Do not regenerate entire files.
- **Apply order** (leaf-first, respecting reference dependencies):
  1. `contributing-docs/` individual files
  2. `.claude/rules/` individual files
  3. Nested `CLAUDE.md` files
  4. `AGENTS.md` (including updating `contributing-docs/` references)
  5. Root `CLAUDE.md` (including updating `AGENTS.md` references)
- **Before each file edit**: Show the exact change to the user and get confirmation before applying.
- **File deletion**: Propose only — do not delete without explicit user approval.
- **File addition**: When new files are needed, follow Stage 3 generation rules to write them.

After all approved changes are applied, proceed to Stage 4 (verification).
