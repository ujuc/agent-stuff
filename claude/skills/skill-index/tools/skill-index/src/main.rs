use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const GROUPS: &[(&str, &str)] = &[
    ("planning", "🧭 기획·스펙"),
    ("analysis", "📐 분석·계획"),
    ("build", "🛠 구현·실행"),
    ("verify", "✅ 검증·QA"),
    ("docs", "📝 문서·커밋"),
    ("writing", "✍️ 글쓰기"),
    ("llm", "🤖 외부 LLM"),
    ("meta", "🧪 메타·관리"),
];

const ALIASES: &[(&str, &str)] = &[
    ("기획", "planning"),
    ("스펙", "planning"),
    ("분석", "analysis"),
    ("계획", "analysis"),
    ("구현", "build"),
    ("실행", "build"),
    ("검증", "verify"),
    ("qa", "verify"),
    ("문서", "docs"),
    ("커밋", "docs"),
    ("글쓰기", "writing"),
    ("쓰기", "writing"),
    ("외부", "llm"),
    ("외부llm", "llm"),
    ("메타", "meta"),
    ("관리", "meta"),
];

const WORKFLOWS: &[(&str, &[&str])] = &[
    (
        "새 프로젝트",
        &[
            "spec-planner",
            "sprint-contract-negotiator",
            "annotate-plan",
            "implement-plan",
            "qa-evaluator",
            "commit",
        ],
    ),
    (
        "기존 코드",
        &["deep-read", "annotate-plan", "implement-plan", "commit"],
    ),
    (
        "스킬 정비",
        &["skill-improver", "generate-skills", "maintain", "eos"],
    ),
    ("글쓰기", &["prompting-assist", "humanizer"]),
    ("디자인", &["frontend-design-evaluator", "multi-agent-orchestrator"]),
];

#[derive(Debug)]
struct Entry {
    name: String,
    description: String,
    group: String,
    is_plugin: bool,
}

fn skills_root() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let p = PathBuf::from(&home).join(".claude/skills");
        if p.exists() {
            return p;
        }
    }
    PathBuf::from(".")
}

fn plugin_groups_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("plugin-groups.toml")
}

fn parse_frontmatter(content: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let lines: Vec<&str> = content.lines().collect();
    if lines.first().map(|l| l.trim()) != Some("---") {
        return map;
    }
    let mut i = 1;
    while i < lines.len() {
        let line = lines[i];
        if line.trim() == "---" {
            break;
        }
        let starts_with_indent = line.starts_with(' ') || line.starts_with('\t');
        if !starts_with_indent {
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim().to_string();
                let val = v.trim();
                if val == "|" || val == ">" {
                    let mut buf = String::new();
                    i += 1;
                    while i < lines.len() {
                        let l = lines[i];
                        if l.trim() == "---" {
                            if i > 0 {
                                i -= 1;
                            }
                            break;
                        }
                        let l_indented = l.starts_with(' ') || l.starts_with('\t');
                        if !l_indented && !l.trim().is_empty() {
                            if i > 0 {
                                i -= 1;
                            }
                            break;
                        }
                        if !buf.is_empty() {
                            buf.push(' ');
                        }
                        buf.push_str(l.trim());
                        i += 1;
                    }
                    map.insert(key, buf);
                } else if val.len() >= 2 && val.starts_with('"') && val.ends_with('"') {
                    map.insert(key, val[1..val.len() - 1].to_string());
                } else if val.len() >= 2 && val.starts_with('\'') && val.ends_with('\'') {
                    map.insert(key, val[1..val.len() - 1].to_string());
                } else if !val.is_empty() {
                    map.insert(key, val.to_string());
                }
            }
        }
        i += 1;
    }
    map
}

fn load_skills(root: &Path) -> Vec<Entry> {
    let mut out = Vec::new();
    let dirs = match fs::read_dir(root) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("warn: cannot read {}: {}", root.display(), e);
            return out;
        }
    };
    for entry in dirs.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }
        let content = match fs::read_to_string(&skill_md) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let fm = parse_frontmatter(&content);
        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();
        let name = fm.get("name").cloned().unwrap_or_else(|| dir_name.clone());
        let desc = fm.get("description").cloned().unwrap_or_default();
        let group = fm
            .get("group")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        if group == "unknown" {
            eprintln!("warn: {} has no `group` frontmatter field", name);
        }
        out.push(Entry {
            name,
            description: desc,
            group,
            is_plugin: false,
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn parse_plugin_toml(path: &Path) -> Vec<Entry> {
    let mut out = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return out,
    };
    let mut current_section: Option<String> = None;
    let mut in_array = false;
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !in_array && line.starts_with('[') && line.ends_with(']') {
            current_section = Some(line[1..line.len() - 1].to_string());
            continue;
        }
        if !in_array && line.contains("commands") && line.contains('=') && line.contains('[') {
            in_array = true;
            let open = line.find('[').unwrap();
            let close = line.rfind(']');
            let inner = match close {
                Some(c) if c > open => {
                    in_array = false;
                    &line[open + 1..c]
                }
                _ => &line[open + 1..],
            };
            push_array_items(inner, current_section.as_deref(), &mut out);
            continue;
        }
        if in_array {
            if line.starts_with(']') {
                in_array = false;
                continue;
            }
            push_array_items(line, current_section.as_deref(), &mut out);
        }
    }
    out
}

