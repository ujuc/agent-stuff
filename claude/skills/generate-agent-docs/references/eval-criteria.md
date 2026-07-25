# Eval Criteria — generate-agent-docs

Six binary checks for any generation or update run. Referenced from
SKILL.md; skill-improver / autoresearch / waza reuse these when optimizing
the skill autonomously. Keep each check binary (Pass/Fail) so runs are
scoreable without human judgment.

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

EVAL 2: Discoverability & placement discipline
  Question: Every line in the generated/modified output passes the
            "Can an agent discover this by reading the code?" test AND
            sits on the right side of the cross-harness split —
            harness-neutral project content in AGENTS.md /
            contributing-docs/, Claude-only content in CLAUDE.md (below
            the @AGENTS.md import) / rules/.
  Pass: No discoverable content anywhere; no Claude-only content
        (hooks, skills, plan mode, tool names) in AGENTS.md; no
        project-general content in CLAUDE.md.
  Fail: One or more lines restate facts readable from package.json,
        source tree, or standard linter rules — or content sits on the
        wrong side of the harness split.

EVAL 3: Size budgets
  Question: CLAUDE.md + imported AGENTS.md combined ≤ 100 lines soft /
            200 hard (official ceiling, source:
            claude-code-best-practices.md), nested CLAUDE.md
            ≤ 50 lines (hard 100), individual rule file ≤ 50 lines. Every
            retained line passes the prune test.
  Pass: Produced files stay within soft limits, or within hard limits
        with a user-approved rationale; no line fails the prune test.
  Fail: Any file exceeds the hard limit without user approval, or a line
        survives that would not cause a mistake if removed.

EVAL 4: Reference integrity
  Question: All cross-file references (CLAUDE.md → @AGENTS.md import,
            AGENTS.md → contributing-docs/, nested → parent,
            rules/ `paths` globs) resolve to existing paths.
  Pass: Every reference is a live path.
  Fail: Any reference is broken or a glob targets non-existent paths.

EVAL 5: Blind reviewer
  Question: When output is more than a single root CLAUDE.md, does the
            Phase 3 Reviewer return PASS on every criterion listed in
            its prompt (or were all FAILs resolved in later iterations)?
  Pass: Final Reviewer run returns PASS, or initial FAILs were resolved
        before the final output.
  Fail: Unresolved Reviewer FAILs at skill completion.

EVAL 6: Instruction-authoring constraints
  Question: Is every produced/retained line free of the patterns
            model-prompting-guides.md rejects — self-verification or
            re-check instructions (W1), commands about showing or
            suppressing reasoning (W2), severity/confidence filter bars
            (W5), effort or thinking configuration (D2) — and does every
            scoped rule name its scope rather than implying it (W3)?
  Pass: None of the four rejected patterns appears; every path-bound rule
        states its paths or lives in .claude/rules/ with `paths`.
  Fail: Any rejected pattern survives, or a rule's scope is left implied.
```
