# Session Context

## User Prompts

### Prompt 1

Implement the following plan:

# YAML Front Matter 언어 수정 계획

## Context

이전 작업에서 Sonnet subagent들이 `docs/` 하위 16개 파일의 YAML front matter를 한국어로 작성했다. 원인은 `spec-design/writing-guide.md`의 언어 정책이 `common-template.md`의 기본 규칙과 충돌하기 때문이다.

**충돌 원인**:
- `common-template.md` (line 21): "모든 내용은 명시하지 않는한 영어로 작성한다" → `description`만 한국어 명시
- `writi...