fn push_array_items(s: &str, section: Option<&str>, out: &mut Vec<Entry>) {
    let Some(section) = section else { return };
    for token in s.split(',') {
        let t = token
            .trim()
            .trim_end_matches(']')
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .trim();
        if t.is_empty() {
            continue;
        }
        out.push(Entry {
            name: t.to_string(),
            description: String::new(),
            group: section.to_string(),
            is_plugin: true,
        });
    }
}

fn truncate_chars(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    let mut t: String = chars.iter().take(max).collect();
    t.push('…');
    t
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("").trim()
}

fn resolve_alias(term: &str) -> Option<String> {
    let lower = term.to_lowercase();
    for (slug, _) in GROUPS {
        if slug.eq_ignore_ascii_case(term) {
            return Some((*slug).to_string());
        }
    }
    for (alias, slug) in ALIASES {
        if *alias == term || alias.to_lowercase() == lower {
            return Some((*slug).to_string());
        }
    }
    None
}

fn print_workflow() {
    println!("## 🔄 워크플로우 색인");
    println!();
    println!("```");
    for (name, steps) in WORKFLOWS {
        println!("[{}]", name);
        println!("  {}", steps.join(" → "));
        println!();
    }
    println!("```");
}

fn print_full(entries: &[Entry], filter: Option<&str>) {
    let mut by_group: BTreeMap<&str, Vec<&Entry>> = BTreeMap::new();
    for e in entries {
        by_group.entry(e.group.as_str()).or_default().push(e);
    }
    for items in by_group.values_mut() {
        items.sort_by(|a, b| {
            (a.is_plugin, &a.name).cmp(&(b.is_plugin, &b.name))
        });
    }

    println!("# 스킬 카탈로그");
    println!();
    if let Some(f) = filter {
        let label = GROUPS
            .iter()
            .find(|(s, _)| *s == f)
            .map(|(_, l)| *l)
            .unwrap_or(f);
        println!("_필터: {} ({})_", label, f);
        println!();
    }

    for (slug, label) in GROUPS {
        if let Some(f) = filter {
            if f != *slug {
                continue;
            }
        }
        println!("## {} (`{}`)", label, slug);
        println!();
        if let Some(items) = by_group.get(*slug) {
            for e in items {
                let desc = truncate_chars(first_line(&e.description), 80);
                let prefix = if e.is_plugin { "🔌 " } else { "" };
                if desc.is_empty() {
                    println!("- **{}{}**", prefix, e.name);
                } else {
                    println!("- **{}{}** — {}", prefix, e.name, desc);
                }
            }
        } else {
            println!("- _(없음)_");
        }
        println!();
    }

    if filter.is_none() {
        if let Some(items) = by_group.get("unknown") {
            println!("## ❓ 미분류 (`unknown`)");
            println!();
            for e in items {
                println!("- **{}** _(group 필드 누락)_", e.name);
            }
            println!();
        }
        print_workflow();
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let workflow_only = args.iter().any(|a| a == "--workflow");
    let _markdown = args.iter().any(|a| a == "--markdown");
    let positional: Vec<&str> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .map(|s| s.as_str())
        .collect();

    if workflow_only {
        print_workflow();
        return ExitCode::SUCCESS;
    }

    let filter_slug: Option<String> = match positional.first() {
        Some(term) => match resolve_alias(term) {
            Some(s) => Some(s),
            None => {
                eprintln!("warn: 알 수 없는 그룹 '{}' — 전체 출력합니다.", term);
                None
            }
        },
        None => None,
    };

    let root = skills_root();
    let mut entries = load_skills(&root);
    let plugin_path = plugin_groups_path();
    let plugin_entries = parse_plugin_toml(&plugin_path);
    entries.extend(plugin_entries);

    print_full(&entries, filter_slug.as_deref());
    ExitCode::SUCCESS
}
