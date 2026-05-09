---
name: eos
description: "세션 종료(End Of Session) 정리 의식. 현재 대화와 미처리 transcript 버퍼를 함께 정리해 gyeol daily log에 append, _recent.md 7일 trim, schema lint, 처리된 버퍼 삭제까지 한 번에 실행한다. /eos, 세션 종료, eos, wrap up, 끝내기 정리, 오늘치 일기, 정리하고 끝내자 류로 호출 시 사용. modifier에 강하게/검수/review 포함 시 advisor 패스 추가."
group: meta
model: haiku
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
argument-hint: "[강하게|review]"
---

# EOS — End Of Session

Wrap-up ritual for the gyeol memory system. Drains pending transcript buffers, appends today's `episodes/daily/{YYYY-MM-DD}.md`, trims `_recent.md` to a 7-day window, runs deterministic schema lint, and deletes processed buffers — all in a single foreground invocation so the user retains permission gating.

`$GYEOL_HOME` resolves to `~/.config/gyeol` on macOS/Linux and `%APPDATA%\gyeol` on Windows. All paths below are macOS/Linux-absolute for clarity; substitute on other platforms.

## Pattern

Linear workflow with a deterministic lint gate. Steps execute top-to-bottom. The lint gate (Step 6) loops back on failure until all three checks pass.

## Sequence

### Step 1 — Discover pending buffers

```bash
GYEOL_HOME="${GYEOL_HOME:-$HOME/.config/gyeol}"
TODAY=$(date +%Y-%m-%d)
BUFFERS=$(ls -1 "$GYEOL_HOME/.session-buffer/"*.jsonl 2>/dev/null || true)
```

Remember the buffer paths — Step 7 deletes them after successful processing.

### Step 2 — Decide what to consolidate

- **Current session context** is already loaded in the working model. Do not re-read its transcript.
- **Each buffer file** is a separate prior session left behind by the SessionEnd hook. Read each JSONL with `Read`; if a file is large, sample the head and tail (~200 lines combined) and skim the middle for decisions and artifacts.

Each buffered transcript becomes a distinct `## Session N` entry in today's daily log.

### Step 3 — Append to today's daily log

Target file: `$GYEOL_HOME/memory/episodes/daily/{TODAY}.md`.

If the file does not exist, create it with frontmatter:

```markdown
---
date: "{TODAY}"
sessions: {count_to_be_added}
---

# {TODAY}
```

If it exists, count existing `## Session ` headers, add the number of sessions to be appended, and update the frontmatter `sessions:` value.

For each session, append using exactly this schema:

```markdown
## Session N — HH:MM — one-line summary

### What Happened
{Korean prose. Meaning, not transcript. What was done, what was discussed.}

### Decisions Made
- {Decision and reasoning}

### Open Questions
- {Unresolved items}

### Artifacts
- {Absolute file paths, commit hashes, generated assets}
```

`HH:MM` rules:
- For the current session, use `date +%H:%M`.
- For buffered transcripts, prefer timestamps inside the JSONL when present; otherwise estimate with a coarse but valid value (e.g. `09:00`, `14:00`). Empty time slots and `??:??` are forbidden — they fail Lint 1.

Body conventions:
- Korean prose. Identifiers and paths stay in source language.
- No large code blocks; reference paths instead.
- One bullet per decision or artifact; avoid multi-line bullets.

### Step 4 — Update `_recent.md` and trim to 7 days

Target file: `$GYEOL_HOME/memory/episodes/_recent.md`.

Add 1–3 bullets under today's date heading. If the heading does not exist, create it. Avoid duplicating bullets already present.

Trim with the cutoff:

```bash
SEVEN_AGO=$(date -v-7d +%Y-%m-%d 2>/dev/null || date -d '7 days ago' +%Y-%m-%d)
```

Remove every `## YYYY-MM-DD` section whose date is strictly earlier than `$SEVEN_AGO`. Update the frontmatter `last_updated:` to today's date.

### Step 5 — Update threads (conditional)

`grep` `$GYEOL_HOME/memory/episodes/threads/*.md` for topics with material progress this session. Append a timeline entry to matching threads. **Do not create new threads** — that is reserved for explicit user request; threads earn their existence by spanning 2+ sessions.

### Step 6 — Lint gate

Run all three checks. If any fails, return to the relevant step, fix, and re-run. Stop after three full retry cycles (see Failure Modes).

```bash
DAILY="$GYEOL_HOME/memory/episodes/daily/${TODAY}.md"
RECENT="$GYEOL_HOME/memory/episodes/_recent.md"

# Lint 1: every Session header matches the strict schema
BAD=$(grep -nE '^## Session [0-9]+ ' "$DAILY" \
      | grep -vE '^[0-9]+:## Session [0-9]+ — [0-9]{2}:[0-9]{2} — .+$' || true)
[[ -z "$BAD" ]] && echo "lint1=OK" || { echo "lint1=FAIL"; echo "$BAD"; }

# Lint 2: declared sessions count == actual header count
DECLARED=$(awk '/^sessions:/ {print $2; exit}' "$DAILY")
ACTUAL=$(grep -cE '^## Session [0-9]+ — ' "$DAILY")
[[ "$DECLARED" == "$ACTUAL" ]] \
  && echo "lint2=OK" \
  || echo "lint2=FAIL declared=$DECLARED actual=$ACTUAL"

# Lint 3: every date heading in _recent.md is within the 7-day window
SEVEN_AGO=$(date -v-7d +%Y-%m-%d 2>/dev/null || date -d '7 days ago' +%Y-%m-%d)
STALE=$(grep -E '^## [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$RECENT" \
        | awk -v cutoff="$SEVEN_AGO" '{gsub(/^## /,""); if ($0 < cutoff) print}')
[[ -z "$STALE" ]] && echo "lint3=OK" || { echo "lint3=FAIL"; echo "$STALE"; }
```

