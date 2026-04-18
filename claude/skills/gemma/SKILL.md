---
name: gemma
description: "로컬 LM Studio와 Google AI Studio(Gemini API)를 통해 Gemma 모델에 프롬프트를 전달한다. variant별 자동 라우팅(e2b/e4b→로컬, 26b/31b→원격)과 LM Studio 미가용 시 Gemini 폴백을 지원한다. gemma, gemma4, gemma로 요약해줘, gemma로 번역해, lm studio로 돌려줘, gemini api로 보내줘, 로컬 LLM, 오프라인 AI, 로컬로 처리해, API 키, 클라우드로 돌려줘, Gemma 호출 요청 시 사용한다. 민감 정보 오프라인 처리, 긴 컨텍스트 요약, 다국어 번역, 초안 생성 등에 적합."
model: sonnet
allowed-tools: Bash(rtk:*), Bash(bash:*), Bash(lms:*), Bash(op:*), Bash(curl:*), Bash(brew:*)
argument-hint: "[--local|--cloud] [variant] prompt"
---

# Gemma (LM Studio + Google AI Studio)

Dispatch a prompt to Gemma via local LM Studio (OpenAI-compatible API) or
Google AI Studio / Gemini API. Variant selects the backend by default, with
automatic fallback to the remote API when LM Studio is unavailable.

## How to invoke

All invocations go through `scripts/query.sh`. Use the `Bash` tool with
`rtk bash ...` (token-optimizing proxy; mandated by the user's global
CLAUDE.md → RTK policy):

```bash
# Default: variant e4b on LM Studio, auto-fallback to Gemini if LM Studio down.
rtk bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh "이 문단을 3줄로 요약해줘: ..."

# Explicit variant.
rtk bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh e4b "hello"

# Force remote (Gemini API).
rtk bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh --cloud 31b "복잡한 추론 문제: ..."

# Force local; fail if LM Studio unavailable (privacy-strict mode).
GEMMA_NO_FALLBACK=1 rtk bash /Users/ujuc/.claude/skills/gemma/scripts/query.sh --local e4b "민감 데이터: ..."
```

stdout contains only the model response. stderr has a single `info:` line
with `backend=lmstudio|gemini model=<id>` so you can surface the real model
to the user (e.g., prefix `Gemma (gemma-3n-e4b-it via LM Studio) 응답:`).

## Variant → Backend Routing

| variant | Default backend | Intent |
|---------|-----------------|--------|
| `e2b`   | LM Studio       | 8GB RAM, fastest |
| **`e4b`** | **LM Studio** (skill default) | 16GB RAM, balanced |
| `26b`   | Gemini API      | High-quality, too big for most laptops |
| `31b`   | Gemini API      | Top-tier reasoning |
| `pro`   | Gemini API      | Bypass Gemma, use `gemini-pro-latest` |
| `flash` | Gemini API      | Bypass Gemma, use `gemini-flash-latest` |

Override with `--local` / `--cloud` or `GEMMA_BACKEND=lmstudio|gemini`.

See `references/models.md` for the model ID matrix and resolution regex.

## Fallback behavior

If a request targets LM Studio but `GET /v1/models` fails within 3 seconds,
or no loaded model matches the variant, the script silently falls back to
Gemini API. The switch is logged to stderr:

```
warn: LM Studio unreachable at http://localhost:1234
info: backend=gemini model=gemini-flash-latest (Gemma not available on API), fallback from LM Studio
```

To **disable** fallback (e.g., for privacy-sensitive prompts that must not
leave the machine): set `GEMMA_NO_FALLBACK=1` or pass `--local`. The script
then exits with code 3 and prints setup hints.

## Remote model auto-discovery

Google AI Studio publishes new Gemma variants on their own schedule, so IDs
are resolved at runtime via `scripts/list-gemini-models.sh` (5-min cached).
The script picks the **highest-version Gemma** matching the requested
variant, falling back to `gemini-flash-latest` or `gemini-pro-latest` aliases
if no Gemma match exists. No code change needed when Gemma 4 ships on the
API — it will be picked up automatically.

Override with `GEMMA_GEMINI_MODEL=<id>` for a specific model.

## Setup (first run)

On first invocation in a fresh environment, `query.sh` will trigger
`scripts/ensure-deps.sh` to install missing tools via `brew`:

- `curl`, `jq` (formulae)
- `lm-studio` cask (for local path)
- `1password-cli` cask (for remote path — API key lives in 1Password)

Prompt-less install: `GEMMA_AUTO_INSTALL=1`.

