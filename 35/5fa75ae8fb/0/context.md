# Session Context

## User Prompts

### Prompt 1

Implement the following plan:

# Plan: Move template/guide files from docs/ to spec-design/

## Context

The files in `docs/` (`common-template.md`, `writing-guide.md`) are user-authored design specifications — rules and templates that AI agents reference when generating documents. `docs/` should be reserved for AI-generated output. These specification files need a new home: `spec-design/`.

## Changes

### 1. Create `spec-design/` directory and move files

- `docs/common-template.md` → `spe...