### Step 7 — Delete processed buffers

Delete every buffer path captured in Step 1. The current session's transcript will be re-buffered by the SessionEnd hook on next exit; do not touch live transcript files.

```bash
for f in $BUFFERS; do rm -f "$f"; done
```

### Step 8 — Report to the user

Print a 3–5 line summary in Korean:

- Number of sessions consolidated (current + N buffered)
- Range of `Session N` numbers appended
- `_recent.md` entries trimmed (if any)
- Lint pass result
- Outstanding open questions worth user follow-up (if any)

### Step 9 — Optional advisor pass

If `$ARGUMENTS` contains `강하게`, `검수`, `review`, `--review`, or `verify`, call `advisor()` once after Step 8. Apply any structural feedback by re-running Steps 3–6 as needed. Skip this step entirely when no modifier is present — routine wrap-up should not invoke advisor.

## Failure Modes

- **Buffer JSON parse failure**: skip that buffer, log a warning, and move the file to `$GYEOL_HOME/.session-buffer/.broken/`. Do not delete it; the user can inspect later.
- **Lint gate fails three full cycles**: stop. Print the failing checks and the last attempted file diff. Do not force-write a non-conforming entry.
- **`$GYEOL_HOME` missing or `IDENTITY.md` absent**: gyeol is not initialized. Direct the user to the first-activation procedure in `~/.claude/CLAUDE.md` and exit without writing.
- **`_recent.md` missing**: create it with the documented frontmatter and an empty body, then proceed normally.

## Extension Slots

Hooks for additional wrap-up tasks. Currently inactive — activate by inserting the relevant block between Steps 5 and 6 (after files are written, before lint).

- **End-of-month consolidation prompt** — when `$(date +%d)` equals the last day of the month, suggest the user run monthly consolidation per `MEMORY_SYSTEM.md`.
- **Semantics index rebuild** — when `$GYEOL_HOME/.semantics_dirty` exists, surface a one-line reminder to run `python3 $GYEOL_HOME/scripts/build-index.py` (or the in-session equivalent per the local Semantics policy).
- **Uncommitted git changes** — when the working tree has staged or unstaged changes, suggest `commit` skill.
- **Thread dormancy** — for any thread file with `last_updated:` older than 90 days, set `status: dormant` in its frontmatter.
- **Stale check markers** — verify `.last_update_check`, `.last_semantics_scan`, `.last_skill_improver_run` against their cadence policies.

## Why This Skill Exists

Two problems in one solution:

1. **Daily logs go unwritten.** Without an end-of-session ritual, identity files like `IDENTITY.md` exist but `episodes/daily/` stays empty for weeks. Memory accumulates only in machinery, not in lived record.
2. **Autonomous SessionEnd spawn carries injection risk.** A `SessionEnd` hook that detaches `claude --permission-mode bypassPermissions` lets a malicious payload in the transcript steer the child agent into arbitrary tool calls. A user-triggered skill keeps the permission flow intact.

The companion `~/.claude/hooks/gyeol-buffer-transcript.sh` (a thin launcher over a Rust binary) compensates for the manual-trigger gap by copying transcripts to `$GYEOL_HOME/.session-buffer/` on every session exit. No spawn, no bypass — pure file copy. That way a forgotten `/eos` becomes "next `/eos` drains the backlog."

Cross-device limitation accepted: if the next session opens on a different machine before `/eos` runs, that day's transcript stays on the prior device until the user returns to it.

## Eval Criteria

```
EVAL 1: Daily log target exists and is well-formed
  Question: After invocation, does $GYEOL_HOME/memory/episodes/daily/{TODAY}.md
            exist with valid YAML frontmatter (date, sessions) and at least
            one ## Session N header?
  Pass: File exists, parseable frontmatter, ≥ 1 Session header.
  Fail: Missing file, malformed frontmatter, or zero Session headers.

EVAL 2: Schema regex compliance
  Question: Does every ## Session header match
            ^## Session [0-9]+ — [0-9]{2}:[0-9]{2} — .+$ ?
  Pass: All headers match.
  Fail: Any header deviates (missing time, free-form summary leader, etc.).

EVAL 3: Frontmatter sessions count == header count
  Question: Does the frontmatter `sessions:` value equal the count of
            ## Session headers in the body?
  Pass: Counts equal.
  Fail: Mismatch.

EVAL 4: _recent.md within 7-day window
  Question: Are all ## YYYY-MM-DD sections in _recent.md dated within the
            last 7 days from today?
  Pass: No stale dates.
  Fail: Any date older than today − 7 days remains.

EVAL 5: Buffer cleanup
  Question: After successful processing, are all buffer files identified
            in Step 1 removed from $GYEOL_HOME/.session-buffer/?
  Pass: All targeted buffers deleted (or moved to .broken/ on parse error).
  Fail: Any targeted buffer still present without explicit failure logged.
```

## Gotchas

1. **Buffer ordering matters.** When multiple buffers exist (forgotten /eos days), process them in chronological order so Session numbers in the daily log match real time order. Sort by filename before reading.
2. **`HH:MM` for buffers is best-effort.** JSONL entries may lack timestamps. A coarse estimate (`09:00`, `14:00`) is acceptable and required to pass Lint 1; never write `00:00` for a session that did not occur at midnight — it makes the log misleading.
3. **`disable-model-invocation` is intentionally off.** This skill must auto-trigger on natural utterances ("정리하고 끝내자"). Do not add the flag — the resulting `/eos`-only behavior would defeat the point.
4. **Lint failure is not a license to lower the bar.** If lint cannot pass after three retries, stop and report. Loosening the regex undermines every future read of the log.
