mod deps;
mod gemini;
mod keychain;
mod lmstudio;
mod log;
mod models;
mod query;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "gemma", about = "Dispatch prompts to LM Studio or Google AI Studio Gemini API")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Route a prompt to local LM Studio or Gemini API.
    Query(query::QueryArgs),
    /// List Gemini API model IDs (cached).
    ListModels,
    /// Read the Google AI Studio API key from 1Password.
    FetchApiKey,
    /// Verify and optionally install dependencies via brew.
    EnsureDeps(deps::DepsArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let rc = match cli.command {
        Command::Query(args) => query::run(args),
        Command::ListModels => models::run_list_cli(),
        Command::FetchApiKey => keychain::run_fetch_cli(),
        Command::EnsureDeps(args) => deps::run(args),
    };
    ExitCode::from(rc)
}
