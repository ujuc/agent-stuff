use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "register-skill",
    about = "Idempotently append a skill row to the Skills table in a CLAUDE.md."
)]
struct Args {
    /// Path to the target CLAUDE.md
    claude_md: PathBuf,

    /// Skill name (goes into the first column, wrapped in backticks)
    #[arg(long)]
    name: String,

    /// Trigger phrases (second column, kept verbatim)
    #[arg(long)]
    triggers: String,

    /// Model assignment (third column, e.g. "sonnet", "opus", "sonnet + advisor")
    #[arg(long)]
    model: String,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let outcome = register(&args.claude_md, &args.name, &args.triggers, &args.model)?;
    match outcome {
        Outcome::Added => {
            println!("✓ registered `{}` in {}", args.name, args.claude_md.display());
            Ok(ExitCode::SUCCESS)
        }
        Outcome::AlreadyPresent => {
            println!(
                "• `{}` already present in {} — no-op",
                args.name,
                args.claude_md.display()
            );
            Ok(ExitCode::SUCCESS)
        }
    }
}

enum Outcome {
    Added,
    AlreadyPresent,
}

fn register(claude_md: &Path, name: &str, triggers: &str, model: &str) -> Result<Outcome> {
    let content = fs::read_to_string(claude_md)
        .with_context(|| format!("failed to read {}", claude_md.display()))?;

    let lines: Vec<&str> = content.lines().collect();

    let skills_header = lines
        .iter()
        .position(|l| l.trim() == "## Skills")
        .ok_or_else(|| anyhow!("`## Skills` section not found in {}", claude_md.display()))?;

    let (table_start, table_end) = locate_table(&lines, skills_header)
        .ok_or_else(|| anyhow!("skills table not found after `## Skills` heading"))?;

    let marker = format!("`{name}`");
    for line in &lines[table_start..=table_end] {
        if line.contains(&marker) {
            return Ok(Outcome::AlreadyPresent);
        }
    }

    let new_row = format!("| `{name}` | {triggers} | {model} |");
    let mut out = String::with_capacity(content.len() + new_row.len() + 2);
    for (i, line) in lines.iter().enumerate() {
        out.push_str(line);
        out.push('\n');
        if i == table_end {
            out.push_str(&new_row);
            out.push('\n');
        }
    }
    if !content.ends_with('\n') && out.ends_with('\n') {
        out.pop();
    }

    fs::write(claude_md, out)
        .with_context(|| format!("failed to write {}", claude_md.display()))?;
    Ok(Outcome::Added)
}

fn locate_table(lines: &[&str], from: usize) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;
    for (i, line) in lines.iter().enumerate().skip(from + 1) {
        if line.starts_with("## ") {
            break;
        }
        if line.trim_start().starts_with('|') {
            if start.is_none() {
                start = Some(i);
            }
            end = Some(i);
        }
    }
    start.zip(end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn sample_claude_md() -> String {
        "# Head\n\n## Skills\n\n| Skill | Triggers | Model |\n| ----- | -------- | ----- |\n| `commit` | /commit | sonnet |\n\n## Next\n".to_string()
    }

    fn tmpfile(tag: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "register-skill-test-{}-{}.md",
            std::process::id(),
            tag
        ));
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(body.as_bytes()).unwrap();
        path
    }

    #[test]
    fn adds_new_row() {
        let path = tmpfile("add", &sample_claude_md());
        let outcome = register(&path, "foo", "/foo, foo해줘", "sonnet").unwrap();
        assert!(matches!(outcome, Outcome::Added));
        let after = fs::read_to_string(&path).unwrap();
        assert!(after.contains("`foo`"));
        assert!(after.contains("/foo, foo해줘"));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn idempotent_on_existing_name() {
        let path = tmpfile("idempotent", &sample_claude_md());
        let first = register(&path, "commit", "/commit", "sonnet").unwrap();
        assert!(matches!(first, Outcome::AlreadyPresent));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn fails_when_no_skills_section() {
        let path = tmpfile("nosection", "# No skills\n\ntext only\n");
        let result = register(&path, "foo", "/foo", "sonnet");
        assert!(result.is_err());
        fs::remove_file(&path).ok();
    }
}
