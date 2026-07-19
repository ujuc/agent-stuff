# Stage 1: Project Analyzer

> Defines how 3 parallel Explore agents analyze a target project. Explore agents
> are read-only (no Write/Edit tool) — findings return as each agent's final
> message, which the orchestrator receives as the Agent tool result.
> Tier 2 reference — loaded during Stage 1 execution.

---

## Complexity Assessment

Run a quick glob before spawning agents to determine whether the project is complex or simple.

### Complex Project (spawn 3 agents)

Criteria — any one sufficient:

- 3 or more distinct config file types present (package.json, Cargo.toml, pyproject.toml, go.mod, Gemfile, pom.xml, etc.)
- Monorepo detected: `workspaces` field in package.json/pnpm-workspace.yaml, or `packages/`, `apps/` directories with child package files
- Submodule detected: `.gitmodules` file present
- 3 or more top-level directories each containing their own package file

### Simple Project (detect directly, no agents)

All of the following:

- 2 or fewer distinct config file types
- No monorepo indicators
- No `.gitmodules`
- Single top-level package or flat configuration directory

For simple projects, read files directly and proceed to the Merge Protocol section.

---

## Agent Definitions

Spawn all three agents **in one message** with `run_in_background: true`. They
are fully independent read-only tasks.

**Collection rule**: the `Explore` agent type has no Write/Edit tool — never
instruct these agents to write files. Each returns its findings as its final
message; the orchestrator collects them from the Agent tool results (use
`TaskOutput` for background runs). If an agent dies or returns nothing, note
the gap and continue with what exists.

### Agent 1: config-explorer

| Parameter       | Value                              |
| --------------- | ---------------------------------- |
| subagent_type   | Explore                            |
| model           | sonnet                             |
| run_in_background | true                             |
| description     | Detect project config files        |

**Skip condition**: Project has 2 or fewer config files visible from a single glob.

**Prompt template**:

```
Explore the project at {target_path} to find all package/build/test/lint/format configuration.

Look for:
1. Package managers: package.json, Cargo.toml, pyproject.toml, go.mod, Gemfile, pom.xml
2. Test config: jest.config.*, vitest.config.*, pytest.ini, .mocharc.*, setup.cfg (testpaths)
3. Lint/format: .eslintrc.*, .prettierrc.*, biome.json, ruff.toml, .golangci.yml, rustfmt.toml
4. Build: webpack.config.*, vite.config.*, tsconfig.json, Makefile, CMakeLists.txt, build.gradle

For each found file, record:
- File path
- Key fields (scripts section, dependencies count, test command, main entry point)
Do NOT include raw file contents — summarize only.

Return your findings as your final message — raw markdown, no preamble — using this structure:

# Config Explorer Findings

## Found Config Files
| File | Key Fields |
|------|------------|
| ... | ... |

## Detected Commands
- Build: ...
- Test: ...
- Lint/Format: ...

## Notes
(anything unusual about the config setup)
```

---

### Agent 2: structure-explorer

| Parameter       | Value                              |
| --------------- | ---------------------------------- |
| subagent_type   | Explore                            |
| model           | sonnet                             |
| run_in_background | true                             |
| description     | Analyze repository structure       |

**Skip condition**: Flat single-package repository with no workspace or submodule indicators.

**Prompt template**:

```
Analyze the repository structure at {target_path}.

Investigate:
1. Monorepo detection: check for workspaces field in package.json/pnpm-workspace.yaml, presence of packages/, apps/ directories with their own package files
2. Submodules: parse .gitmodules for paths, URLs, and infer independent repo status from remote URL patterns
3. Nested package managers: subdirectories that contain their own package.json/Cargo.toml/pyproject.toml/go.mod
4. Directory tree: top 2 levels only, noting the apparent purpose of each major directory

Report: structure type (monorepo / single-package / hybrid / config-only), list of independent units with their detected tech stack.
Do NOT recursively read all files — observe patterns from names and top-level structure.

Return your findings as your final message — raw markdown, no preamble — using this structure:

# Structure Explorer Findings

## Repository Type
(monorepo / single-package / hybrid / config-only)

## Independent Units
| Path | Type | Tech Stack |
|------|------|------------|
| ... | ... | ... |

## Directory Tree (top 2 levels)
(annotated list)

## Submodules
| Path | Remote URL | Independent? |
|------|------------|--------------|
| ... | ... | ... |

## Notes
(anything unusual about the repo layout)
```

---

### Agent 3: docs-explorer

| Parameter       | Value                              |
| --------------- | ---------------------------------- |
| subagent_type   | Explore                            |
| model           | sonnet                             |
| run_in_background | true                             |
| description     | Scan documentation and CI          |

**Skip condition**: No documentation files (CLAUDE.md, AGENTS.md, .cursor/rules/, CONTRIBUTING.md) and no CI config detected in initial glob.

**Prompt template**:

