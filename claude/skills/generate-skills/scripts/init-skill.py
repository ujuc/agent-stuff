#!/usr/bin/env python3
"""init-skill.py — Scaffold a new Claude skill directory.

Usage:
    python3 init-skill.py <skill-name>
    python3 init-skill.py <skill-name> --path <target-directory>

Creates:
    <skill-name>/
    └── SKILL.md

Pass --with-references / --with-scripts / --with-assets to add empty
subdirectories (with .gitkeep). Optional folders are not created by default —
add them only when the skill actually needs Tier 3 resources.
"""

import argparse
import os
import re
import sys

DEFAULT_PATH = "agents/claude/skills"

SKILL_TEMPLATE = """\
---
name: {name}
description: |
  TODO: What does this skill do? (WHAT)
  TODO: When should it trigger? Include example phrases. (WHEN)
# model: opus          # Uncomment for complex multi-step workflows
# disable-model-invocation: true  # Uncomment for destructive/high-cost skills
---

# TODO: Skill Title

TODO: One-line description of what this skill does.

$ARGUMENTS 가 주어지면 해당 경로/이름을 대상으로 한다. 없으면 사용자에게 확인한다.

## 1단계: TODO

TODO: Describe the first step.

---

## 2단계: TODO

TODO: Describe the second step.

---

## 검증

TODO: Describe how to verify the output.

```bash
# TODO: validation command
```
"""


def is_kebab_case(name: str) -> bool:
    """Validate kebab-case: lowercase letters/digits, single hyphens, no leading/trailing."""
    return bool(re.match(r"^[a-z0-9]+(-[a-z0-9]+)*$", name))


def create_skill(
    name: str,
    base_path: str,
    with_references: bool,
    with_scripts: bool,
    with_assets: bool,
) -> None:
    skill_dir = os.path.join(base_path, name)

    if os.path.exists(skill_dir):
        print(f"Error: Directory already exists: {skill_dir}", file=sys.stderr)
        sys.exit(1)

    os.makedirs(skill_dir, exist_ok=True)

    # SKILL.md
    skill_md_path = os.path.join(skill_dir, "SKILL.md")
    with open(skill_md_path, "w", encoding="utf-8") as f:
        f.write(SKILL_TEMPLATE.format(name=name))

    optional_dirs = []
    for flag, dirname in (
        (with_references, "references"),
        (with_scripts, "scripts"),
        (with_assets, "assets"),
    ):
        if not flag:
            continue
        sub_dir = os.path.join(skill_dir, dirname)
        os.makedirs(sub_dir, exist_ok=True)
        gitkeep = os.path.join(sub_dir, ".gitkeep")
        with open(gitkeep, "w", encoding="utf-8"):
            pass
        optional_dirs.append(dirname)

    print(f"Created: {skill_dir}")
    print()
    print(f"  {name}/")
    print(f"  └── SKILL.md")
    for d in optional_dirs:
        print(f"      {d}/  (empty)")
    print()
    print("Next steps:")
    print("  1. Edit SKILL.md — fill in TODO placeholders")
    print("  2. Validate:")
    print(
        f"     bash agents/claude/skills/generate-skills/scripts/validate-skill.sh {skill_dir}"
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Scaffold a new Claude skill directory.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument(
        "name",
        help="Skill name in kebab-case (e.g., my-new-skill)",
    )
    parser.add_argument(
        "--path",
        default=DEFAULT_PATH,
        help=f"Target parent directory (default: {DEFAULT_PATH})",
    )
    parser.add_argument(
        "--with-references",
        action="store_true",
        help="Create empty references/ subdirectory",
    )
    parser.add_argument(
        "--with-scripts",
        action="store_true",
        help="Create empty scripts/ subdirectory",
    )
    parser.add_argument(
        "--with-assets",
        action="store_true",
        help="Create empty assets/ subdirectory",
    )
    args = parser.parse_args()

    if not is_kebab_case(args.name):
        print(f"Error: name must be kebab-case: '{args.name}'", file=sys.stderr)
        print("  Valid:   my-skill, generate-skills, notion-setup", file=sys.stderr)
        print("  Invalid: MySkill, my_skill, -my-skill, my--skill", file=sys.stderr)
        return 1

    if len(args.name) > 64:
        print(
            f"Error: name must be 64 characters or fewer (got {len(args.name)})",
            file=sys.stderr,
        )
        return 1

    create_skill(
        args.name,
        args.path,
        args.with_references,
        args.with_scripts,
        args.with_assets,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
