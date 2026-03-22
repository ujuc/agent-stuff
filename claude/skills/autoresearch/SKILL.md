---
name: autoresearch
description: "Autonomously optimize any Claude Code skill by running it repeatedly, scoring outputs against binary evals, mutating the prompt, and keeping improvements. Based on Karpathy's autoresearch methodology. Use when: 스킬 최적화, 스킬 개선, autoresearch, run autoresearch on, make this skill better, benchmark skill, eval my skill."
model: opus
disable-model-invocation: true
argument-hint: "[skill-name]"
---

# Autoresearch for Skills

Adapts Andrej Karpathy's autoresearch methodology (autonomous experimentation loops) to Claude Code skills. Instead of optimizing ML training code, we optimize skill prompts.

---

## The Core Job

Take any existing skill, define what "good output" looks like as binary yes/no checks, then run an autonomous loop that:

1. Generates outputs from the skill using test inputs
2. Scores every output against the eval criteria
3. Mutates the skill prompt to fix failures
4. Keeps mutations that improve the score, discards the rest
5. Repeats until the score ceiling is hit or the budget is exhausted

**Output:** An improved SKILL.md + `results.tsv` log + `changelog.md` of every mutation attempted.

---

## Before Starting: Gather Context

**STOP. Do not run any experiments until all fields below are confirmed with the user via AskUserQuestion.**

1. **Target skill** — Exact path to SKILL.md
2. **Test inputs** — 3-5 different prompts/scenarios covering different use cases (avoid overfitting to one scenario)
3. **Eval criteria** — 3-6 binary yes/no checks defining a good output (see [references/eval-guide.md](references/eval-guide.md))
4. **Runs per experiment** — How many times to run per mutation. Default: 5
5. **Budget cap** — Max number of experiment cycles. Default: 20

If the target skill already has eval criteria (in `## Eval Criteria` section or `evals.md`), present them to the user and ask whether to reuse, modify, or replace.

---

## Step 1: Read the Skill

Before changing anything, read and understand the target skill completely.

1. Read the full SKILL.md
2. Read any files in `references/` that the skill links to
3. Identify the skill's core job, process steps, and output format
4. Note any existing quality checks or anti-patterns

Do NOT skip this. You need to understand what the skill does before you can improve it.

---

## Step 2: Build the Eval Suite

Convert the user's eval criteria into structured tests. Every check must be binary — pass or fail.

**Format each eval as:**

```
EVAL [number]: [Short name]
Question: [Yes/no question about the output]
Pass condition: [What "yes" looks like — be specific]
Fail condition: [What triggers a "no"]
```

**Rules:**
- Binary only. No scales.
- Specific enough that two reviewers reach the same answer independently.
- Not so narrow the skill games the eval.
- 3-6 evals is the sweet spot.

See [references/eval-guide.md](references/eval-guide.md) for detailed examples.

**Max score calculation:**
```
max_score = [number of evals] × [runs per experiment]
```

---

## Step 3: Establish Baseline

Run the skill AS-IS before changing anything. This is experiment #0.

1. Create working directory: `autoresearch-[skill-name]/` inside the skill's folder
2. Create `results.tsv` with the header row
3. Back up the original SKILL.md as `SKILL.md.baseline`
4. Run the skill [N] times using the test inputs
5. Score every output against every eval
6. Record the baseline score

**results.tsv format (tab-separated):**

```
experiment	score	max_score	pass_rate	status	description
0	14	20	70.0%	baseline	original skill — no changes
```

**After baseline:** Print summary to terminal. If baseline is 90%+, confirm with the user whether optimization is worthwhile.

---

## Step 4: Run the Experiment Loop

This is the core autoresearch loop. Once started, run autonomously until stopped.

**LOOP:**

### 4-1. Analyze Failures

Look at which evals fail most. Read the actual outputs. Identify the pattern — formatting? missing instruction? ambiguity?

### 4-2. Form a Hypothesis

Pick ONE thing to change. Never change multiple things at once.

**Good mutations:**
- Add a specific instruction addressing the most common failure
- Reword an ambiguous instruction to be more explicit
- Add an anti-pattern ("Do NOT do X") for a recurring mistake
- Move a buried instruction higher (priority = position)
- Add or improve an example showing correct behavior
- Remove an instruction causing over-optimization for one thing

**Bad mutations:**
- Rewriting the entire skill
- Adding 10 new rules at once
- Adding vague instructions ("make it better")

### 4-3. Make the Change

Edit SKILL.md with ONE targeted mutation.

### 4-4. Run and Score

Execute the skill [N] times with the same test inputs. Score every output.

### 4-5. Keep or Discard

- **Score improved** → KEEP. This is the new baseline.
- **Score unchanged** → DISCARD. Revert. Added complexity without improvement.
- **Score worse** → DISCARD. Revert.

### 4-6. Log and Report

Append to `results.tsv`. Print progress to terminal:

```
[Experiment N] score/max (pass_rate%) — KEEP/DISCARD — one-line description
```

### 4-7. Repeat

Go back to 4-1. Continue until:
- User manually stops
- Budget cap reached
- 95%+ pass rate for 3 consecutive experiments (diminishing returns)

**If out of ideas:** Re-read failing outputs. Combine two near-miss mutations. Try removal instead of addition. Simplification that maintains score is a win.

---

## Step 5: Write the Changelog

After each experiment, append to `changelog.md`:

```markdown
## Experiment [N] — [keep/discard]

**Score:** [X]/[max] ([percent]%)
**Change:** [One sentence describing what was changed]
**Reasoning:** [Why this change was expected to help]
**Result:** [Which evals improved/declined]
**Remaining failures:** [What still fails, if anything]
```

---

## Step 6: Deliver Results

When the loop stops, present to terminal:

1. **Score summary:** Baseline → Final (percent improvement)
2. **Total experiments:** How many mutations tried
3. **Keep rate:** Kept vs discarded
4. **Top 3 changes** that helped most
5. **Remaining failure patterns**
6. **File locations** of results.tsv and changelog.md

---

## Output Structure

```
autoresearch-[skill-name]/
├── results.tsv          # score log for every experiment
├── changelog.md         # detailed mutation log
└── SKILL.md.baseline    # original skill before optimization
```

Plus the improved SKILL.md saved back to its original location.

---

## Gotchas

1. **Never skip the baseline.** Without it, you cannot measure improvement.
2. **One change at a time.** Multi-variable changes make it impossible to attribute improvement.
3. **Revert fully on discard.** Partial reverts accumulate drift.
4. **Evals can be wrong.** If all evals pass but output quality is bad, fix the evals first — go back to Step 2.
5. **Overfitting to test inputs.** If the skill improves on test inputs but degrades on novel inputs, the test inputs lack variety — go back to context gathering.
6. **Skill size creep.** Each kept mutation adds words. Periodically check if the skill exceeds 5,000 words and consolidate if needed.

---

## How This Connects to Other Skills

- **generate-skills** may define initial eval criteria during skill creation (Step 5 eval guide)
- If the target skill has existing eval criteria, autoresearch reuses them
- The changelog serves as a research log for future optimization runs
