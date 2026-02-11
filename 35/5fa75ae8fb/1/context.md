# Session Context

## User Prompts

### Prompt 1

[@common-template.md](file:///Users/ujuc/repos/agent-stuff/docs/common-template.md)

 은 Agent에서 사용하는 문서들을 관리하기 위해서 사용되는 yaml frontmatter 내용에 대해서 정의해놨어. 혹시 제인해줄 것이 있을까?


<context ref="file:///Users/ujuc/repos/agent-stuff/docs/common-template.md">
# 공통 템플릿

## 사용법

공통으로 사용하는 부분에 대한 내용을 정의하는데 사용한다.

## 서식

### YAML Front Matter

```yaml
---
n...

### Prompt 2

이야기해준 것에서 받아들여지는 것들을 추가했어.

### Prompt 3

어 해줘. 그리고 필드 전부 필수에 대한 항목을 명시해줘.

### Prompt 4

Agent를 이용해서 문서를 작성할때 

[@common-template.md](file:///Users/ujuc/repos/agent-stuff/docs/common-template.md)

 을 사용하여 문서에 대한 내역을 표시할 수 있도록 명시할 수 있는 방안을 미련해줘.


<context ref="file:///Users/ujuc/repos/agent-stuff/docs/common-template.md">
# 공통 템플릿

## 사용법

공통으로 사용하는 부분에 대한 내용을 정의하는데 사용한다.

## 서식

### YAML Front Matter

```yaml
---
name: [문�...

### Prompt 5

metadata에서 내부 필드는 필수가 아니라 예제인데 들어간거같아. 해당 부분은 제거해줘.

### Prompt 6

[@CLAUDE.md](file:///Users/ujuc/repos/agent-stuff/CLAUDE.md)

 에 내용이 300줄을 넘어갈 경우, 사용자에게 분리가 필요하다고 알림을 줄 수 있도록 가이드를 추가해줘.


<context ref="file:///Users/ujuc/repos/agent-stuff/CLAUDE.md">
# CLAUDE.md

This file provides guidance to AI agents when working with code in this repository.

## Project Overview

Utilities and configurations for AI agents.

**Status**: Early skeleton — no source code, build system, or depen...

### Prompt 7

문서에 대해서 작업한 내용들을 커밋해줘.

### Prompt 8

Base directory for this skill: /Users/ujuc/.claude/skills/commit

# Git Commit Skill

This skill creates git commits following the team's version control guidelines.

## Source of Truth

- **Commit Template**: [`gitmessage`](../../gitmessage)
- **Guidelines**: [`version-control.md`](../guides/version-control.md)

## Commit Message Principles

When creating commits, follow these core principles:

- **Intent focused**: Explain WHY the change was made, not just what changed
- **Context aware**: Inc...

