# 스킬 검토 체크리스트

> 자동 검증(`scripts/validate-skill.sh`)이 잡지 못하는 의미·트리거 검토만 다룬다. 형식 규칙(kebab-case, 길이, 예약 접두사 등)은 자동 검증으로 처리되니 여기서 다시 적지 않는다.

상세 사양은 다음을 참조한다:

- frontmatter 필드: `references/frontmatter-spec.md`
- description 작성법·예시: `references/description-examples.md`
- 폴더 구조 규칙: `references/skill-structure.md`

---

## 트리거 튜닝

### 과소 트리거 (스킬이 로드되지 않음)

증상:
- 사용자가 관련 요청을 해도 스킬이 자동 로드되지 않음
- 매번 수동으로 `/skill-name`을 입력해야 함

해결:
- description에 사용자가 **실제로 말할 법한 표현** 추가
- 동의어, 줄임말, 구어체 표현 포함
- 예: "PR 만들어줘", "풀리퀘 생성", "pull request"

### 과잉 트리거 (무관한 요청에도 로드됨)

증상:
- 관련 없는 작업에도 스킬이 로드됨
- 사용자가 스킬을 비활성화함

해결:
- description에 부정 트리거 추가: "Do NOT use for simple data exploration"
- 범위를 더 구체적으로 제한
- 지나치게 일반적인 단어 제거 (예: "help", "manage")

---

## 의미 검토 체크리스트

자동 검증은 형식만 본다. 다음은 사람의 판단이 필요한 항목이다.

### description

- [ ] WHAT(무엇을 하는가)이 명시되어 있는가
- [ ] WHEN(언제 사용하는가)이 명시되어 있는가
- [ ] 사용자가 실제로 쓸 법한 트리거 문구가 들어 있는가
- [ ] 너무 일반적인 단어("help", "manage")로 시작해 과잉 트리거 위험이 없는가

### 본문 지시사항

- [ ] 각 단계가 실행 가능한 명령·기준으로 구체화되었는가
- [ ] 실패 시나리오와 대응 방법이 포함되었는가
- [ ] 입력/출력 예시가 있는가
- [ ] 사용하는 도구(Read, Bash, AskUserQuestion 등)가 명시되었는가

### 구조

- [ ] 모든 `references/` 경로가 실제 존재하는 파일을 가리키는가
- [ ] 본문이 500줄을 넘으면 `references/`로 분리할 후보가 있는가
