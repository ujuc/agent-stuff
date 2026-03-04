# 스킬 폴더/파일 구조 규칙

> 스킬 디렉토리를 올바르게 구성하기 위한 상세 가이드.

---

## 최소 구조

```
my-skill/
└── SKILL.md
```

모든 스킬은 반드시 `SKILL.md` 파일 하나를 포함해야 한다. 이 파일이 스킬의 진입점이다.

## 확장 구조

```
my-skill/
├── SKILL.md              # 필수: 프론트매터 + 지시사항
├── references/           # 선택: 상세 문서, 예시, 체크리스트
│   ├── api-guide.md
│   └── examples/
│       └── basic.md
├── scripts/              # 선택: 검증·유틸리티 스크립트
│   └── validate.sh
└── assets/               # 선택: 이미지, 다이어그램, PDF
    └── architecture.png
```

---

## 폴더명 규칙

- **kebab-case** 필수: `my-skill`, `notion-project-setup`, `generate-skills`
- 대문자, 언더스코어(`_`), 공백 금지
- 폴더명은 SKILL.md 프론트매터의 `name` 필드와 **반드시 일치**해야 한다
- 의미 있는 이름 사용: 스킬이 하는 일을 유추할 수 있어야 한다

---

## 하위 폴더 용도

### `references/`

SKILL.md 본문에서 참조하는 상세 문서를 배치한다.

- 체크리스트, 명세서, 예시 모음 등
- SKILL.md 본문이 5,000단어를 초과할 경우 반드시 분리
- SKILL.md에서 상대 경로로 참조: `references/api-guide.md`
- 스킬 트리 외부의 파일(프로젝트 루트 문서, 다른 서브모듈 등)에 의존하는 경우, 해당 파일을 `references/`에 복사하여 스킬이 단독으로 동작하도록 한다

### `scripts/`

자동화·검증 스크립트를 배치한다.

- 셸 스크립트, Python 스크립트 등
- 실행 권한(`chmod +x`) 부여 필수
- SKILL.md에서 실행 명령 포함 시 경로 명시

### `assets/`

이미지, 다이어그램, PDF 등 바이너리/미디어 파일을 배치한다.

- SKILL.md나 references에서 참조하는 시각 자료
- 텍스트로 대체 가능하면 텍스트를 우선 사용

---

## Progressive Disclosure 원칙

스킬은 3단계로 정보를 계층 분리한다. 에이전트가 한 번에 모든 정보를 로드하지 않도록 한다.

### Tier 1: 메타데이터 (항상 로딩)

- `name` + `description` 필드만 포함
- 시스템이 트리거 판단에 사용
- 목표: ~100단어 이하

### Tier 2: SKILL.md 본문 (트리거 시 로딩)

- 핵심 워크플로우와 실행 지시사항
- 한계: **5,000단어 이하 / 500줄 이하**
- 초과 시 Tier 3으로 분리

### Tier 3: 번들 리소스 (필요시 로딩)

- `references/`: 상세 규칙, 예시, 체크리스트
- `scripts/`: 자동화·검증 스크립트
- `assets/`: 이미지, 다이어그램, PDF
- 크기 제한 없음

---

## 금지 항목

| 항목 | 이유 |
|------|------|
| `README.md` 포함 | 스킬 폴더에 README를 넣으면 안 됨. SKILL.md가 유일한 진입점 |
| `claude` 접두사 (`claude-my-skill`) | 예약된 네임스페이스 |
| `anthropic` 접두사 (`anthropic-helper`) | 예약된 네임스페이스 |
| 폴더명과 `name` 불일치 | 스킬 로딩 실패 원인 |
| SKILL.md 외 진입점 | `main.md`, `index.md` 등 사용 불가 |
