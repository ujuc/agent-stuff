---
name: prompting-assist
description: "사용자가 LLM에 보낼 프롬프트를 개선·리뷰·피드백받고 싶어할 때 사용. Anthropic 공식 프롬프팅 모범 사례(semantics 참조 001)에 근거한 체크리스트로 진단하고 개선안을 제시한다. '프롬프트 개선해줘', '이 프롬프트 리뷰해줘', '프롬프팅 팁', '/prompting' 등 명시적 어구에만 발동하며, 일반 대화 속 '프롬프트'라는 단어만으로는 발동하지 않는다."
group: writing
model: sonnet
allowed-tools: Read, Edit, AskUserQuestion
---

# Prompting Assist

## Purpose

Diagnose a user-authored prompt against Anthropic's official prompting best practices and propose concrete improvements. Activates only when prompt authoring / improvement is the explicit subject — not whenever the word "prompt" appears.

## Trigger Policy

Korean trigger phrases are kept verbatim because they must match user utterances directly.

**Activate on:**
- "프롬프트 개선해줘"
- "이 프롬프트 리뷰해줘" / "이 프롬프트 피드백 줘"
- "프롬프팅 팁 알려줘"
- `/prompting`
- "system prompt 개선해줘"

**Do NOT activate on:**
- "프롬프트가 너무 길어서..." (word appears ≠ improvement request)
- "프롬프트 엔지니어링이 뭐야?" (concept question)
- "이 프롬프트 의미가 뭐야?" (interpretation request)

If intent is ambiguous, ask one clarifying question first: "이 프롬프트를 개선해드릴까요, 아니면 의미를 설명해드릴까요?"

## Workflow

### Stage 1: Context Collection

1. **Acquire the prompt source.**
   - If already pasted into chat, confirm the range.
   - If a file path is given, `Read` it.
   - If absent, request once: "어떤 프롬프트를 보고 싶으신가요?"

2. **Collect the minimum necessary context** via `AskUserQuestion` (batch the questions, do not re-ask):
   - Target model: Claude 4.x family / another LLM / unknown
   - Primary use case: one-shot / agentic / tool-calling / long-context / coding
   - Hard constraints: response length / cost / latency / output format

If the model is unknown, default to Claude 4.6/4.7 and state the assumption explicitly.

### Stage 2: Reference Load

`Read` the primary reference:

```
$GYEOL_HOME/memory/semantics/summary/001-anthropic-prompting-best-practices.md
```

Use the "Prompt Authoring Checklist" section as the diagnostic baseline. Code snippets in "Detailed Notes" can be reused verbatim as improvement examples.

Only `Read` the full archived source (`source/001-...source.md`) when the checklist cannot resolve the question.

### Stage 3: Diagnosis

Judge pass / fail per checklist category:

