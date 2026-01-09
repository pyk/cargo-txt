//! cargo-txt: A cargo doc for coding agents
//!
//! This tool converts rustdoc HTML output into markdown documentation designed
//! for coding agents to browse and understand crate APIs.

mod cargo;
mod commands;
mod html2md;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use commands::{build, show};

/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo txt")]
#[command(bin_name = "cargo txt")]
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate markdown documentation from rustdoc HTML for coding agents.
    Build {
        /// Crate name to build documentation for
        #[arg(value_name = "CRATE")]
        crate_name: String,
    },

    /// Show and display crate documentation.
    Show {
        /// Item path (e.g., 'serde', 'serde::Error', 'serde::ser::StdError')
        #[arg(value_name = "ITEM")]
        item_path: String,
    },
}

fn main() -> Result<()> {
    // 1. Collect arguments
    let mut args: Vec<String> = std::env::args().collect();

    // 2. If called via `cargo txt`, Cargo appends subcommand name ("txt") as first arg.
    // We need to remove it so our actual subcommands (build, open) are recognized.
    if args.len() > 1 && args[1] == "txt" {
        args.remove(1);
    }

    // 3. Parse modified arguments using parse_from
    let args = Args::parse_from(&args);

    let verbosity_level = args.verbosity.log_level_filter().to_string();
    let env = env_logger::Env::default().default_filter_or(verbosity_level);
    env_logger::Builder::from_env(env).init();

    match args.command {
        Command::Build { crate_name } => build(&crate_name)?,
        Command::Show { item_path } => show(&item_path)?,
    }

    Ok(())
}
