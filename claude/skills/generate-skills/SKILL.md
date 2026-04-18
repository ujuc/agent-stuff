---
name: generate-skills
description: "Claude 스킬을 생성하거나 기존 스킬을 최신 spec에 맞게 업데이트한다. 스킬 만들어줘, 새 스킬 추가, 스킬 업데이트, 스킬 수정, generate-skills 요청 시 사용한다."
model: opus
disable-model-invocation: true
argument-hint: "[skill-name]"
---

# Skill creation / update workflow

## Mode detection

Inspect `$ARGUMENTS` to choose a mode:

- **Update mode**: `$ARGUMENTS` contains any of `업데이트`, `수정`, `update`
  → Run Step 0 → Steps U1–U3 → Step 5 (validation)
- **Create mode**: anything else
  → Run Step 0 → Steps 1–5

If `$ARGUMENTS` is empty, ask the user via AskUserQuestion which mode and which target skill.

---

## Design principles

Principles that guide every step. See `references/design-principles.md` for the full version.

1. **Concise is key.** The context window is shared. Don't restate things Claude already knows.
2. **Match degrees of freedom to task fragility** (low / medium / high specificity).
3. **Progressive disclosure**: split content across three tiers (metadata → body → bundled resources).
4. **Use subagents** wherever they protect the main context or unlock parallel work. See `references/subagent-guidelines.md` for the decision criteria.

---

## Language policy

Apply this to every skill created or updated through this workflow.

- **`description` field → Korean.** This is the only part the user sees. Korean trigger phrases are also matched against `$ARGUMENTS` and the user's natural utterances, so Korean wording is functional, not just stylistic.
- **SKILL.md body, references/, scripts/, init templates → English.** This content is read by the LLM. English is more token-efficient and avoids translation drift in instructional prose.
- **Keep Korean verbatim where it has functional value:** trigger keywords used for `$ARGUMENTS` matching (e.g., `업데이트`, `수정`), Korean usage examples in description-writing guides, Korean Conventional Commits examples, and any text the user is expected to read (e.g., user-facing summary blocks defined inside a skill).
- **When updating a legacy skill** that has Korean prose in the body or references, translate the prose to English while preserving the items above.

---

## Step 0: Spec sanity check (before anything else)

The official skills doc changes often. Verify it before generating.

### Procedure

1. WebFetch `https://code.claude.com/docs/en/skills`.
2. Extract the field list from the Frontmatter reference section.
3. Diff against the "Field Reference" section in `references/frontmatter-spec.md`.
4. **If changes are detected**: update `references/frontmatter-spec.md`, then proceed to Step 1.
5. **If unchanged**: proceed to Step 1.

### What to compare

- Field additions / removals / behavior changes
- String substitution changes
- Invocation control matrix changes

### Notes

- If WebFetch fails (network error etc.), keep the existing `references/frontmatter-spec.md` and proceed to Step 1.
- If the upstream change set is large, summarize it for the user and confirm before rewriting the local copy.

---

## Step 1: Capture the use case

Use AskUserQuestion to collect:

1. **Problem / scenario**: what concrete problem does this skill solve?
2. **Target tools**: which tools does it call (built-in tools, MCP servers, external CLIs)?
3. **Expected output**: what does running the skill produce (files, messages, code, ...)?
4. **Trigger phrases**: what does the user actually say when they want this skill?

Use the answers to first identify the **domain type** in `references/skill-types.md`, then pick a **structural pattern** from `references/patterns.md`:

| Pattern | Best fit | Freedom |
|---------|----------|---------|
| Linear workflow | Fixed sequence of steps | Low–medium |
| Interview-based | Requirements depend on user context | High |
| Tool orchestration | Combines several tools | Medium |
| Template fill | Produces a fixed-shape artifact | Low |
| Validation / review | Quality-checking existing artifacts | Medium |

Confirm the picked pattern (and the reason) with the user.

### Parallel exploration (optional)

After sending the AskUserQuestion, spawn an Explore subagent to survey existing skills while the user types. Follow `references/subagent-guidelines.md` → "Explore-1".

Skip when `$ARGUMENTS` already contains enough information.

---

## Step 2: Scaffold the structure

Follow `references/skill-structure.md`.

### Auto-init (preferred)

Run `scripts/init-skill.py`:

```bash
python3 agents/claude/skills/generate-skills/scripts/init-skill.py <skill-name> --path <target-path>
```

By default this creates only `SKILL.md`. If the skill needs Tier-3 resources, pass `--with-references`, `--with-scripts`, and/or `--with-assets`. Fill in the body in Steps 3–4.

### Manual scaffold (when init-skill.py is unavailable)

**Required:**

1. Create the skill folder (kebab-case).
2. Create `SKILL.md` (empty — Steps 3–4 will fill it).

**Optional (depending on Step 1 outcome):**

3. `references/` for detailed reference docs.
4. `scripts/` for utility scripts.
5. `assets/` for media.

### Checks

- Folder name is kebab-case.
- No `README.md` was created.
- Folder name does not start with `claude` or `anthropic`.

---

## Step 3: Write the frontmatter

Use `references/frontmatter-spec.md` together with `references/description-examples.md`.

### Procedure

