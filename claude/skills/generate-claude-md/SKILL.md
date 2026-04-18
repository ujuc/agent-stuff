---
name: generate-claude-md
description: 프로젝트용 CLAUDE.md, AGENTS.md, contributing-docs/, .claude/rules/ 파일을 발견 불가능 정보 원칙에 따라 생성하거나 업데이트한다.
when_to_use: "문서 생성/갱신 요청일 때. 트리거: '/generate-claude-md', 'CLAUDE.md 업데이트', 'AGENTS.md 갱신', 'rules 생성', 'contributing-docs 추가', 'update CLAUDE.md', 'refresh AGENTS.md'. 단일 파일 편집은 Edit 도구를 직접 쓰고 이 스킬을 호출하지 않는다."
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Agent, advisor
---

# CLAUDE.md Generator — Orchestrator

## Mode Detection

Inspect `$ARGUMENTS` and pick a branch:

- **Update mode**: `$ARGUMENTS` contains any of `업데이트`, `수정`, `갱신`, `update`, `refresh`
  → Stage 1 (re-analysis) → U1 (audit) → U2 (compare) → U3 (apply) → Stage 4 (verification)
- **Generate mode**: anything else
  → Stage 1 → Stage 2 → Stage 3 → Stage 4

### Target Identification

| Keyword in `$ARGUMENTS` | Target |
|-------------------------|--------|
| `CLAUDE.md` alone | Root CLAUDE.md only |
| `AGENTS.md` | AGENTS.md + contributing-docs/ |
| `rules` | `.claude/rules/` only |
| `업데이트` with no specific file name | All 5 file types |

If `$ARGUMENTS` is empty, run in generate mode against the current working directory.

---

## Generation Philosophy

Principles that govern every stage of this skill.

**Design principles** (references/karpathy-guidelines.md): think before acting, simplicity first, surgical precision, goal-driven execution.

**Content principles** (references/osmani-guidelines.md): include only undiscoverable information. AGENTS.md is a diagnostic list of problems that code has not yet solved.

**Performance evidence**: auto-generated context → success rate −2–3%, cost +20%. Human-written gotchas → success rate +4% (ETH Zurich). Every line must justify its existence.

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

**What to pass**: Stage 1 summary, Stage 2 answers, target file list, and the core principles from the four guideline files (karpathy, osmani, entry-router, SOUL).

**5 generation targets**: Root CLAUDE.md, AGENTS.md, contributing-docs/, nested CLAUDE.md, `.claude/rules/`. Generate only the applicable ones.

**Update mode**: run U3 (apply) from references/update-mode.md. Use the Edit tool for surgical modifications only — never regenerate entire files.

---

## Stage 4: Verification

**Reference**: references/stage4-verifier.md (10-item checklist, anti-patterns, Reviewer prompt).

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

4. **`model: opus` is an orchestrator hint, not a subagent default.** Agents spawned in Stage 1, 3, and 4 explicitly request `model: sonnet` to control cost. Do not assume a single model applies throughout the pipeline.

5. **`disable-model-invocation` is intentionally unset.** The skill is invasive (writes/edits several project files). Because it is registered in CLAUDE.md's Skills table, auto-invocation can still fire from vague user phrasing. If false positives become a problem, flip this flag on and rely on `/generate-claude-md` plus the Skills-table triggers.

6. **advisor() takes no parameters; the entire transcript is forwarded.** Calling it before Stage 1 results are visible is premature. Prefer calling it right after orchestrator-internal reasoning has crystallized.

7. **Update mode assumes existing files were generated by this skill.** Hand-crafted CLAUDE.md files with unusual structures may register as drift when they are intentional. Confirm with the user before removing sections that look "redundant" but carry project-specific meaning.

---

## Eval Criteria

Five binary checks for any generation or update run. Autoresearch may reuse these when optimizing autonomously.

```
EVAL 1: Mode routing
  Question: Does the run pick the correct branch (generate vs. update)
            based on $ARGUMENTS keywords, and identify the right target files?
  Pass: Chosen mode matches user intent; generated/modified file list
        matches declared targets.
  Fail: Wrong branch, or target file list drifts from stated intent.

EVAL 2: Discoverability discipline
  Question: Every line in the generated/modified output passes the
            "Can an agent discover this by reading the code?" test.
  Pass: No discoverable content included in CLAUDE.md, AGENTS.md,
        contributing-docs/, or rules/.
  Fail: One or more lines restate facts readable from package.json,
        source tree, or standard linter rules.

EVAL 3: Size budgets
  Question: Root CLAUDE.md ≤ 100 lines (hard 300), nested CLAUDE.md
            ≤ 50 lines (hard 100), individual rule file ≤ 50 lines.
  Pass: Produced files stay within soft limits, or within hard limits
        with a user-approved rationale.
  Fail: Any file exceeds the hard limit without user approval.

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
