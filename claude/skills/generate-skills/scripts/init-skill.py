#!/usr/bin/env python3
"""init-skill.py — Scaffold a new Claude skill directory.

Usage:
    python3 init-skill.py <skill-name>
    python3 init-skill.py <skill-name> --path <target-directory>

Creates:
    <skill-name>/
    ├── SKILL.md
    ├── references/
    │   └── example.md
    ├── scripts/
    │   └── example.py
    └── assets/
        └── example_asset.txt
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

---

<!--
## Structuring This Skill (delete this block when done)

Follow Progressive Disclosure — 3 tiers:
  Tier 1: frontmatter above (always loaded, ~100 words)
  Tier 2: this SKILL.md body (loaded on trigger, max 500 lines)
  Tier 3: references/, scripts/, assets/ (loaded on demand, no limit)

Set Degrees of Freedom to match task fragility:
  Low    → "ALWAYS use this exact format"
  Medium → "Follow this structure, adapt as needed"
  High   → "Use your best judgment"

Do NOT include:
  - General knowledge Claude already knows
  - README.md, CHANGELOG.md, or other supporting docs
  - Tokens that don't change Claude's behavior
-->

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

REFERENCES_EXAMPLE = """\
# Example Reference

TODO: Replace with actual reference content.

This file is referenced from SKILL.md when needed:

    See references/example.md for details.

Remove this file if not needed.
"""

SCRIPTS_EXAMPLE = """\
#!/usr/bin/env python3
\"\"\"example.py — TODO: describe what this script does.

Usage:
    python3 example.py <argument>
\"\"\"

import sys


def main() -> int:
    # TODO: implement script logic
    print("TODO: implement this script")
    return 0


if __name__ == "__main__":
    sys.exit(main())
"""

ASSETS_EXAMPLE = """\
TODO: Replace or remove this file.

Placeholder for binary/media assets (images, diagrams, PDFs).
If assets are not needed, delete this file and the assets/ directory.
"""


def is_kebab_case(name: str) -> bool:
    """Validate kebab-case: lowercase letters/digits, single hyphens, no leading/trailing."""
    return bool(re.match(r"^[a-z0-9]+(-[a-z0-9]+)*$", name))


def create_skill(name: str, base_path: str) -> None:
    skill_dir = os.path.join(base_path, name)

    if os.path.exists(skill_dir):
        print(f"Error: Directory already exists: {skill_dir}", file=sys.stderr)
        sys.exit(1)

    # Create directories
    dirs = [
        skill_dir,
        os.path.join(skill_dir, "references"),
        os.path.join(skill_dir, "scripts"),
        os.path.join(skill_dir, "assets"),
    ]
    for d in dirs:
        os.makedirs(d, exist_ok=True)

    # SKILL.md
    skill_md_path = os.path.join(skill_dir, "SKILL.md")
    with open(skill_md_path, "w", encoding="utf-8") as f:
        f.write(SKILL_TEMPLATE.format(name=name))

    # references/example.md
    ref_path = os.path.join(skill_dir, "references", "example.md")
    with open(ref_path, "w", encoding="utf-8") as f:
        f.write(REFERENCES_EXAMPLE)

    # scripts/example.py (executable)
    script_path = os.path.join(skill_dir, "scripts", "example.py")
    with open(script_path, "w", encoding="utf-8") as f:
        f.write(SCRIPTS_EXAMPLE)
    os.chmod(script_path, 0o755)

    # assets/example_asset.txt
    asset_path = os.path.join(skill_dir, "assets", "example_asset.txt")
    with open(asset_path, "w", encoding="utf-8") as f:
        f.write(ASSETS_EXAMPLE)

    print(f"Created: {skill_dir}")
    print()
    print(f"  {name}/")
    print(f"  ├── SKILL.md")
    print(f"  ├── references/")
    print(f"  │   └── example.md")
    print(f"  ├── scripts/")
    print(f"  │   └── example.py")
    print(f"  └── assets/")
    print(f"      └── example_asset.txt")
    print()
    print("Next steps:")
    print("  1. Edit SKILL.md — fill in TODO placeholders, delete the comment block")
    print("  2. Remove example files you don't need")
    print("  3. Validate:")
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

    create_skill(args.name, args.path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
