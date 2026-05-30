---
name: generate-claude-md
description: 프로젝트용 CLAUDE.md, AGENTS.md, contributing-docs/, .claude/rules/ 파일을 발견 불가능 정보 원칙에 따라 생성하거나 업데이트한다.
when_to_use: "문서 생성/갱신 요청일 때. 트리거: '/generate-claude-md', 'CLAUDE.md 업데이트', 'AGENTS.md 갱신', 'rules 생성', 'contributing-docs 추가', 'update CLAUDE.md', 'refresh AGENTS.md'. 단일 파일 편집은 Edit 도구를 직접 쓰고 이 스킬을 호출하지 않는다."
group: docs
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Agent, WebFetch, advisor
---

# CLAUDE.md Generator — Orchestrator

## Stage 0: Bootstrap & Routing

This skill is the **"refine over time"** layer on top of the built-in `/init`
command. `/init` is a user-only slash command — it **cannot** be invoked
programmatically here — so the integration is to **consume its output** (an
existing CLAUDE.md) as the baseline, exactly as the official docs prescribe:
*"Run `/init` to generate a starter CLAUDE.md ... then refine over time."* See
references/claude-code-best-practices.md.

### Routing precedence

Decide the branch from two signals — keyword **and** whether a target CLAUDE.md
already exists. **File existence overrides the keyword default:**

| Signal | Branch |
|--------|--------|
| `$ARGUMENTS` contains `업데이트` / `수정` / `갱신` / `update` / `refresh` | **Update mode** (refine path) |
| No keyword **+ target CLAUDE.md exists** | **Update mode** (treat the existing file as the `/init` baseline) |
| No keyword **+ no CLAUDE.md** | **Generate mode** (full Stage 1→4), after the `/init` recommendation below |

- **Refine path = the existing update-mode** (U1 audit → U2 drift → U3 surgical
  edit). It is *not* a new mechanism. Route every "file exists" case through it.
- **No-baseline recommendation**: when no CLAUDE.md exists, surface one line
  before generating — *"베이스라인이 없습니다. `/init`으로 스타터 CLAUDE.md를
  만든 뒤 이 스킬로 정제하길 권장합니다(공식 워크플로). 지금 풀 생성으로
  진행할까요, 아니면 `/init` 실행 후 다시 호출할까요?"* If the user proceeds, run
  full generation (Stage 1→4) as the standalone fallback.
- **Light refine when the baseline is rich**: if the existing CLAUDE.md came from
  `/init`'s `CLAUDE_CODE_NEW_INIT=1` flow (it already did subagent exploration +
  interview), skip the heavy Stage 1 deep-explore. Apply only the skill's
  differentiators: discoverability filter, AGENTS.md, contributing-docs/, rules/,
  blind review.

### Target Identification

| Keyword in `$ARGUMENTS` | Target |
|-------------------------|--------|
| `CLAUDE.md` alone | Root CLAUDE.md only |
| `AGENTS.md` | AGENTS.md + contributing-docs/ |
| `rules` | `.claude/rules/` only |
| `업데이트` with no specific file name | All 5 file types |

If `$ARGUMENTS` is empty, apply the routing precedence above against the current
working directory.

---

## Generation Philosophy

Principles that govern every stage of this skill.

**Authoritative guidance** (references/claude-code-best-practices.md — **live-fetched**): the official Anthropic ✅ include / ❌ exclude table, the prune test (*"Would removing this cause Claude to make mistakes? If not, cut it"*), the 200-line ceiling, `@import` semantics, AGENTS.md loading, and the "over-specified CLAUDE.md" failure pattern. On skill start, **first load WebFetch with `ToolSearch` (query `select:WebFetch`) — it is a deferred tool and is not callable until its schema is loaded, even though `allowed-tools` pre-grants permission — then** WebFetch the upstream URLs in that file's frontmatter. On **any** failure (tool not loaded, offline, rate limit, layout change), fall back to its cached snapshot and tell the user in one line. This source **supersedes** osmani for the include/exclude rule and the size budget.

**Design principles** (references/karpathy-guidelines.md): think before acting, simplicity first, surgical precision, goal-driven execution.

**Content principles & research rationale** (references/osmani-guidelines.md): the *why* behind "include only undiscoverable information" — AGENTS.md is a diagnostic list of problems code has not yet solved. Performance evidence: auto-generated context → success rate −2–3%, cost +20%; human-written gotchas → +4% (ETH Zurich). Every line must justify its existence. (For the operative include/exclude rule itself, defer to the authoritative source above.)

**Governance principle** (references/entry-router-guidelines.md): when autonomous-agent safeguards are required, reflect the Entry Router CORE rules in AGENTS.md Boundaries and CLAUDE.md behavioral guidelines.

