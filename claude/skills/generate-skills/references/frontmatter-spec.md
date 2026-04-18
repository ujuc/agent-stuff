---
source_url: https://code.claude.com/docs/en/skills
last_upstream_check: 2026-04-18
check_interval_days: 14
---

# YAML Frontmatter Field Specification

> Field rules and validation criteria for YAML frontmatter at the top of SKILL.md.
> Source: https://code.claude.com/docs/en/skills#frontmatter-reference
>
> Freshness is tracked by the YAML block above; `generate-skills` Step 0 reads
> `last_upstream_check` and only re-fetches when `today - last_upstream_check >
> check_interval_days`.

---

## Delimiter Rules

Frontmatter sits at the very top of the file, wrapped in `---` delimiters:

```yaml
---
name: my-skill
description: What this skill does. When to use it.
---
```

- The first `---` MUST be on **line 1** (no blank lines before it)
- The second `---` closes the frontmatter
- No whitespace before or after delimiters

---

## Field Reference

All fields are optional. Only `description` is recommended so Claude knows when to use the skill.

### `name`

Display name for the skill. If omitted, uses the directory name.

| Rule | Description |
|------|-------------|
| Format | Lowercase letters, numbers, and hyphens only (max 64 characters) |
| Pattern | `^[a-z0-9]+(-[a-z0-9]+)*$` |
| Folder match | Should match the skill directory name |
| Reserved prefixes | `claude-*`, `anthropic-*` |
| Example | `generate-skills`, `notion-setup`, `tdd-workflow` |

### `description`

What the skill does and when to use it. Claude uses this to decide when to apply the skill. If omitted, uses the first paragraph of markdown content.

| Rule | Description |
|------|-------------|
| Max length | Combined with `when_to_use`, truncated at **1,536 characters** in the skill listing |
| Recommended structure | **WHAT** (what it does) + **WHEN** (when to use it) |
| XML tags | `< >` forbidden |
| Language | Per project language policy (Korean or English) |

This is the primary field the system uses for natural language matching. Its quality directly affects trigger accuracy. Front-load the key use case — tail content may be truncated when the session has many skills.

### `when_to_use`

Additional context describing when Claude should invoke the skill — trigger phrases, example requests, domain hints. Appended to `description` in the skill listing and shares the 1,536-character cap.

```yaml
when_to_use: "Trigger phrases, example user utterances, when NOT to use this skill."
```

- Use this to separate concise WHAT (in `description`) from noisy trigger keywords.
- Keep Korean trigger phrases verbatim; the matcher compares against raw user input.

### `argument-hint`

Hint shown during autocomplete to indicate expected arguments.

```yaml
argument-hint: "[issue-number]"
```

- Example values: `[issue-number]`, `[filename] [format]`
- Appears in the `/` menu autocomplete UI

### `disable-model-invocation`

Set to `true` to prevent Claude from automatically loading this skill. Use for workflows you want to trigger manually with `/name`.

```yaml
disable-model-invocation: true
```

- Default: `false` (auto-trigger allowed)
- Recommended for destructive or high-cost operations

### `user-invocable`

Set to `false` to hide from the `/` menu. Use for background knowledge users should not invoke directly.

```yaml
user-invocable: false
```

- Default: `true`
- Use for reference/context skills that Claude should load automatically but users have no reason to call

### `allowed-tools`

Tools Claude can use without asking permission when this skill is active.

```yaml
# canonical (space-separated string, per official docs)
allowed-tools: Read Grep Glob

# or YAML list
allowed-tools:
  - Read
  - Grep
  - Bash(git add *)

# also accepted in practice (comma-separated with spaces) — de-facto convention
allowed-tools: Read, Grep, Bash(git status:*)
```

- Official spec lists **space-separated string** or **YAML list**; Claude Code also parses comma-separated-with-spaces, which is the convention used across most real-world skills
- Creates a scoped permission grant for the skill's duration; does not restrict which tools are callable, only which skip per-use approval
- Baseline permission settings still apply to tools not listed

### `model`

Model to use when this skill is active.

```yaml
model: opus
```