1. Set `name` (recommended): same as folder, kebab-case. If omitted, the directory name is used.
2. Write `description` (recommended) using the **WHAT + WHEN** formula:
   - WHAT: what the skill does (from Step 1's problem / scenario).
   - WHEN: when it should trigger (from Step 1's trigger phrases).
   - If omitted, the first paragraph of the markdown body is used.
3. Decide optional fields by category:

   **Invocation control:**
   - `disable-model-invocation`: `true` for destructive or expensive skills.
   - `user-invocable`: `false` for background-knowledge skills (hides from the `/` menu).

   **Execution environment:**
   - `model`: `opus` for complex workflows; omit otherwise.
   - `effort`: set when a different effort level than the session default is needed (`low`, `medium`, `high`, `max`).
   - `context`: `fork` to run in an isolated subagent context.
   - `agent`: subagent type when `context: fork` is set (`Explore`, `Plan`, `general-purpose`, ...).

   **Tools / permissions:**
   - `allowed-tools`: comma-separated list of tools usable without confirmation while the skill is active.

   **Other:**
   - `argument-hint`: autocomplete hint (e.g., `[issue-number]`).
   - `hooks`: hooks scoped to the skill's lifecycle.

Mechanical checks (kebab-case, length, reserved prefix, etc.) are handled by `validate-skill.sh` in Step 5. Here, only check semantics: **does `description` contain both WHAT and WHEN?**

---

## Step 4: Write the instructions

Write the SKILL.md body following the pattern picked in Step 1.

### Reference-skill analysis (optional)

If Step 1 surfaced a similar-pattern skill worth studying, spawn an Explore subagent to dissect it in parallel with drafting. Fold the result into the draft. Follow `references/subagent-guidelines.md` → "Explore-2".

### Common rules

- **Be specific**: include runnable commands, exact paths, concrete acceptance criteria.
- **Handle errors**: list failure modes and how to recover.
- **Show examples**: input/output samples per step.
- **Name the tools**: state which tools are used (Read, Write, Bash, AskUserQuestion, ...).
- **Build a Gotchas section**: known failure points are the highest-value content in any skill. See `references/design-principles.md` principle 4.

### Pick an output pattern

When the output shape matters, see `references/output-patterns.md`:

- **Template Pattern**: when the output format must be exact.
- **Examples Pattern**: when input/output pairs convey the quality bar.

### Apply degrees of freedom

Pick instruction specificity per the freedom guide in `references/design-principles.md`.

### Size limits

- SKILL.md body: aim for ≤ 5,000 words.
- Over the limit? Move detail into `references/` and link with relative paths.

### Post-write checks

- Every `references/` path resolves to a real file.
- Instructions are verifiable (no fuzzy phrasing).
- No filler (no linter-style preaching, no speculation, no over-explaining).
- **Redundancy audit**: the body must not restate rules already enforced by dispatched agent definitions, sibling skills, or standard LLM knowledge. Run the audit in `references/redundancy-check.md` whenever the body references an agent file, overlaps with an existing skill, or exceeds 150 lines.

---

## Step U1: Inspect the target skill (update mode)

1. Extract the target skill path / name from `$ARGUMENTS`.
2. Read the target SKILL.md.
3. Parse frontmatter fields (name, description, optional fields).
4. Note whether `references/` and `scripts/` exist.
5. Count SKILL.md body lines.

If the target cannot be identified, ask via AskUserQuestion.

---

## Step U2: Compare against the latest spec (update mode)

Using the freshly verified `references/frontmatter-spec.md` from Step 0:

1. **Missing recommended fields**: warn when `name` or `description` is absent.
2. **Removed fields**: detect fields no longer in the official doc (e.g., `license`, `metadata`).
3. **New fields worth adopting**: suggest `context`, `agent`, `effort`, `allowed-tools` etc. when they would help.
4. **`description` quality**: WHAT + WHEN coverage, trigger phrasing.
5. **Structural health**: SKILL.md line count (500-line ceiling), whether content should be split into `references/`.
6. **Redundancy audit**: detect body content that duplicates dispatched agent definitions, sibling skills, or standard LLM knowledge. Follow `references/redundancy-check.md`. Typical findings: constraints mirrored between skill and agent, prompt templates restating agent rules, generic markdown conventions.

Summarize the comparison for the user and get approval for the update scope.

---

## Step U3: Apply updates (update mode)

For the approved scope, edit with the Edit tool:

1. Add / modify / remove frontmatter fields.
2. Rewrite `description` if needed.
3. Reshape body sections if needed.

Show the change to the user before each edit and confirm.
When done, proceed to Step 5 (validation).

---

## Step 5: Validate

### Automated checks

Run `scripts/validate-skill.sh`:

```bash
bash agents/claude/skills/generate-skills/scripts/validate-skill.sh <skill-directory>
```

If anything fails, return to the relevant step, fix, and re-run.

### Behavior evaluation (optional)

Define binary (yes/no) eval criteria that measure output quality.
Per `references/eval-guide.md`, write 3–6 yes/no checks under an `## Eval Criteria` section in SKILL.md or in a separate `evals.md`.
The autoresearch skill reuses these criteria when optimizing autonomously.

### Independent review (optional)

If the generated skill includes `references/` or `scripts/`, spawn a `general-purpose` agent to do a blind review. Follow `references/subagent-guidelines.md` → "Reviewer".

Skip for minimal skills (SKILL.md only) or when the user requested a quick build.

### Trigger review (the part automation cannot catch)

Automation only checks form. Trigger accuracy needs a human eye.

- Are the phrases users actually say present in `description`? (avoids under-triggering)
- Does `description` lead with overly generic words ("help", "manage") that would over-trigger? (avoids over-triggering)

When in doubt, follow the trigger-tuning guide in `references/review-checklist.md`.

### Registration

Once validation passes, add a row to the Skills table in the appropriate CLAUDE.md:

```markdown
| `skill-name` | trigger phrases | model |
```

### Distribution (optional)

For team-wide distribution, see `references/distribution-guide.md` — repo check-in vs. plugin marketplace, composing skills, and measuring usage.
