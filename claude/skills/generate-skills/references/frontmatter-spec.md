# YAML Frontmatter 필드 명세

> SKILL.md 상단 YAML frontmatter의 필드별 규칙과 유효성 검사 기준.

---

## 구분자 규칙

frontmatter는 파일 최상단에 위치하며, `---` 구분자로 감싼다:

```yaml
---
name: my-skill
description: 이 스킬이 무엇을 한다. 어떤 상황에서 사용한다.
---
```

- 첫 번째 `---`는 반드시 파일의 **1행**이어야 한다 (앞에 빈 줄 불가)
- 두 번째 `---`로 frontmatter를 닫는다
- 구분자 앞뒤에 공백 불가

---

## 필수 필드

### `name`

스킬의 고유 식별자.

| 규칙 | 설명 |
|------|------|
| 형식 | kebab-case (`^[a-z0-9]+(-[a-z0-9]+)*$`) |
| 폴더명 일치 | 스킬 폴더명과 **반드시 동일** |
| 금지 접두사 | `claude-*`, `anthropic-*` |
| 예시 | `generate-skills`, `notion-setup`, `tdd-workflow` |

### `description`

스킬이 무엇을 하고 언제 사용하는지 기술.

| 규칙 | 설명 |
|------|------|
| 최대 길이 | 1024자 |
| 필수 구성 | **WHAT** (무엇을 하는가) + **WHEN** (언제 사용하는가) |
| XML 태그 | `< >` 사용 금지 |
| 언어 | 프로젝트 언어 정책에 따름 (한국어 또는 영어) |

description은 시스템이 자연어 매칭에 사용하는 핵심 필드이다. 트리거 정확도에 직접 영향을 미친다.

---

## 선택 필드

### `model`

스킬 실행 시 사용할 모델 지정.

```yaml
model: opus
```

- 허용 값: `opus`, `sonnet`, `haiku`
- 미지정 시 현재 세션 모델 사용
- 복잡한 워크플로우나 창의적 작업에는 `opus` 권장

### `disable-model-invocation`

`true`로 설정하면 자동 트리거를 비활성화하고, 사용자가 명시적으로 호출해야만 동작한다.

```yaml
disable-model-invocation: true
```

- 기본값: `false` (자동 트리거 허용)
- 파괴적 작업이나 비용이 높은 스킬에 권장

### `license`

```yaml
license: MIT
```

### `metadata`

자유 형식의 추가 메타데이터.

```yaml
metadata:
  author: ujuc
  version: 1.0.0
  mcp-server: custom-server
```

---

## 유효성 검사 체크리스트

frontmatter 작성 후 다음을 확인한다:

- [ ] 파일 1행이 `---`인가
- [ ] 닫는 `---`가 존재하는가
- [ ] `name` 필드가 존재하는가
- [ ] `name`이 kebab-case 정규식(`^[a-z0-9]+(-[a-z0-9]+)*$`)에 매칭하는가
- [ ] `name`이 폴더명과 일치하는가
- [ ] `name`이 `claude` 또는 `anthropic`으로 시작하지 않는가
- [ ] `description` 필드가 존재하는가
- [ ] `description`이 1024자 이하인가
- [ ] `description`에 XML 태그(`< >`)가 없는가
- [ ] `description`에 WHAT + WHEN이 모두 포함되어 있는가
