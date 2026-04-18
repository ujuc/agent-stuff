use crate::log;
use clap::Args;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

#[derive(Args, Debug)]
pub struct DepsArgs {
    /// Check/install LM Studio requirements only.
    #[arg(long)]
    pub lmstudio: bool,
    /// Check/install Gemini requirements only.
    #[arg(long)]
    pub gemini: bool,
    /// Check/install everything (default when no flag given).
    #[arg(long)]
    pub all: bool,
}

pub fn run(args: DepsArgs) -> u8 {
    let (check_lmstudio, check_gemini) = if !args.lmstudio && !args.gemini && !args.all {
        (true, true)
    } else if args.all {
        (true, true)
    } else {
        (args.lmstudio, args.gemini)
    };

    if which("brew").is_err() {
        log::err("Homebrew (brew) not found.");
        log::err("install from https://brew.sh then re-run.");
        return 2;
    }

    if let Err(rc) = ensure_formula("curl", "curl") {
        return rc;
    }
    if let Err(rc) = ensure_formula("jq", "jq") {
        return rc;
    }

    if check_lmstudio {
        if let Err(rc) = ensure_cask("lms", "lm-studio", "$HOME/.lmstudio/bin") {
            return rc;
        }
    }
    if check_gemini {
        if let Err(rc) = ensure_cask("op", "1password-cli", "") {
            return rc;
        }
    }

    log::info("all required dependencies present.");
    0
}

fn ensure_formula(bin: &str, formula: &str) -> Result<(), u8> {
    if which(bin).is_ok() {
        return Ok(());
    }
    if confirm_install(formula, formula) {
        log::info(&format!("installing {formula}..."));
        let status = Command::new("brew").arg("install").arg(formula).status();
        match status {
            Ok(s) if s.success() => Ok(()),
            _ => {
                log::err(&format!("'brew install {formula}' failed"));
                Err(2)
            }
        }
    } else {
        log::err(&format!(
            "'{bin}' missing; install manually: brew install {formula}"
        ));
        Err(2)
    }
}

fn ensure_cask(bin: &str, cask: &str, path_hint: &str) -> Result<(), u8> {
    if which(bin).is_ok() {
        return Ok(());
    }
    let display = format!("--cask {cask}");
    if confirm_install(&display, &format!("{cask} (cask)")) {
        log::info(&format!("installing {cask}..."));
        let status = Command::new("brew")
            .arg("install")
            .arg("--cask")
            .arg(cask)
            .status();
        match status {
            Ok(s) if s.success() => {}
            _ => {
                log::err(&format!("'brew install --cask {cask}' failed"));
                return Err(2);
            }
        }
    } else {
        log::err(&format!(
            "'{bin}' missing; install manually: brew install --cask {cask}"
        ));
        return Err(2);
    }

    if which(bin).is_err() && !path_hint.is_empty() {
        log::warn(&format!("{bin} still not on PATH; add this to your shell rc:"));
        log::warn(&format!("  export PATH=\"{path_hint}:$PATH\""));
    }
    Ok(())
}

fn confirm_install(_pkg: &str, display: &str) -> bool {
    if env::var("GEMMA_AUTO_INSTALL").ok().as_deref() == Some("1") {
        return true;
    }
    // Prompt on /dev/tty so we survive subprocess stdio redirection.
    let prompt = format!("install {display} via brew? [y/N] ");
    let tty_path = Path::new("/dev/tty");
    let writer = OpenOptions::new().write(true).open(tty_path);
    let reader = OpenOptions::new().read(true).open(tty_path);

    match (writer, reader) {
        (Ok(mut w), Ok(r)) => {
            let _ = w.write_all(prompt.as_bytes());
            let _ = w.flush();
            let mut line = String::new();
            if BufReader::new(r).read_line(&mut line).is_err() {
                return false;
            }
            matches!(line.trim(), "y" | "Y")
        }
        _ => {
            // No TTY — fall back to stderr prompt + stdin read.
            let _ = write!(io::stderr(), "{prompt}");
            let _ = io::stderr().flush();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                return false;
            }
            matches!(line.trim(), "y" | "Y")
        }
    }
}

fn which(cmd: &str) -> Result<(), ()> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            if Path::new(dir).join(cmd).exists() {
                return Ok(());
            }
        }
    }
    Err(())
}
