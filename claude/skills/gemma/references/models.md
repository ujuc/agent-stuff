# Gemma Model Reference

Auto-detection selects the latest installed `gemma[0-9]+` family from Ollama.
Override with `GEMMA_MODEL` env var if needed.

Current documentation covers Gemma 4. Add new sections as newer versions are released.

## Gemma 4

### Variant Summary

| variant   | Parameters             | Context | FP16 Memory | Multimodal            | Ollama Tag         |
| --------- | ---------------------- | ------- | ----------- | --------------------- | ------------------ |
| `e2b`     | 2.3B effective / 5.1B  | 128K    | ~10-11GB    | Image + Audio         | `gemma4:e2b`       |
| **`e4b`** | 4.5B effective / 8B    | 128K    | ~16-18GB    | Image + Audio         | `gemma4:e4b` (= `gemma4:latest`) |
| `26b`     | 4B active / 26B MoE    | 256K    | ~50GB (full)| Image + Video         | `gemma4:26b`       |
| `31b`     | 31B dense              | 256K    | ~62GB       | Image + Video         | `gemma4:31b`       |

> "E" prefix denotes **Per-Layer Embeddings (PLE)** — injecting auxiliary embedding signals into each decoder layer for higher effective performance per parameter.

### Why `e4b` is the Default

1. **Runs on most Macs**: comfortable at 16GB RAM, works with Q4 quantization at 8GB
2. **Multimodal**: supports both image and audio input (26B/31B lack audio)
3. **128K context**: sufficient for long note/log summarization
4. **Significant improvement over e2b**: MMLU Pro 60.0% → 69.4%, AIME 2026 37.5% → 42.5%
5. **Ollama default tag**: `gemma4:latest` resolves to e4b (Q4_K_M) — no extra download

### When to Choose a Different Variant

#### `e2b` — Go Smaller

- Less than 10GB RAM
- Batch jobs where speed >> quality
- Battery-constrained laptop

#### `26b` (MoE) — Go Bigger

- 32GB+ RAM available
- High-quality reasoning and code generation needed (GPQA Diamond 82.3%, AIME 88.3%)
- Documents exceeding 128K tokens
- MoE activates only 4B params — speed comparable to e4b

#### `31b` (Dense) — Go Maximum

- Mac Studio/Pro with 64GB+ unified memory
- Highest quality required (LMArena Elo ~1452, #3 open model)
- Cost savings vs Claude with no quality compromise

### Benchmarks (low → high)

| Benchmark      | e2b   | e4b   | 26b MoE | 31b   |
| -------------- | ----- | ----- | ------- | ----- |
| MMLU Pro       | 60.0% | 69.4% | 82.6%   | 85.2% |
| AIME 2026      | 37.5% | 42.5% | 88.3%   | 89.2% |
| GPQA Diamond   | 43.4% | 58.6% | 82.3%   | 84.3% |
| Codeforces ELO | 633   | 940   | 1718    | 2150  |
| MMMU Pro       | 44.2% | 52.6% | 73.8%   | 76.9% |

**Takeaway**: small models degrade sharply on math, coding, and hard reasoning. Use Claude or 26b/31b for those domains.

### Common Traits (All Variants)

- **Languages**: 140+ natively supported
- **License**: Apache 2.0 (commercial and sovereign use, no MAU limits)
- **Agentic**: function calling, structured JSON output, multi-step planning, thinking mode — all native
- **Release date**: 2026-04-02

### Limitations (All Variants)

1. No music/non-speech audio understanding (speech only)
2. Video understanding limited due to insufficient post-training
3. Long-context utilization favors larger models (MRCR v2 @128K: e2b 19.1% vs 31b 66.4%)
4. Even 31b scores ~26% on HLE (Hard Lab Eval) — top-tier reasoning still limited

### Sources

- https://ai.google.dev/gemma/docs/core/model_card_4
- https://huggingface.co/blog/gemma4
- https://blog.google/innovation-and-ai/technology/developers-tools/gemma-4/