**Soul** (references/SOUL.md): the foundation of agent identity and attitude. This is a static seed copy used when generating project files — not a pointer to `$GYEOL_HOME/SOUL.md`.

**LLM context**: LLMs are in-context learners. Retrieving code patterns is enough to follow style, so style rules are unnecessary. High-level errors compound geometrically downstream. Write instructions as verifiable success criteria.

---

## Stage 1: Project Analysis

**Reference**: references/stage1-analyzer.md (full procedure, including agent prompts).

Detect package/build/test/lint config, repository structure (monorepo/submodule), documentation/CI layout, and existing `.claude/rules/` in the target directory.

**Complexity judgment**: 3+ config file types, monorepo, or submodules present → complex project.

- **Complex project**: spawn 3 Explore agents (`model: sonnet`) — Explore-Config, Explore-Structure, Explore-Docs
- **Simple project**: detect directly, no subagent

Classify findings as discoverable vs. undiscoverable and present them to the user. Distinguish facts from assumptions.

**advisor() call condition ①**: Stage 1 reveals a monorepo with 5+ packages, 3+ submodules, or an existing CLAUDE.md with complex structure → call advisor() to validate the analysis strategy.

**Effort note (Opus 4.8)**: the orchestrator inherits the session model/effort — do not hardcode `effort` in frontmatter. For very large monorepos where the analysis itself is the hard part, suggest the user raise effort (`/effort xhigh`, available on Opus 4.8/4.7) before re-running, rather than forcing it.

---

## Stage 2: Interview (run directly — cannot delegate to subagents)

Ask only about items Stage 1 could not resolve:

- **WHY**: project purpose / role
- **WHAT**: monorepo package roles, submodule relationships, external service dependencies
- **HOW**: work rules / workflow, whether agents make repeated mistakes, approval for generating nested CLAUDE.md files

For ambiguous items, present candidate interpretations and ask the user to choose. Confirm Stage 1 assumptions with the user.

**Deep exploration (optional)**: while `AskUserQuestion` is pending and the project is a large monorepo (5+ packages) with unresolved questions, spawn an Explore-Deep agent (`model: sonnet`) in the background. Skip when Stage 1 results are sufficient.

**Update mode**: integrate U1 (audit) and U2 (compare) from references/update-mode.md into this stage. Present the U2 comparison report and confirm the update scope with the user.

**advisor() call condition ②**: the user's answers contradict Stage 1 detection, or update mode surfaces 10+ drift items.

---

## Stage 3: Generation

**Reference**: references/stage3-generator.md (per-file rules A–E, common writing rules).

Spawn a single general-purpose agent (`model: sonnet`) to generate files.

**What to pass**: Stage 1 summary, Stage 2 answers, target file list, and the core principles from the guideline files — the **authoritative include/exclude table + prune test + 200-line budget** (references/claude-code-best-practices.md, live-fetched result preferred), plus karpathy, osmani, entry-router, and SOUL.

**5 generation targets**: Root CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, `.claude/rules/`. Generate only the applicable ones.

**Update mode**: run U3 (apply) from references/update-mode.md. Use the Edit tool for surgical modifications only — never regenerate entire files.

---

## Stage 4: Verification

**Reference**: references/stage4-verifier.md (10-item checklist, anti-patterns, Reviewer prompt). The checklist's size/staleness items enforce the authoritative prune test and 200-line ceiling from references/claude-code-best-practices.md.

Three-phase pipeline:

1. **Verifier**: apply the 10-item checklist line by line (`model: sonnet`)
2. **Iterative Fix**: fix failing items and re-verify (up to 2 iterations)
3. **Reviewer**: if the output exceeds a single CLAUDE.md, spawn a blind independent review agent (`model: sonnet`). Do not pass Phase 1/2 results to it.

Report verification results to the user. For failing items, quote the line and the reason.

**advisor() call condition ③**: Reviewer returns FAIL and 2 orchestrator fix iterations still cannot reach PASS.

---

## Advisor Escalation Summary

| # | When | Trigger |
|---|------|---------|
| ① | After Stage 1 | Monorepo 5+ packages, 3+ submodules, or complex existing CLAUDE.md |
| ② | During Stage 2 | User answer ↔ detection mismatch, or 10+ drift items in update mode |
| ③ | During Stage 4 | Reviewer FAIL followed by 2 fix iterations still not PASS |

**When not to call advisor()**: simple project generation, 1–2 target files, verification passes on the first iteration, or the user gave unambiguous instructions.

---

## Gotchas

Skill-specific pitfalls that automation cannot catch. Update whenever a new edge case is discovered.

1. **Stage 2 cannot be delegated to a subagent.** It requires `AskUserQuestion`, which only runs in the main orchestrator context. Explore-Deep can overlap with the user's typing, but the question flow itself must stay in the main agent.