After install, you still need to:

1. **LM Studio**: open GUI once, download `lmstudio-community/gemma-3n-E4B-it-MLX-4bit`,
   run `lms server start` and `lms load <model>`. Details in
   `references/backends.md`.
2. **1Password**: store the API key at `op://key/gemini-key/credential`
   (override via `GEMMA_OP_REFERENCE`), ensure `op whoami` succeeds.

## Procedure

1. Identify the prompt body and (optional) variant from the user's request.
2. For long inputs, build the prompt with a clear instruction on top and the
   body in a single string (heredoc or quoted).
3. Call `query.sh` via `rtk bash` with the chosen flags.
4. Surface the stdout response to the user with a header that names the
   actual backend and model (read from the stderr `info:` line). Never
   present Gemma/Gemini output as if it were Claude's own reply.
5. On script failure, relay the hint printed to stderr (install, server
   start, 1Password signin, etc.) to the user verbatim.

## Environment variables

| Variable             | Default                              | Purpose |
|----------------------|--------------------------------------|---------|
| `GEMMA_BACKEND`      | *(auto by variant)*                  | `lmstudio` or `gemini` |
| `GEMMA_LMSTUDIO_HOST`| `http://localhost:1234`              | LM Studio OpenAI base URL |
| `GEMMA_GEMINI_MODEL` | *(auto: Gemma-first, Gemini fallback)* | Remote model ID override |
| `GEMMA_OP_REFERENCE` | `op://key/gemini-key/credential`     | 1Password secret reference |
| `GOOGLE_AI_API_KEY`  | *(not set)*                          | Skip 1Password; use key directly |
| `GEMMA_TIMEOUT`      | `120`                                | HTTP timeout (seconds) |
| `GEMMA_NO_FALLBACK`  | `0`                                  | `1` = disable LM Studio → Gemini fallback |
| `GEMMA_AUTO_INSTALL` | `0`                                  | `1` = install deps without prompting |
| `GEMMA_MODELS_TTL`   | `300`                                | Gemini model-list cache TTL |
| `GEMMA_MODELS_FORCE` | `0`                                  | `1` = bypass model-list cache |

Full walkthrough in `references/backends.md`.

## When to use Gemma vs Claude

Appropriate for:

- Summarizing or classifying sensitive data offline (use `--local` +
  `GEMMA_NO_FALLBACK=1`)
- Multilingual translation (140+ languages supported natively)
- Long-document summarization (128K+ context)
- Drafting notes, obsidian pages, initial outlines
- Structured JSON output or function-call style tool use

Better left to Claude:

- High-difficulty math/reasoning (AIME, GPQA Hard) — Claude wins decisively
- Large-codebase navigation — Claude Code's Agent/Explore is purpose-built
- Music/non-speech audio understanding — Gemma does not support it

## Error handling

| Exit | Cause | Hint |
|------|-------|------|
| 2    | `curl` / `jq` missing | `ensure-deps.sh` auto-runs; accept install or `GEMMA_AUTO_INSTALL=1` |
| 3    | LM Studio unavailable and fallback disabled | `lms server start && lms load <model>` |
| 3    | 1Password not signed in | `eval "$(op signin)"` |
| 4    | 1Password item not readable | Check `GEMMA_OP_REFERENCE`, vault/item/field |
| 5    | Gemini HTTP failure | Check key validity / rate limits / network |
| 6    | Malformed Gemini response | Stderr prints raw body; usually a 401/429 text error |
| 64   | Usage error | Empty prompt or unknown flag |

## References

- `references/models.md` — variant table, MLX model IDs, Gemma 4 benchmarks.
- `references/backends.md` — LM Studio install, Google AI Studio key flow.
- `references/delegation-guide.md` — when to delegate to Gemma from Claude.

## Eval Criteria

Binary checks the skill must pass when invoked in realistic conditions:

1. With LM Studio running and Gemma loaded, a default `e4b` call returns a
   non-empty response and logs `backend=lmstudio` on stderr.
2. With LM Studio stopped and `GEMMA_NO_FALLBACK` unset, the same call
   silently falls back to Gemini and returns a non-empty response.
3. `GEMMA_NO_FALLBACK=1` with LM Studio stopped exits non-zero without
   contacting Google.
4. `--cloud 31b` resolves to a `gemini-pro-latest` (or newer Gemma 31b if
   the API exposes one) model and returns a response.
5. `GEMMA_OP_REFERENCE` pointing to a non-existent item produces a clear
   error (exit 4) rather than a Google 401.