| Category | Key question |
|----------|--------------|
| Clarity & specificity | Is the desired outcome explicit? Are scope and exceptions clear? |
| Context & motivation | Is the reason for each constraint stated? |
| Examples | Are 3–5 examples present for few-shot tasks? Are they wrapped in `<example>`? |
| Structure | Are content types separated by XML tags? Is long context at the top, query at the bottom? |
| Role & identity | Does the system prompt assign a role? |
| Output control | Is the language prescriptive (do) rather than prohibitive (don't)? Is there no reliance on last-turn prefill? |
| Thinking & effort | Does the effort setting match task difficulty? Is there no aggressive over-trigger wording? |
| Tool use & agentic | Is the action-vs-suggest intent clear? Is parallel intent marked? |
| Long-horizon | Is state held in structured files? Are completion criteria verifiable? |
| Anti-patterns | No test hard-coding, no over-defensive coding, no pressure toward needless abstraction? |

For each failing item, record a **short justification + improvement direction**. Cite the checklist item or a specific line from "Detailed Notes".

### Stage 4: Proposal

Pick the proposal format by change magnitude:

- **Small fix** (≤ 3 items): per-section diff
  ```
  Before: "Make it better"
  After:  "Refactor the loop to use parallel tool calls (see Parallel tool-call prompt)."
  Why:    Clarity & specificity (§Stage 3), Tool use (§parallel)
  ```
- **Full rewrite** (multiple failures): the improved prompt in full + a bullet list of key changes

**Prefer presenting options** when the user has a real choice: lay out "Option A (terse)" vs "Option B (strict)".

Close with a one-line checklist coverage report: "10개 범주 중 7개 합격, 3개 개선 반영."

## Constraints

- **Preserve original intent.** Never change what the user is trying to do — only raise quality.
- **Evidence-backed.** Do not assert anything outside the Anthropic reference. Every recommendation maps to a checklist item or a "Detailed Notes" line.
- **Language preservation.** Keep the prompt's original language in the artifact. Diagnosis and explanation follow the conversation language (default Korean).
- **Brevity.** Diagnosis report: 1–2 lines per category. Strip filler.
- **Model-version awareness.** Claude 4.5 → 4.6 → 4.7 diverge in non-trivial ways. When the target model is unknown, state the assumption and proceed.

## References

- `$GYEOL_HOME/memory/semantics/summary/001-anthropic-prompting-best-practices.md` — primary reference (summary, checklist, code snippets)
- `$GYEOL_HOME/memory/semantics/source/001-anthropic-prompting-best-practices.source.md` — archived full source (read only for edge cases)
- `$GYEOL_HOME/memory/semantics/_index.md` — extend here when related references accumulate

## Gotchas

1. **`$GYEOL_HOME` must be defined.** On first activation, gyeol requires `$GYEOL_HOME` to be set (`~/.config/gyeol` on macOS/Linux). If the semantics summary is missing, fall back to in-context diagnosis using well-known Anthropic guidance and surface the gap: "reference 001 not found — proceeding on general knowledge, diagnosis may be less grounded."

2. **Semantics file may be stale.** The summary is refreshed on a 60-day cadence by the session-start upstream check. If `last_upstream_check` in its frontmatter is old, cite findings with an explicit caveat rather than silently trusting.

3. **Do not over-trigger.** The description intentionally encodes do/don't patterns. Treat the word "prompt" in a user sentence as a keyword, not an invocation. Default to one clarifying question when the intent is ambiguous.

4. **Never edit the prompt in place without consent.** `Edit` is in `allowed-tools` for cases where the prompt lives in a file the user asked to be improved. Always show the proposal first, then apply the edit only after explicit confirmation.

5. **Model-version drift.** Claude 4.5 → 4.6 → 4.7 differ enough (extended thinking defaults, parallel tool-call norms, effort tuning) that a checklist pass for 4.5 can be a near-fail for 4.7. When unknown, default to the latest and state the assumption.

## Eval Criteria

```
EVAL 1: Trigger precision
  Question: Given a user utterance that only contains the word "프롬프트" without
            an improvement-request framing, does the skill decline to activate
            (or ask a clarifying question) instead of running a full diagnosis?
  Pass: Skill does not run Stage 2–4 without an explicit improvement intent.
  Fail: Skill begins full diagnosis on mere keyword presence.

EVAL 2: Reference grounding
  Question: Does every improvement recommendation cite a specific checklist
            category or "Detailed Notes" line in reference 001?
  Pass: Each recommendation has an anchor (category name or §section).
  Fail: Any recommendation is stated without a reference anchor.

EVAL 3: Intent preservation
  Question: Does the proposed prompt preserve the user's original goal,
            scope, and persona?
  Pass: Target task, constraints, and role remain intact; only phrasing/
        structure/specificity changes.
  Fail: Meaning drifts — task narrowed/broadened, constraints dropped, or
        persona replaced.

EVAL 4: Proposal structure
  Question: Is the proposal formatted per Stage 4 (Before/After diff for
            small fixes, full rewrite + key changes for larger ones) and
            closed with a one-line coverage report?
  Pass: Format matches change magnitude; coverage line present.
  Fail: Format mismatched, or coverage report missing.

EVAL 5: Language fidelity
  Question: Is the rewritten prompt in the same language as the original,
            with diagnosis written in the conversation language?
  Pass: Prompt language preserved; diagnosis in conversation language.
  Fail: Prompt translated silently, or diagnosis in the wrong language.
```
