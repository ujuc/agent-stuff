---
name: generate-claude-md
description: 프로젝트용 CLAUDE.md, AGENTS.md, contributing-docs/, .claude/rules/ 파일을 발견 불가능 정보 원칙에 따라 생성하거나 업데이트한다.
when_to_use: "문서 생성/갱신 요청일 때. 트리거: '/generate-claude-md', 'CLAUDE.md 업데이트', 'AGENTS.md 갱신', 'rules 생성', 'contributing-docs 추가', 'update CLAUDE.md', 'refresh AGENTS.md'. 단일 파일 편집은 Edit 도구를 직접 쓰고 이 스킬을 호출하지 않는다."
group: docs
model: opus
allowed-tools: Read Write Edit Glob Grep Agent AskUserQuestion ToolSearch WebFetch TaskOutput advisor
---

# CLAUDE.md Generator — Orchestrator

Generate or refine project documentation — root CLAUDE.md, AGENTS.md,
contributing-docs/, nested CLAUDE.md, `.claude/rules/` — under one governing
rule: **document only what an agent cannot discover by reading the code.**

## Pipeline Map

Execute stages strictly in order. Update mode swaps in U1–U3
(references/update-mode.md) at the marked points but keeps the same order.

| Stage | Purpose | Executed by | Reference |
|-------|---------|-------------|-----------|
| 0 | Live-fetch guidance, route generate/update, pick targets | Orchestrator | this file |
| 1 | Analyze project; classify discoverable vs undiscoverable | 3 Explore agents (complex) or direct reads (simple); update adds U1 audit | references/stage1-analyzer.md |
| 2 | Interview user on unresolved items | Orchestrator via AskUserQuestion; update adds U2 drift report | this file + references/update-mode.md |
| 3 | Write files | 1 general-purpose agent; update mode: U3 surgical edits by orchestrator | references/stage3-generator.md |
| 4 | Verify: checklist → fix loop → blind review | sonnet subagents + advisor | references/stage4-verifier.md |

## Stage 0: Bootstrap & Routing

This skill is the **"refine over time"** layer on top of the built-in `/init`
command. `/init` is a user-only slash command — it cannot be invoked
programmatically — so this skill consumes its output (an existing CLAUDE.md)
as the baseline, exactly as the official docs prescribe: *"Run `/init` to
generate a starter CLAUDE.md ... then refine over time."*

### Step 0-1 — Load authoritative guidance (live fetch, loud fallback)

references/claude-code-best-practices.md is the **single authoritative
source** for the ✅ include / ❌ exclude table, the prune test (*"Would
removing this cause Claude to make mistakes? If not, cut it"*), the 200-line
ceiling, `@import` semantics, AGENTS.md loading, and the over-specified
CLAUDE.md failure pattern. Its upstream changes often, so fetch live on every
run:

1. Call `ToolSearch` with query `select:WebFetch`. WebFetch is a **deferred
   tool**: `allowed-tools` only pre-grants permission — until the schema is
   loaded, calling it fails with a validation error that is *not* a network
   error.
2. `WebFetch` the `source_url` in that file's frontmatter (plus
   `secondary_source_url` when CLAUDE.md sizing or `/init` behavior is in
   scope).
3. Success → use the fetched text; if it differs materially from the cached
   snapshot, update the cache and bump `last_upstream_check`.
4. **Any** failure (tool not loaded, offline, rate limit, layout change) →
   use the cached snapshot **and** tell the user in one line:
   *"best-practices 라이브 로드 실패, 캐시 사용 (last check: <date>)."*
   Never fall back silently.

### Step 0-2 — Route generate vs update

Two signals: keyword and whether a target CLAUDE.md exists.
**File existence overrides the keyword default.**

| Signal | Branch |
|--------|--------|
| `$ARGUMENTS` contains `업데이트` / `수정` / `갱신` / `update` / `refresh` | **Update mode** (U1→U3 refine path) |
| No keyword + target CLAUDE.md exists | **Update mode** — the existing file is the `/init` baseline; never regenerate |
| No keyword + no CLAUDE.md | **Generate mode** (full Stage 1→4), after the recommendation below |

- **No-baseline recommendation**: state in one line that no baseline was found
  and that running `/init` first (the official "/init then refine" workflow)
  is preferred, then ask whether to proceed with full generation now or
  re-invoke after `/init`. Render the prompt in the user's language. If the
  user proceeds, run Stage 1→4 as the standalone fallback.
- **Light refine for rich baselines**: if the existing CLAUDE.md came from
  `/init`'s `CLAUDE_CODE_NEW_INIT=1` flow (it already did subagent exploration
  + interview), skip heavy Stage 1 exploration and apply only this skill's
  differentiators: discoverability filter, AGENTS.md, contributing-docs/,
  rules/, blind review.

