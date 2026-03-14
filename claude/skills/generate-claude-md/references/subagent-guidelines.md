# Subagent Guidelines for CLAUDE.md Generation

> When and how to use subagents during CLAUDE.md/AGENTS.md generation. Tier 3 reference — load only when needed.

---

## Decision Criteria

Ask these 3 questions before spawning a subagent:

| # | Question | If Yes |
|---|----------|--------|
| 1 | Will the main agent accumulate 500+ tokens of raw data by doing this directly? | Subagent candidate |
| 2 | Can this run in parallel with user wait time or another task? | Subagent candidate |
| 3 | Can the result be delivered as a summary? | Subagent suitable |

**Rule**: 2+ Yes → use a subagent. All No → do it directly.

---

## Available Subagent Types

| Type | Strengths | Limitations |
|------|-----------|-------------|
| Explore | Fast codebase search, file pattern matching, keyword search | Cannot edit files or run arbitrary commands |
| general-purpose | Full tool access, autonomous multi-step tasks | Slower startup, heavier context |

**Default**: Prefer Explore for read-only investigation. Use general-purpose only when the task requires validation logic or multi-tool orchestration.

---

## Workflow Integration

### Explore-Config: Package & Build Detection (Stage 1)

**Trigger**: Target project has 3+ config file types (package.json, Cargo.toml, pyproject.toml, go.mod, etc.).

**When to skip**: Simple project with ≤2 config files visible from a single glob.

**Prompt template**:

```
Explore the project at {target_path} to find all package/build/test/lint/format configuration:
1. Package managers: package.json, Cargo.toml, pyproject.toml, go.mod, Gemfile, pom.xml
2. Test config: jest.config, vitest.config, pytest.ini, .mocharc
3. Lint/format: .eslintrc, .prettierrc, biome.json, ruff.toml, .golangci.yml, rustfmt.toml
4. Build: webpack, vite, tsconfig, Makefile, CMakeLists.txt, build.gradle

Report: list each found file with its key fields (scripts, dependencies count, test command).
Do NOT include file contents — summarize only.
```

**Agent parameters**:
- `subagent_type`: `Explore`
- `description`: "Detect project config"
- `run_in_background`: `true`

---

### Explore-Structure: Repository Layout (Stage 1)

**Trigger**: Project root contains workspaces config, .gitmodules, or 3+ top-level directories with their own package files.

**When to skip**: Flat single-package repository.

**Prompt template**:

```
Analyze repository structure at {target_path}:
1. Monorepo detection: workspaces in package.json/pnpm-workspace.yaml, packages/, apps/
2. Submodules: parse .gitmodules for paths, URLs, independent repo status
3. Nested package managers: subdirectories with their own package.json/Cargo.toml/etc.
4. Directory tree: top 2 levels only, noting purpose of major directories

Report: structure type (monorepo/single/hybrid), list of independent units with tech stack.
```

**Agent parameters**:
- `subagent_type`: `Explore`
- `description`: "Analyze repo structure"
- `run_in_background`: `true`

---

### Explore-Docs: Documentation & CI Analysis (Stage 1)

**Trigger**: Project has existing CLAUDE.md, AGENTS.md, .cursorrules, or CI configuration.

**When to skip**: No documentation files or CI config detected in initial glob.

**Prompt template**:

```
Scan documentation and CI at {target_path}:
1. Existing AI config: CLAUDE.md (root + nested), AGENTS.md, .cursorrules, .github/copilot
2. Contributing docs: CONTRIBUTING.md, contributing-docs/, docs/
3. CI/CD: .github/workflows/*.yml, .gitlab-ci.yml, Jenkinsfile — extract test/build/deploy commands
4. Nested CLAUDE.md: list all paths, note content length and key sections

Report: list of files found with one-line summary of each. For existing CLAUDE.md files, note section headings and line count.
```

**Agent parameters**:
- `subagent_type`: `Explore`
- `description`: "Scan docs and CI"
- `run_in_background`: `true`

---

### Explore-Deep: In-Depth Analysis (Stage 2)

**Trigger**: Large monorepo (5+ packages) or complex project where Stage 1 results raised unanswered questions.

**When to skip**: Stage 1 results are sufficient. Simple or medium-sized projects. AskUserQuestion response arrived quickly.

**Prompt template**:

