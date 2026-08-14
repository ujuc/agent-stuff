# Gemma Model Reference

The `gemma` skill routes each request to either **LM Studio** (local) or the
**Google AI Studio / Gemini API** (remote), depending on the requested variant
and the availability of the local runtime.

## Variant → Backend Routing

| variant | Default backend | Why |
|---------|-----------------|-----|
| `e2b`   | LM Studio       | 2.3B effective, ≥8GB RAM |
| **`e4b`** | **LM Studio** (default) | 4.5B effective, ≥16GB RAM — skill default |
| `26b`   | Gemini API      | 26B MoE — too large for most laptops |
| `31b`   | Gemini API      | 31B dense — workstation-class only |
| `pro`   | Gemini API      | Skip Gemma, go straight to `gemini-pro-latest` |
| `flash` | Gemini API      | Skip Gemma, go straight to `gemini-flash-latest` |

Manual override: `--local` / `--cloud` on `query.sh`, or `GEMMA_BACKEND=lmstudio|gemini`. Forcing `lmstudio` disables remote fallback.

## Automatic Fallback

LM Studio prefers a Gemma model matching the requested variant, then any loaded Gemma model. Unless disabled, any failed local attempt falls back to the Gemini API. Set `GEMMA_NO_FALLBACK=1` or pass `--local` when prompts must never leave the machine.

When a fallback happens, stderr logs:

```
info: backend=gemini model=<id> (Gemma not available on API), fallback from LM Studio
```

## Local LM Studio Catalog (MLX, Apple Silicon)

MLX is preferred on Apple Silicon for memory and speed efficiency. GGUF
variants from the same repositories work as a fallback on x86 or other
platforms.

| variant | LM Studio model ID                                     | Size (Q4) | Min RAM |
|---------|---------------------------------------------------------|-----------|---------|
| `e2b`   | `lmstudio-community/gemma-3n-E2B-it-MLX-4bit`           | ~3GB      | 8GB     |
| `e4b`   | `lmstudio-community/gemma-3n-E4B-it-MLX-4bit`           | ~5GB      | 16GB    |

These IDs track the `lmstudio-community` HuggingFace space. When the community
publishes Gemma 4 MLX builds, update this table — `query.sh` auto-matches any
loaded model whose id contains both `gemma` and the variant tag, so the script
picks new versions up without code changes.

Load once in LM Studio:

```bash
lms load lmstudio-community/gemma-3n-E4B-it-MLX-4bit
lms server start
```

## Remote Model Resolution (Gemma-first, Gemini fallback)

Gemma 4 is the current top family (per ai.google.dev/gemma/docs), but Google AI
Studio publishes each variant on its own timeline, so hardcoding IDs would age
poorly. `query.sh` resolves the remote model as follows:

1. If `GEMMA_GEMINI_MODEL` is set, use it verbatim.
2. Otherwise, fetch the live model list via `list-gemini-models.sh` and pick
   the **highest-version** Gemma match per regex:

   | variant | Regex                                    |
   |---------|-------------------------------------------|
   | `e2b`   | `gemma-[0-9]+n?-e2b-it`                   |
   | `e4b`   | `gemma-[0-9]+n?-e4b-it`                   |
   | `26b`   | `gemma-[0-9]+-(26b\|27b)-it`              |
   | `31b`   | `gemma-[0-9]+-31b-it`                     |

3. If no Gemma match, fall back to a Gemini alias:

   | variant         | Fallback alias         |
   |-----------------|------------------------|
   | `e2b`, `e4b`, `26b`, `flash` | `gemini-flash-latest`  |
   | `31b`, `pro`    | `gemini-pro-latest`    |

`*-latest` aliases carry a two-week deprecation notice per Google's model
policy, so they're safe for long-running workflows.

Cache: the model list is cached at `/tmp/gemma-skill-models.cache` for 5
minutes (`GEMMA_MODELS_TTL`). Force-refresh with `GEMMA_MODELS_FORCE=1`.

## Gemma 4 Variants (source: ai.google.dev/gemma/docs/core/model_card_4)

Released 2026-04-02.

| variant   | Parameters            | Context | Multimodal             |
|-----------|------------------------|---------|------------------------|
| `e2b`     | 2.3B effective / 5.1B | 128K    | Image + Audio          |
| **`e4b`** | 4.5B effective / 8B   | 128K    | Image + Audio          |
| `26b`     | 4B active / 26B MoE   | 256K    | Image + Video          |
| `31b`     | 31B dense              | 256K    | Image + Video          |

"E" = Per-Layer Embeddings (PLE), boosting effective performance per parameter.

### Benchmarks (selected)

| Benchmark      | e2b   | e4b   | 26b MoE | 31b   |
|----------------|-------|-------|---------|-------|
| MMLU Pro       | 60.0% | 69.4% | 82.6%   | 85.2% |
| AIME 2026      | 37.5% | 42.5% | 88.3%   | 89.2% |
| GPQA Diamond   | 43.4% | 58.6% | 82.3%   | 84.3% |
| Codeforces ELO | 633   | 940   | 1718    | 2150  |

Small variants degrade sharply on math / hard reasoning. Use Gemini Pro or
Claude for those domains.

### Languages & Licensing

- 140+ languages natively supported
- Apache 2.0 license — commercial use, no MAU caps
- Function calling, structured JSON output, thinking mode all native

### Known Limitations

1. No music / non-speech audio (speech only)
2. Video understanding post-training is limited
3. Long-context utilization favors larger variants (MRCR@128K: e2b 19.1% vs
   31b 66.4%)
4. Even 31b scores ~26% on HLE — top-tier reasoning still bounded

## Sources

- https://ai.google.dev/gemma/docs/core/model_card_4
- https://ai.google.dev/gemma/docs
- https://ai.google.dev/gemini-api/docs/models
- https://ai.google.dev/gemini-api/docs/coding-agents
- https://huggingface.co/lmstudio-community
