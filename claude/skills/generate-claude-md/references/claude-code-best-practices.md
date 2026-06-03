---
source_url: https://code.claude.com/docs/en/best-practices.md
secondary_source_url: https://code.claude.com/docs/en/memory.md
last_upstream_check: 2026-05-30
check_interval_days: 0  # 0 = fetch on every run (user preference: always live; the doc changes often). WebFetch caches per-URL for ~15 min, so this is cheap.
---

# Claude Code Best Practices — Authoritative CLAUDE.md Guidance

This file is the **authoritative source** for how the skill writes and verifies
CLAUDE.md / AGENTS.md / rules. It is the single authoritative
source for the include/exclude rule and the size budget; the research rationale
(ETH Zurich data) is inlined in SKILL.md's Generation Philosophy.

## How this file is used (live fetch first, cache as fallback)

The upstream Claude Code docs change frequently, so the orchestrator does **not**
treat the snapshot below as frozen truth. Policy: **fetch live on every run**
(`check_interval_days: 0`) — no cache-skip window — because freshness is the whole
point. WebFetch caches each URL for ~15 minutes, so repeated runs in one sitting
do not re-hit the network. At skill start (Stage 0 / Generation Philosophy load),
the orchestrator:

0. **Load WebFetch first**: it is a deferred tool — call `ToolSearch` with query
   `select:WebFetch` to load its schema before using it. `allowed-tools` only
   pre-grants permission; without the ToolSearch load the call errors out.
1. **WebFetch `source_url`** (and `secondary_source_url` when CLAUDE.md sizing or
   `/init` behavior is in scope) for the latest guidance.
2. On success: use the fetched content; if it differs materially from the cache
   below, update the cache and bump `last_upstream_check` to today.
3. On **any** failure (tool not loaded, offline, rate limit, layout change): fall
   back to the cached snapshot below and tell the user in one line — *"best-practices
   라이브 로드 실패, 캐시 사용 (last check: <date>)."*

The split across two pages is deliberate: the include/exclude table, prune-test,
and failure patterns live on `best-practices`; the 200-line budget, `/init`
behavior, and AGENTS.md loading live on `memory`.

---

## Cached snapshot (last verified 2026-05-30)

### ✅ Include / ❌ Exclude (source: best-practices)

| ✅ Include                                            | ❌ Exclude                                          |
| ---------------------------------------------------- | -------------------------------------------------- |
| Bash commands Claude can't guess                     | Anything Claude can figure out by reading code     |
| Code style rules that differ from defaults           | Standard language conventions Claude already knows |
| Testing instructions and preferred test runners      | Detailed API documentation (link to docs instead)  |
| Repository etiquette (branch naming, PR conventions) | Information that changes frequently                |
| Architectural decisions specific to your project     | Long explanations or tutorials                     |
| Developer environment quirks (required env vars)     | File-by-file descriptions of the codebase          |
| Common gotchas or non-obvious behaviors              | Self-evident practices like "write clean code"     |

### Prune test — the real gate (source: best-practices)

> Keep it concise. For each line, ask: *"Would removing this cause Claude to make
> mistakes?"* If not, cut it. Bloated CLAUDE.md files cause Claude to ignore your
> actual instructions!

This is the single most important verification criterion. Every produced line of
CLAUDE.md (and every retained line in update mode) must pass it.

### Size budget (source: memory)

> **Size**: target under 200 lines per CLAUDE.md file. Longer files consume more
> context and reduce adherence.

- Root CLAUDE.md: soft target ~100 lines (keep it tight), **hard ceiling 200**.
- "Files over 200 lines consume more context and may reduce adherence." When a
  file grows past the ceiling, split into path-scoped `.claude/rules/` or
  `@`-imports rather than letting CLAUDE.md sprawl.

### Imports (source: best-practices + memory)

> CLAUDE.md files can import additional files using `@path/to/import` syntax.

- Imported files are **expanded and loaded in full at launch** — `@import` aids
  organization but does **not** reduce context. Use it only for content that
  genuinely belongs in every session.
- This is why the skill keeps AGENTS.md as an on-demand pointer (markdown link),
  **not** an `@import`: AGENTS.md is progressive-disclosure detail, not
  every-session context.

### AGENTS.md loading (source: memory)

> Claude Code reads `CLAUDE.md`, not `AGENTS.md`.

- AGENTS.md is not auto-loaded. For Claude Code to read it every session, a
  CLAUDE.md must `@AGENTS.md`-import it or be a symlink to it.
- The skill's design intentionally does **not** auto-load AGENTS.md: CLAUDE.md
  links to it as a pointer so agents read it on demand. Preserve this — do not
  convert the pointer into an `@import` unless the user wants AGENTS.md in every
  session's context.
- `/init` in a repo with an existing AGENTS.md (or `.cursorrules` /
  `.windsurfrules`) reads it and incorporates relevant parts into the generated
  CLAUDE.md.

### Advisory vs. deterministic — convert rules to hooks (source: best-practices)

> Unlike CLAUDE.md instructions which are advisory, hooks are deterministic and
> guarantee the action happens.

Failure-pattern fix: *"If Claude already does something correctly without the
instruction, delete it or convert it to a hook."* When a candidate CLAUDE.md line
is really a must-run-every-time gate (e.g., run lint before commit), recommend a
hook instead of a CLAUDE.md line.

### `/init` behavior (source: memory) — the baseline this skill refines

> Run `/init` to generate a starting CLAUDE.md automatically. ... **If a CLAUDE.md
> already exists, `/init` suggests improvements rather than overwriting it.**
> Refine from there with instructions Claude wouldn't discover on its own.

- `/init` is a **user-only slash command** — this skill cannot invoke it
  programmatically. The integration is to **consume its output** (an existing
  CLAUDE.md) as the baseline and refine, matching the documented "/init then
  refine over time" workflow.
- `CLAUDE_CODE_NEW_INIT=1` enables an interactive multi-phase flow: `/init` asks
  which artifacts to set up (CLAUDE.md, skills, hooks), explores the codebase
  with a subagent, fills gaps via follow-up questions, and presents a reviewable
  proposal before writing. When the baseline came from this mode, it already did
  subagent exploration + interview — so the skill's refine pass should be **light**
  (discoverability filter, AGENTS.md / contributing-docs/ / rules/, blind review),
  not a full Stage 1 re-analysis.

### Over-specified CLAUDE.md — the failure to avoid (source: best-practices)

> **The over-specified CLAUDE.md.** If your CLAUDE.md is too long, Claude ignores
> half of it because important rules get lost in the noise. **Fix**: Ruthlessly
> prune. If Claude already does something correctly without the instruction,
> delete it or convert it to a hook.

This is the verifier's primary anti-pattern: long, noisy CLAUDE.md → reduced
adherence. Pruning is not cosmetic; it is correctness.
