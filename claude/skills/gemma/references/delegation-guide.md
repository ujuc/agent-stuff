# Gemma Delegation Guide

A calling convention for **other skills** that want to offload mechanical text
tasks to the local Gemma model via `scripts/query.sh`. This document is the
single source of truth — skills that delegate to gemma should link here rather
than redefining the protocol inline.

See [models.md](models.md) for variant specifications and benchmarks. This
document covers only *how* to call, not *which model* to use.

## Purpose

Delegation makes sense when a step in your skill is:

- **LLM-shaped but not Claude-shaped** — the task needs a language model but
  does not need Claude's specific reasoning, tool use, or conversational
  context.
- **Bulky** — the input is large enough that handing it to Claude would burn
  context or tokens disproportionate to the value of the output.
- **Offline-preferable** — the input contains sensitive data that should not
  leave the local machine.

The point is not to avoid Claude. The point is to keep Claude focused on work
only Claude can do.

## When to Delegate

Green-light cases for calling gemma:

- **Long-document summarization** — logs, dumps, transcripts, or source files
  longer than ~2000 lines. Gemma produces a first-pass condensation; Claude
  then reasons over the condensation.
- **Bulk translation** — 140+ languages natively. Quality is good enough for
  draft translations; Claude refines if needed.
- **Simple classification** — tagging, intent detection, yes/no filtering over
  a batch of items.
- **Draft generation** — initial boilerplate, structured JSON scaffolds, form
  letters where style matters less than structure.
- **Sensitive-data processing** — anything the user would not want sent over
  the network. Gemma runs entirely on localhost:11434.

## When NOT to Delegate

Red-light cases. Keep these on Claude:

- **Hard reasoning** — math, algorithmic problems, proofs. Small gemma variants
  degrade sharply on AIME, GPQA, Codeforces. See [models.md](models.md)
  benchmarks.
- **Code analysis or generation** — code review, refactoring, bug hunting,
  architectural judgment. Codeforces ELO for e4b is ~940; for e2b ~633. Use
  Claude or the 26b/31b variants only if explicitly needed.
- **Creative writing with voice** — tasks where the user expects Claude's
  specific voice, phrasing, or judgment.
- **Tasks requiring conversation context** — gemma sees only the prompt string.
  It has no access to the current conversation, prior tool results, or session
  memory.
- **Music or non-speech audio understanding** — gemma does not support this.
- **Final user-facing output** — gemma output should usually pass through a
  Claude review step before reaching the user, unless the delegating skill
  explicitly marks output as "from gemma".

## Calling Convention

### Invocation

Always call through the bundled script, never assemble curl calls inline:

```bash
bash "${CLAUDE_SKILL_DIR}/../gemma/scripts/query.sh" "<prompt>"
```

When calling from outside a skill context (no `${CLAUDE_SKILL_DIR}`), use the
absolute path:

```bash
bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "<prompt>"
```

To select a specific variant (rare — the default `latest` is usually right):

```bash
bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh e2b "<prompt>"
```

### Passing Large Input

For a **purely literal** prompt (no variable interpolation), a quoted heredoc
is the simplest form:

```bash
bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "$(cat <<'EOF'
Summarize the following paragraph in one sentence.

<paste the paragraph inline here>
EOF
)"
```

Set `GEMMA_TIMEOUT` if the prompt is large enough to risk the default 120s
ceiling:

```bash
GEMMA_TIMEOUT=300 bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "..."
```

#### Pitfall: quoted heredoc blocks command substitution

Quoted heredocs (`<<'EOF'`) suppress **everything** inside — including `$()`
command substitution. This is usually what you want for literal prompts but
it breaks the moment you try to inject dynamic content:

```bash
# BROKEN — the literal string "$(git diff --cached)" reaches gemma,
# not the actual diff. gemma responds "I cannot see any diff".
bash query.sh "$(cat <<'EOF'
Summarize this diff:
$(git diff --cached)
EOF
)"
```

Use one of these forms instead when the prompt needs dynamic content:

**Option A — Variable pre-capture + double-quoted string** (recommended).
Double-quoted strings interpolate `$var` without re-parsing the variable's
value, so diffs containing `$`, backticks, or backslashes pass through
cleanly:

```bash
DIFF=$(git diff --cached)
PROMPT="Summarize the following git diff in 5 bullet points.

---
$DIFF"
bash query.sh "$PROMPT"
```

**Option B — Unquoted heredoc**. Variable interpolation works, but the diff's
own `$`, backticks, and backslashes will be re-parsed by the shell. Avoid
unless you trust the content:

```bash
DIFF=$(git diff --cached)
bash query.sh "$(cat <<EOF
Summarize this diff:
$DIFF
EOF
)"
```

#### Pitfall: zsh noclobber on stderr redirect

Under `set -o noclobber` (common in zsh configurations), `2>/tmp/fixed.log`
succeeds on the first call but fails on the second with `file exists`.
Skills that call gemma repeatedly must use a unique log name or clear the
file first:

```bash
LOG=/tmp/gemma-$$.log        # PID-based, unique per shell
rm -f "$LOG"                 # clear any leftover
bash query.sh "$PROMPT" 2>"$LOG"
```