- Allowed values: `opus`, `sonnet`, `haiku`
- If unset, inherits from the current session model
- Use `opus` for complex workflows or creative tasks

### `effort`

Effort level when this skill is active. Overrides the session effort level.

```yaml
effort: max
```

- Options: `low`, `medium`, `high`, `xhigh`, `max` (availability depends on the model)
- Default: inherits from session

### `context`

Set to `fork` to run in a forked subagent context.

```yaml
context: fork
```

- The skill content becomes the prompt that drives the subagent
- The subagent does NOT have access to conversation history
- Only makes sense for skills with explicit task instructions (not pure guidelines)

### `agent`

Which subagent type to use when `context: fork` is set.

```yaml
context: fork
agent: Explore
```

- Options: built-in agents (`Explore`, `Plan`, `general-purpose`) or custom subagents from `.claude/agents/`
- If omitted, uses `general-purpose`
- Only meaningful when `context: fork` is set

### `hooks`

Hooks scoped to this skill's lifecycle. See Claude Code hooks documentation for configuration format.

```yaml
hooks:
  - event: on_skill_start
    command: echo "Skill started"
```

### `paths`

Glob patterns that limit when the skill is auto-activated by Claude. When set, the skill is loaded only when working with files matching the patterns. Uses the same format as path-specific memory rules.

```yaml
# comma-separated string
paths: "src/**/*.ts, tests/**/*.spec.ts"

# or YAML list
paths:
  - "src/**/*.ts"
  - "tests/**/*.spec.ts"
```

- Useful for domain-specific skills (e.g., "only load when touching Terraform files").
- Has no effect on `/skill-name` manual invocation.

### `shell`

Shell used for `` !`<command>` `` inline injections and ` ```! ` fenced blocks inside the skill.

```yaml
shell: powershell
```

- Options: `bash` (default) or `powershell`.
- `powershell` requires `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.

---

## Invocation Control Matrix

| Frontmatter | User can invoke | Claude can invoke | Context loading |
|-------------|----------------|-------------------|-----------------|
| (default) | Yes | Yes | Description always in context; full skill loads when invoked |
| `disable-model-invocation: true` | Yes | No | Description NOT in context; full skill loads when user invokes |
| `user-invocable: false` | No | Yes | Description always in context; full skill loads when invoked |

---

## String Substitutions

Skills support dynamic value substitution in skill content:

| Variable | Description |
|----------|-------------|
| `$ARGUMENTS` | All arguments passed when invoking the skill. If not present in content, arguments are appended as `ARGUMENTS: <value>` |
| `$ARGUMENTS[N]` | Access a specific argument by 0-based index (e.g. `$ARGUMENTS[0]` for first) |
| `$N` | Shorthand for `$ARGUMENTS[N]` (e.g. `$0` for first argument) |
| `${CLAUDE_SESSION_ID}` | Current session ID. Useful for logging or session-specific files |
| `${CLAUDE_SKILL_DIR}` | Directory containing the skill's SKILL.md. Use in bash injection to reference bundled scripts regardless of working directory |

### Example

```yaml
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.
```

Running `/fix-issue 123` replaces `$ARGUMENTS` with `123`.

---

## Validation Checklist

After writing frontmatter, verify:

- [ ] Line 1 is `---`
- [ ] Closing `---` exists
- [ ] If `name` is present: matches kebab-case pattern (`^[a-z0-9]+(-[a-z0-9]+)*$`)
- [ ] If `name` is present: matches folder name
- [ ] If `name` is present: does not start with `claude` or `anthropic`
- [ ] Combined `description` + `when_to_use` length ≤ 1,536 characters
- [ ] `description` has no XML tags (`< >`)
- [ ] `description` includes WHAT; WHEN lives in `description` or `when_to_use`
- [ ] If `context` is set: value is `fork`
- [ ] If `agent` is set: `context: fork` is also set
- [ ] If `allowed-tools` is set: space-separated string or YAML list (not comma-separated)
- [ ] If `paths` is set: glob patterns only apply to auto-activation, not manual `/` invocation
