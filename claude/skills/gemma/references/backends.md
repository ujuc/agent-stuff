# Backend Setup Guide

## LM Studio (local)

### Install

```bash
brew install --cask lm-studio
```

Or let `scripts/ensure-deps.sh --lmstudio` do it interactively. After
installation, open LM Studio once to initialize the `~/.lmstudio` directory
and accept the EULA.

PATH: the `lms` CLI lives under `~/.lmstudio/bin/lms`. Add to your shell rc
if not already:

```bash
export PATH="$HOME/.lmstudio/bin:$PATH"
```

### Download a model

Inside LM Studio GUI, search for `gemma-3n-E4B-it-MLX-4bit` (publisher:
`lmstudio-community`). Or from CLI:

```bash
lms get lmstudio-community/gemma-3n-E4B-it-MLX-4bit
```

### Start the server

```bash
lms server start
lms load lmstudio-community/gemma-3n-E4B-it-MLX-4bit
```

Verify:

```bash
lms server status
curl -s http://localhost:1234/v1/models | jq '.data[].id'
```

A running server responds to `GET /v1/models` in under a second; if it
doesn't, `query.sh` falls back to Gemini API.

### Swap models per variant

```bash
lms unload --all
lms load lmstudio-community/gemma-3n-E2B-it-MLX-4bit   # lighter
```

## Google AI Studio (remote)

### Get a key

1. Open https://aistudio.google.com/apikey
2. Create a new API key (or reuse an existing project's).
3. **Store in 1Password** — do not paste into shell history or dotfiles.

### 1Password item layout

The skill expects a key at `op://key/gemini-key/credential`:

- Vault: `key`
- Item name: `gemini-key`
- Field: `credential`

Override with `GEMMA_OP_REFERENCE="op://Vault/Item/field"` if your layout
differs.

### Install 1Password CLI

```bash
brew install --cask 1password-cli
```

Enable the Touch ID integration in the 1Password desktop app → Developer →
"Integrate with 1Password CLI" — this lets `op read` unlock without prompting.

Sign in once per shell:

```bash
eval "$(op signin)"
```

Verify the reference resolves:

```bash
op read "op://key/gemini-key/credential" | head -c 12
```

### Inspect available models

```bash
bash ~/.claude/skills/gemma/scripts/list-gemini-models.sh
```

The output is cached at `/tmp/gemma-skill-models.cache` for 5 minutes. Force
refresh:

```bash
GEMMA_MODELS_FORCE=1 bash ~/.claude/skills/gemma/scripts/list-gemini-models.sh
```

Look for:

- `gemma-3n-e4b-it`, `gemma-3-27b-it` (Gemma variants)
- `gemini-flash-latest`, `gemini-pro-latest` (Gemini aliases, safe defaults)
- `gemini-3.1-pro`, `gemini-3-flash` (pinned versions)

### Gemma vs Gemini on the API

- Gemma models on the API support text generation, but function calling /
  image input are **limited** compared to Gemini models. For tool-use
  workflows, override with `GEMMA_GEMINI_MODEL=gemini-flash-latest`.
- Free tier has per-minute request caps. Volume workloads belong on local LM
  Studio, not the API.

## Environment variable quick reference

| Variable                  | Default                                 | Purpose |
|---------------------------|-----------------------------------------|---------|
| `GEMMA_BACKEND`           | *(auto by variant)*                     | Force `lmstudio` or `gemini` |
| `GEMMA_LMSTUDIO_HOST`     | `http://localhost:1234`                 | LM Studio OpenAI-compatible base |
| `GEMMA_GEMINI_MODEL`      | *(auto: Gemma-first, Gemini fallback)*  | Full remote model id |
| `GEMMA_OP_REFERENCE`      | `op://key/gemini-key/credential`        | 1Password path for API key |
| `GOOGLE_AI_API_KEY`       | *(not set)*                             | Direct key injection (skip op) |
| `GEMMA_TIMEOUT`           | `120`                                   | HTTP timeout in seconds |
| `GEMMA_NO_FALLBACK`       | `0`                                     | `1` = fail instead of falling back |
| `GEMMA_AUTO_INSTALL`      | `0`                                     | `1` = install deps without prompting |
| `GEMMA_MODELS_CACHE`      | `/tmp/gemma-skill-models.cache`         | Model-list cache path |
| `GEMMA_MODELS_TTL`        | `300`                                   | Cache TTL in seconds |
| `GEMMA_MODELS_FORCE`      | `0`                                     | `1` = bypass cache |
