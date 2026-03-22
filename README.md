# Agent Stuff

AI 에이전트 설정을 모아두는 개인 저장소. [dotrc](https://github.com/ujuc/dotrc)의 git 서브모듈로 관리되며, 각 도구의 시스템 경로에 심링크로 배포된다.

## 구조

| 소스 | 대상 | 상태 |
| -------- | ----------- | ----------- |
| `claude/` | `~/.claude` | 활성 |

`rules/SOUL.md`에 에이전트 공통 미션과 가치관을 정의한다.

## 설치

부모 저장소([dotrc](https://github.com/ujuc/dotrc))에서 서브모듈을 초기화한다:

```bash
git submodule update --init --recursive
```

## 라이선스

[MIT](LICENSE)