### Step 0-3 — Identify targets

| Keyword in `$ARGUMENTS` | Target |
|-------------------------|--------|
| `CLAUDE.md` alone | Root CLAUDE.md only |
| `AGENTS.md` | AGENTS.md + contributing-docs/ |
| `rules` | `.claude/rules/` only |
| `업데이트` with no specific file name | All 5 file types |

Empty `$ARGUMENTS` → apply Step 0-2 against the current working directory.

## Generation Philosophy

- **Undiscoverable information only.** AGENTS.md is a diagnostic list of
  problems the code has not yet solved. Research evidence: auto-generated
  context → success rate −2–3%, cost +20%; human-written gotchas → +4%
  (ETH Zurich). Every line must justify its existence. The operative
  include/exclude rule lives in the authoritative source (Step 0-1).
- **Code patterns are discoverable** — style rules are unnecessary; exclude
  them. Write instructions as verifiable success criteria.
- **Governance** (references/entry-router-guidelines.md): when
  autonomous-agent safeguards are required, reflect the Entry Router CORE
  rules in AGENTS.md Boundaries and CLAUDE.md behavioral guidelines.
- **Workflow-usage policy — document conditionally**: when Stage 1/2 reveal
  large-scale parallel/adversarial orchestration (eval harnesses,
  rule-compliance verification, claim-source cross-checking, bulk triage,
  multi-agent pipelines), have Section A emit its short "Workflow
  Orchestration" policy block. Otherwise **omit it** — it fails the prune
  test and burns the size budget.
- **Soul** (references/SOUL.md): the agent-identity seed used when generating
  project files — a static copy, not a pointer to the live identity file.

## Stage 1: Project Analysis

**Reference**: references/stage1-analyzer.md (complexity criteria, agent
prompt templates, merge protocol).

Detect package/build/test/lint config, repository structure
(monorepo/submodule), documentation/CI layout, and existing `.claude/rules/`
in the target directory.

- **Complex project** (any of: 3+ config file types, monorepo, submodules) →
  spawn 3 Explore agents (`model: sonnet`) in one message: config-explorer,
  structure-explorer, docs-explorer. Explore agents are **read-only** — each
  returns findings as its final message; collect from Agent tool results
  (TaskOutput for background runs).
- **Simple project** → read directly, no subagents.

Merge findings, classify each as discoverable vs undiscoverable, separate
facts from `[ASSUMPTION]`s, and present the summary to the user.

**advisor() gate ①**: monorepo with 5+ packages, 3+ submodules, or an
existing CLAUDE.md with complex structure → validate the analysis strategy.

**Effort note**: the orchestrator inherits the session model/effort — do not
pin `effort` in frontmatter. If analysis itself is the bottleneck on a very
large monorepo, suggest the user raise the session effort level and re-run.

## Stage 2: Interview (orchestrator only — do not delegate)

`AskUserQuestion` runs only in the main orchestrator context (Gotcha 1).
Ask only about items Stage 1 could not resolve:

- **WHY**: project purpose / role
- **WHAT**: monorepo package roles, submodule relationships, external service
  dependencies
- **HOW**: work rules / workflow, recurring agent mistakes, approval for
  nested CLAUDE.md files

Present candidate interpretations for ambiguous items and let the user
choose. Confirm every Stage 1 `[ASSUMPTION]`.

**Deep exploration (optional)**: while AskUserQuestion is pending and the
project is a large monorepo (5+ packages) with unresolved questions, spawn
Explore-Deep (`model: sonnet`) in the background. Skip when Stage 1 results
suffice.

**Non-interactive session**: if AskUserQuestion is unavailable (headless
run), skip the interview, generate from confirmed facts only, and list every
unresolved question in the final report. Never write assumptions into
generated files.

**Update mode**: run U1 (audit) and U2 (drift comparison) in this stage
(references/update-mode.md). Present the U2 comparison report and confirm the
update scope with the user.

**advisor() gate ②**: user answers contradict Stage 1 detection, or update
mode surfaces 10+ drift items.

## Stage 3: Generation

**Reference**: references/stage3-generator.md (dispatch prompt template,
per-file rules A–E, common writing rules).

Spawn one general-purpose agent (`model: sonnet`) using the dispatch prompt
template. The agent Reads the rule files itself; the orchestrator pastes into
the prompt only the live-fetched authoritative constraints, the Stage 1
summary, the Stage 2 answers, and the target list.

**5 possible targets**: root CLAUDE.md, AGENTS.md, contributing-docs/,
nested CLAUDE.md, `.claude/rules/`. Generate only the applicable ones.

**Update mode**: run U3 instead (references/update-mode.md) — the
orchestrator applies surgical Edits, one user-confirmed change at a time.
Never regenerate whole files.