```
Scan documentation and CI configuration at {target_path}.

Look for:
1. Existing AI config: CLAUDE.md (root and all nested locations), AGENTS.md, .cursor/rules/*.mdc, .github/copilot-instructions.md
2. Contributing docs: CONTRIBUTING.md, contributing-docs/, docs/
3. CI/CD: .github/workflows/*.yml, .gitlab-ci.yml, Jenkinsfile, .circleci/config.yml — extract test, build, and deploy commands
4. Nested CLAUDE.md files: list all paths, record section headings and approximate line count

For each found file, provide a one-line summary.
For existing CLAUDE.md files, note section headings and line count.
Do NOT include full file contents.

Return your findings as your final message — raw markdown, no preamble — using this structure:

# Docs Explorer Findings

## AI Config Files
| File | Line Count | Section Headings |
|------|------------|-----------------|
| ... | ... | ... |

## Contributing Docs
| File | Summary |
|------|---------|
| ... | ... |

## CI/CD Pipelines
| File | Test Command | Build Command | Deploy Target |
|------|-------------|---------------|---------------|
| ... | ... | ... | ... |

## Notes
(anything unusual: conflicting instructions between AI configs, deprecated sections, etc.)
```

---

## Optional: Explore-Deep (Stage 2 overlap)

**Trigger**: Large monorepo (5+ packages) or Stage 1 results leave unresolved structural questions.

**Timing**: Spawn during Stage 2 AskUserQuestion wait — runs in parallel with user response time.

**Skip condition (broad)**: Stage 1 results are sufficient, project is small or medium, or user response arrives quickly.

| Parameter       | Value                              |
| --------------- | ---------------------------------- |
| subagent_type   | Explore                            |
| model           | sonnet                             |
| run_in_background | true                             |
| description     | Deep project analysis              |

**Prompt template**:

```
Deep analysis of {target_path} based on Stage 1 gaps.

Address these specific open questions:
1. {specific_gap_1} — e.g., "Determine relationship between packages/core and packages/cli"
2. {specific_gap_2} — e.g., "Find external service dependencies (DB connections, API calls)"
3. Cross-package dependencies: which packages import or depend on which others
4. Non-obvious patterns: custom build steps, code generation, unusual testing patterns

Provide file:line references wherever relevant.
Do NOT reproduce raw file contents — cite and summarize.

Return your findings as your final message — raw markdown, no preamble — using this structure:

# Deep Explorer Findings

## Gap Resolutions
### {gap_1_title}
...
### {gap_2_title}
...

## Cross-Package Dependencies
| From | To | Nature |
|------|-----|--------|
| ... | ... | ... |

## Non-Obvious Patterns
(list with file:line citations)
```

---

## Merge Protocol

After all launched agents complete, execute the following steps before advancing to Stage 2.

### Step 1: Collect Findings

Gather each launched agent's final message from its Agent tool result
(`TaskOutput` for background runs):

- config-explorer findings
- structure-explorer findings
- docs-explorer findings
- Explore-Deep findings (if spawned)

If a result is missing (agent skipped, died, or returned nothing), record the
gap explicitly and continue with what exists.

### Step 2: Classify Discoverability

For every detected fact, classify it:

| Class | Definition | Action |
|-------|-----------|--------|
| **Discoverable** | An agent can learn this by reading code or config files (e.g., "uses TypeScript", "test command is `npm test`") | Exclude from CLAUDE.md candidates |
| **Undiscoverable** | Requires explicit documentation for an agent to know (e.g., "this monorepo package must always be released together", "never modify generated files in dist/") | Include in CLAUDE.md candidates |

### Step 3: Separate Facts from Assumptions

- **Fact**: Directly observed from a file (cite source)
- **Assumption**: Inferred from patterns or absence of evidence — mark with `[ASSUMPTION]` and verify in Stage 2 interview

### Step 4: List Stage 2 Questions

Enumerate items that Stage 1 could not determine. These become Stage 2 interview questions.

### Step 5: Prepare Nested CLAUDE.md Candidate Table

If the structure-explorer detected monorepo packages or submodules, prepare this table for the Stage 2 interview:

| Path | Type | Existing CLAUDE.md | Recommended |
|------|------|-------------------|-------------|
| e.g., packages/core | monorepo package | No | Yes — if package has distinct agent workflow rules |
| e.g., tools/cli | monorepo package | Yes (12 lines) | Update — existing file is outdated |
| e.g., infra/ | submodule | No | No — infra rules belong in root CLAUDE.md |

### Step 6: Present Summary to User

Show the merged analysis before proceeding to Stage 2. Include:

1. Detected project type and structure
2. Discoverable vs undiscoverable classification (brief list)
3. Facts vs assumptions (flag assumptions clearly)
4. Items that need Stage 2 questions
5. Nested CLAUDE.md candidate table (if applicable)

Nothing is persisted to disk — the merged analysis exists only in the main
agent's context. There is no cleanup step.