Using `mktemp` directly as the redirect target does **not** help — mktemp
creates the file eagerly, so the `>` redirect then refuses to clobber it.
Use `mktemp` + `rm -f` if you need randomness.

### Exit Code Contract

`query.sh` distinguishes failure modes with exit codes so the caller can react
precisely. The full list:

| Code | Meaning                                         | Caller action                               |
|------|-------------------------------------------------|---------------------------------------------|
| 0    | Success. stdout contains the response.          | Use stdout.                                 |
| 2    | Missing dependency (`curl` or `jq` not found).  | Fall back. Inform user once.                |
| 3    | Ollama not reachable at `$GEMMA_HOST`.          | Fall back silently (see Fallback Policy).   |
| 4    | No gemma model installed, or variant missing.   | Fall back. Suggest `ollama pull gemma4`.    |
| 5    | curl failed (e.g., network, timeout).           | Fall back. Mention shortening input.        |
| 6    | Ollama returned unexpected JSON.                | Fall back. This is a bug — log the response.|
| 64   | Usage error (empty prompt).                     | Fix the call. This is a caller bug.         |

The practical rule for most skills is:

> **exit 0 → use stdout. Any other code → fall back and continue.**

Only more sophisticated skills need to distinguish exit 3 (Ollama down, expected
in many environments) from exit 4 (missing model, one-time setup issue).

### Stdout vs Stderr

- **stdout** contains *only* the gemma response text. Capture it into a
  variable for direct use.
- **stderr** contains info messages (`info: using model gemma4:latest`) and
  error messages. Always preserve stderr for debugging. In automated flows,
  redirect stderr to a log file rather than discarding it.

Correct pattern:

```bash
if response=$(bash /path/to/query.sh "$prompt" 2>/tmp/gemma.log); then
  # use $response
else
  # fall back, optionally surface /tmp/gemma.log
fi
```

## Fallback Policy

**Gemma is optional infrastructure.** A skill that delegates to gemma must
still function when gemma is unavailable. The fallback rule is strict:

1. If `query.sh` exits non-zero, the calling skill must **continue** through
   its primary Claude-only path.
2. The fallback must be **silent by default** — do not block the user with an
   error dialog. Log the failure to stderr or an internal note, and proceed.
3. The user may be informed *once per session* that gemma was unavailable, as
   a short aside ("note: gemma pre-summarization skipped, Ollama not running").
   Do not repeat this per call.
4. The skill must never *fail* because gemma failed. A gemma-dependent skill
   is a broken skill.

This is intentional. Ollama may not be running, may not have a gemma model
installed, may be on a different port, or may time out on large inputs. The
caller absorbs all of these as normal conditions, not errors.

## Result Presentation

When a delegating skill shows gemma output to the user, it must:

1. **Label the source explicitly.** A prefix like `Gemma (gemma4:latest):` or
   a block quote marker. Users should be able to tell at a glance which words
   came from gemma vs Claude. The model name can be extracted from stderr's
   `info: using model <name>` line.
2. **Show the gemma output verbatim or lightly edited.** Do not launder gemma
   output as Claude's voice.
3. **Follow up with Claude's own judgment** if the skill uses gemma as a
   pre-processing step. Claude reviews, refines, and takes responsibility for
   the final answer.

Example of a well-formed presentation:

```
Gemma (gemma4:latest) first-pass summary:
> The diff refactors the auth middleware to use a session token store.
> Three files changed: auth.ts, session.ts, middleware-test.ts.

Claude review: the summary is accurate. For the commit body I'll also note
the test coverage added in middleware-test.ts.
```

## Anti-Patterns

Things to avoid when designing a delegation point:

- **Silent laundering** — passing gemma output off as if Claude wrote it.
  Users lose the ability to calibrate trust.
- **Delegation for reasoning steps** — if the output of the gemma call feeds
  into a decision the skill will make, re-check the "When NOT to Delegate"
  list. Small gemma variants are not reliable for decisions.
- **Chaining gemma calls** — don't build multi-step pipelines inside a single
  skill. If you need a pipeline, use Claude to orchestrate and call gemma once
  per stage.
- **Assuming gemma is available** — no default-to-gemma paths. Every skill's
  golden path must work with Ollama off.
- **Hardcoding a specific variant** — prefer the default `latest` resolution.
  Only specify a variant if the skill has a measured reason (e.g., e2b for
  throughput on battery, 26b for code-adjacent tasks).

## Minimum Skill Checklist

Before adding a Gemma Delegation section to your SKILL.md, verify:

- [ ] The delegated step is one that gemma can actually do well (see When to
      Delegate).
- [ ] The skill still works if `query.sh` exits non-zero (see Fallback Policy).
- [ ] The calling path captures stdout into a variable and redirects stderr.
- [ ] The SKILL.md's `allowed-tools` includes the Bash entry needed to run
      `query.sh` — typically `Bash(bash:*)` is already present.
- [ ] The user-facing output distinguishes gemma content from Claude content.
- [ ] A short section in the SKILL.md body links back to this guide and states
      the specific trigger condition (e.g., "when git diff exceeds 500 lines").

If all boxes are checked, the delegation is safe to add.