## Stage 4: Verification

**Reference**: references/stage4-verifier.md (verifier dispatch template,
10-item checklist, anti-patterns, reviewer prompt). The checklist's
size/staleness items enforce the authoritative prune test and 200-line
ceiling from Step 0-1.

1. **Verifier** (`model: sonnet`): apply the 10-item checklist line by line
   via the dispatch template.
2. **Fix loop**: the orchestrator fixes FAIL items, then re-verifies.
   Maximum **3 verification runs total** (initial + up to 2 fix rounds);
   after that, report remaining FAILs to the user and proceed.
3. **Blind Reviewer** (`model: sonnet`, consults advisor): spawn when output
   exceeds a single root CLAUDE.md. Pass generated file contents **only** —
   no Stage 1/2 results, no verifier output (Gotcha 3). The Reviewer calls
   advisor() for an opus cross-model second opinion on low-confidence
   findings.

Report verification results to the user; for each FAIL, quote the line and
the reason.

**advisor() gate ③**: Reviewer FAIL persists after the 2 fix rounds.

## Advisor Escalation Summary

| # | When | Trigger |
|---|------|---------|
| ① | After Stage 1 | Monorepo 5+ packages, 3+ submodules, or complex existing CLAUDE.md |
| ② | During Stage 2 | User answer ↔ detection mismatch, or 10+ drift items in update mode |
| ③ | During Stage 4 | Reviewer FAIL persists after 2 fix rounds |

**When not to call advisor()**: simple project generation, 1–2 target files,
verification passes on the first run, or the user gave unambiguous
instructions.

## Red Flags — STOP

| You are about to… | Do instead |
|-------------------|------------|
| Regenerate an existing CLAUDE.md from scratch | Route to update mode — the existing file is the `/init` baseline |
| Call WebFetch before loading its schema | `ToolSearch` `select:WebFetch` first (Step 0-1) |
| Use the cached best-practices without saying so | Announce the fallback in one line |
| Give the blind Reviewer anything beyond the generated files | Generated file contents only |
| Apply an update-mode edit the user has not seen | Show the exact change; get per-file confirmation (U3) |
| Tell a Stage 1 Explore agent to write a file | Explore is read-only — findings return as final messages |
| Claim completion without Stage 4 output | Report checklist/reviewer results with quoted failures |

## Gotchas

Skill-specific pitfalls automation cannot catch. Update whenever a new edge
case is discovered.

1. **Stage 2 cannot be delegated to a subagent.** `AskUserQuestion` only runs
   in the main orchestrator context. Explore-Deep can overlap with the user's
   typing, but the question flow itself stays in the main agent.
2. **references/SOUL.md is a static seed copy, not the live identity file.**
   The live identity is `~/.config/dotrc/rules/SOUL.md` (`../rules/SOUL.md`).
   The bundled copy keeps generation reproducible across environments — do
   not substitute the live file at runtime.
3. **Blind Reviewer independence is the whole point.** If Phase 1/2 output or
   Stage 1/2 context leaks into the Reviewer prompt, the review becomes
   confirmation and the FAIL filter loses its value.
4. **`model: opus` is an orchestrator hint, not a pipeline default.** Stage
   1/3/4 subagents explicitly request `model: sonnet` for cost; the Stage 4
   Reviewer additionally consults advisor() (opus per `advisorModel`) on
   low-confidence findings. Model aliases (`opus`/`sonnet`) resolve to the
   current generation at runtime — never hardcode version IDs. `effort` is
   inherited from the session, never pinned in frontmatter.
5. **`disable-model-invocation` is intentionally unset.** The skill is
   invasive (writes/edits several project files); auto-invocation can still
   fire from vague phrasing via the Skills-table triggers. If false positives
   become a problem, flip the flag on and rely on `/generate-claude-md`.
6. **advisor() takes no parameters; the entire transcript is forwarded.**
   Calling it before the relevant results are visible in the transcript is
   premature. Call it right after the reasoning it should review has
   crystallized.
7. **Update mode may misread hand-crafted files as drift.** Unusual
   structures can be intentional. Confirm with the user before removing
   sections that look redundant but may carry project-specific meaning.
8. **Explore agents are read-only.** The Agent tool's `Explore` type has no
   Write/Edit tool. Collect Stage 1 / Explore-Deep findings from their final
   messages (Agent tool result, or TaskOutput for background runs) — never
   instruct them to write `.research/` files.

## Eval Criteria

references/eval-criteria.md defines 5 binary checks — mode routing,
discoverability discipline, size budgets, reference integrity, blind
review — for any generation or update run. skill-improver / autoresearch /
waza reuse them when optimizing this skill.