2. **`references/SOUL.md` is a static seed copy, not the live identity file.** The user's identity lives at `$GYEOL_HOME/SOUL.md`. The copy bundled here is a frozen snapshot so generation is reproducible across environments. Do not substitute `$GYEOL_HOME/SOUL.md` at runtime.

3. **Blind Reviewer must receive no orchestrator context.** If Phase 1 or Phase 2 output leaks into the Reviewer prompt, the review stops being independent and the FAIL filter loses its value. Only generated file contents should be passed in.

4. **`model: opus` is an orchestrator hint, not a subagent default.** Agents spawned in Stage 1, 3, and 4 explicitly request `model: sonnet` to control cost. Do not assume a single model applies throughout the pipeline. Model aliases (`opus`/`sonnet`) resolve to the current generation at runtime (Opus 4.8 today) — never hardcode version IDs like `claude-opus-4-8`. Likewise `effort` is inherited from the session, not pinned in frontmatter; Opus 4.8 exposes `xhigh` via `/effort` for the user to opt into.

5. **`disable-model-invocation` is intentionally unset.** The skill is invasive (writes/edits several project files). Because it is registered in CLAUDE.md's Skills table, auto-invocation can still fire from vague user phrasing. If false positives become a problem, flip this flag on and rely on `/generate-claude-md` plus the Skills-table triggers.

6. **advisor() takes no parameters; the entire transcript is forwarded.** Calling it before Stage 1 results are visible is premature. Prefer calling it right after orchestrator-internal reasoning has crystallized.

7. **Update mode assumes existing files were generated by this skill.** Hand-crafted CLAUDE.md files with unusual structures may register as drift when they are intentional. Confirm with the user before removing sections that look "redundant" but carry project-specific meaning.

8. **An existing CLAUDE.md is the `/init` baseline, and its existence overrides the keyword default.** No update keyword + a CLAUDE.md already on disk → route to update mode (refine), never regenerate from scratch. This both matches the "/init then refine" workflow and fixes the old "empty arguments → generate mode" path that would clobber an existing file. `/init` itself cannot be called programmatically — only its output is consumed.

9. **best-practices is live-fetched, so a fetch failure must degrade loudly, not silently.** WebFetch is a **deferred tool** — load its schema with `ToolSearch select:WebFetch` before the first call, or the call fails with a validation error that is *not* a network failure. Treat **any** failure (not-loaded, offline, rate limit, layout change) the same way: fall back to the cached snapshot in references/claude-code-best-practices.md **and** say so in one line. Silently using a stale cache hides that the authoritative guidance may have moved.

---

## Eval Criteria

Five binary checks for any generation or update run. Autoresearch may reuse these when optimizing autonomously.

```
EVAL 1: Mode routing
  Question: Does the run pick the correct branch per the Stage 0 routing
            precedence — keyword OR an existing CLAUDE.md routes to update
            (refine), no keyword + no CLAUDE.md routes to generate after
            the /init recommendation — and identify the right target files?
  Pass: Chosen branch matches the precedence table (file existence overrides
        the keyword default); generated/modified file list matches targets.
  Fail: An existing CLAUDE.md was regenerated from scratch, wrong branch,
        or target file list drifts from stated intent.

EVAL 2: Discoverability discipline
  Question: Every line in the generated/modified output passes the
            "Can an agent discover this by reading the code?" test.
  Pass: No discoverable content included in CLAUDE.md, AGENTS.md,
        contributing-docs/, or rules/.
  Fail: One or more lines restate facts readable from package.json,
        source tree, or standard linter rules.

EVAL 3: Size budgets
  Question: Root CLAUDE.md ≤ 100 lines soft / 200 hard (official ceiling,
            source: claude-code-best-practices.md), nested CLAUDE.md
            ≤ 50 lines (hard 100), individual rule file ≤ 50 lines. Every
            retained line passes the prune test.
  Pass: Produced files stay within soft limits, or within hard limits
        with a user-approved rationale; no line fails the prune test.
  Fail: Any file exceeds the hard limit without user approval, or a line
        survives that would not cause a mistake if removed.

EVAL 4: Reference integrity
  Question: All cross-file references (CLAUDE.md → AGENTS.md,
            AGENTS.md → contributing-docs/, nested → parent,
            rules/ globs) resolve to existing paths.
  Pass: Every reference is a live path.
  Fail: Any reference is broken or a glob targets non-existent paths.

EVAL 5: Blind reviewer
  Question: When output is more than a single root CLAUDE.md, does
            Phase 3 Reviewer return PASS on all 7 criteria (or all
            FAILs are resolved in subsequent iterations)?
  Pass: Final Reviewer run returns PASS, or initial FAILs were resolved
        before the final output.
  Fail: Unresolved Reviewer FAILs at skill completion.
```