```
Deep analysis of {target_path} based on Stage 1 gaps:
1. {specific_gap_1}: e.g., "Determine relationship between packages/core and packages/cli"
2. {specific_gap_2}: e.g., "Find external service dependencies (DB connections, API calls)"
3. Cross-package dependencies: which packages depend on which
4. Non-obvious patterns: custom build steps, code generation, unusual testing patterns

Report: findings for each gap, with file:line references where relevant.
```

**Agent parameters**:
- `subagent_type`: `Explore`
- `description`: "Deep project analysis"
- `run_in_background`: `true` (runs during AskUserQuestion wait)

---

### Reviewer: Independent Blind Validation (Stage 4)

**Trigger**: Generated output includes AGENTS.md, contributing-docs/, or nested CLAUDE.md files (i.e., more than a single root CLAUDE.md).

**When to skip**:
- Only a single root CLAUDE.md was generated
- User explicitly requested fast generation

**What to provide**: Generated file contents only. Do NOT provide Stage 1/2 analysis results — the reviewer must evaluate independently.

**What NOT to provide**: Stage 1 detection results, user interview answers, internal reasoning. This ensures blind review.

**Prompt template**:

```
You are reviewing generated CLAUDE.md and related files. You did NOT write these.
Review independently using these criteria:

Files to review: {list_of_generated_file_paths}

1. Discoverability: Does each line pass the test "Can an agent learn this by reading the code?" If yes → flag for removal
2. Staleness risk: Does any line reference specific versions, tool names, or dependencies that may become inaccurate within 6 months? → flag with reason
3. Redundancy: Is any content duplicated between CLAUDE.md, AGENTS.md, and contributing-docs/? → flag the duplicate
4. Hierarchy: Does CLAUDE.md reference contributing-docs/ directly (should go through AGENTS.md)? → flag
5. Nested CLAUDE.md: Does any nested file repeat content from root CLAUDE.md? Does scope exceed its directory? → flag
6. Size: Is root CLAUDE.md under 100 lines (hard limit 300)? Nested under 50 (hard limit 100)?
7. Actionability: Is every instruction verifiable? Any vague guidance? → flag with suggestion

Report: PASS/FAIL per criterion. For each FAIL, quote the specific line and explain why.
Do NOT fix issues — only report them.
```

**Agent parameters**:
- `subagent_type`: `general-purpose`
- `description`: "Blind review generated files"
- `run_in_background`: `false` (must receive results before final report)

---

## Parallelism Rules

### Safe to parallelize

- Explore-Config + Explore-Structure + Explore-Docs (Stage 1, all independent read-only)
- Explore-Deep + AskUserQuestion wait (Stage 2, independent)

### Must NOT parallelize

- Reviewer + final user report (reviewer results must be incorporated first)
- Stage 1 Explore agents with Stage 2 analysis (Stage 2 depends on Stage 1 results)
- Multiple agents querying the exact same file set (redundant work)

### Execution timeline

```
Stage 1: [Main]           Glob check → decide agents
         [Explore-Config] Config detection ──── done ──┐
         [Explore-Struct] Structure analysis ── done ──┤
         [Explore-Docs]   Docs/CI scan ──────── done ──┤
         [Main]           Merge results, present to user

Stage 2: [Main]           AskUserQuestion ─── wait ──── response received
         [Explore-Deep]   Deep analysis ────── done ──┘ (optional)

Stage 3: [Main only]      Generate files

Stage 4: [Main]           Self-test checklist ── done
         [Reviewer]       Independent review ── results ──┐
         [Main]           Incorporate review, final report
```

---

## Anti-patterns

| Anti-pattern | Why it's wrong | Do instead |
|--------------|----------------|------------|
| Spawning 3 Explore agents for a 5-file project | Overhead exceeds benefit | Read files directly |
| Running Explore-Deep on every project | Most projects don't need it | Only when Stage 1 leaves gaps in large repos |
| Giving Reviewer the Stage 1/2 analysis | Breaks blind review independence | Provide only generated file contents |
| Letting Reviewer fix issues it finds | Breaks review/fix separation | Main agent fixes based on report |
| Parallelizing Reviewer with final report | Results arrive too late to incorporate | Run Reviewer before final report |
| Spawning general-purpose for read-only tasks | Unnecessary tool access, slower | Use Explore instead |
