# Global Memory

## Shell Command Rules

- Bash 도구에서 `cd`를 사용하지 말 것 — working directory가 호출 간에 유지되어 후속 명령이 실패할 수 있음
- 항상 **절대 경로**를 사용할 것
  - `git -C /absolute/path status` (O)
  - `cd some-dir && git status` (X)
