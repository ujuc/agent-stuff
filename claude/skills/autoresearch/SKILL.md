---
name: autoresearch
description: "편집 가능한 대상(프롬프트, 설정, 코드 등)을 반복 실행-평가-변이하여 자율적으로 최적화한다. Karpathy의 autoresearch 방법론 기반. 자동 실험, eval 루프, autoresearch, make this better, benchmark, eval 요청 시 사용한다."
model: opus
disable-model-invocation: true
argument-hint: "[target-path]"
---

# Autoresearch

Adapts Andrej Karpathy's autoresearch methodology (autonomous experimentation loops) to any editable artifact — skills, prompts, configurations, code, queries, or any file where output quality can be measured.

---

## The Core Job

Take any editable target, define what "good output" looks like as binary yes/no checks, then run an autonomous loop that:

1. Runs the target using test inputs and the specified execution method
2. Scores every output against the eval criteria
3. Mutates the target file to fix failures
4. Keeps mutations that improve the score, discards the rest
5. Repeats until the score ceiling is hit or the budget is exhausted

**Output:** An improved target file + `results.tsv` log + `changelog.md` of every mutation attempted.

---

## Before Starting: Gather Context

**STOP. Do not run any experiments until all fields below are confirmed with the user via AskUserQuestion.**

1. **Target file** — Path to the file to optimize (e.g., SKILL.md, config.yaml, prompt.md, query.sql)
2. **Execution method** — How to run/test the target (e.g., invoke as skill, run a shell command, call an API)
3. **Test inputs** — 3-5 different inputs/scenarios covering different use cases (avoid overfitting to one scenario)
4. **Eval criteria** — 3-6 binary yes/no checks defining a good output (see [references/eval-guide.md](references/eval-guide.md))
5. **Runs per experiment** — How many times to run per mutation. Default: 5
6. **Budget cap** — Max number of experiment cycles. Default: 20

If the target already has eval criteria (in `## Eval Criteria` section or `evals.md`), present them to the user and ask whether to reuse, modify, or replace.

---

## Step 1: Read the Target

Before changing anything, read and understand the target completely.

1. Read the target file in full
2. Read any related files it references or depends on
3. Identify the target's core purpose, structure, and expected output
4. Note any existing quality checks or anti-patterns

Do NOT skip this. You need to understand what the target does before you can improve it.

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

Run the target AS-IS before changing anything. This is experiment #0.

1. Create working directory: `autoresearch-[target-name]/` next to the target file
2. Create `results.tsv` with the header row
3. Back up the original file as `{filename}.baseline`
4. Run the target [N] times using the test inputs and execution method
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

**Good mutations (prompts/skills):**
- Add a specific instruction addressing the most common failure
- Reword an ambiguous instruction to be more explicit
- Add an anti-pattern ("Do NOT do X") for a recurring mistake
- Move a buried instruction higher (priority = position)
- Add or improve an example showing correct behavior
- Remove an instruction causing over-optimization for one thing

**Good mutations (code/config):**
- Adjust a parameter or threshold value
- Change algorithm or logic flow
- Reorganize structure or ordering
- Toggle an option on/off
- Simplify a complex section

**Bad mutations:**
- Rewriting the entire file
- Changing multiple things at once
- Making vague changes without a clear hypothesis

### 4-3. Make the Change

Edit the target file with ONE targeted mutation.

### 4-4. Run and Score

Run the target [N] times with the same test inputs. Score every output.

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
autoresearch-[target-name]/
├── results.tsv          # score log for every experiment
├── changelog.md         # detailed mutation log
└── {filename}.baseline  # original file before optimization
```

Plus the improved target file saved back to its original location.

---

## Gotchas

1. **Never skip the baseline.** Without it, you cannot measure improvement.
2. **One change at a time.** Multi-variable changes make it impossible to attribute improvement.
3. **Revert fully on discard.** Partial reverts accumulate drift.
4. **Evals can be wrong.** If all evals pass but output quality is bad, fix the evals first — go back to Step 2.
5. **Overfitting to test inputs.** If the target improves on test inputs but degrades on novel inputs, the test inputs lack variety — go back to context gathering.
6. **Size creep.** Each kept mutation adds complexity. Periodically check if the target has grown significantly and consolidate if needed.

---

## How This Connects to Other Skills

- If optimizing a skill, **generate-skills** may have defined initial eval criteria during creation
- If the target already has eval criteria, autoresearch reuses them
- The changelog serves as a research log for future optimization runs
