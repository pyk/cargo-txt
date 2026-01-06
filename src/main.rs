//! cargo-docmd: A cargo doc for coding agents
//!
//! This tool converts rustdoc JSON output into markdown documentation designed
//! for coding agents to browse and understand crate APIs.

mod cargo;
mod commands;
mod error;
mod markdown;

use clap::{Parser, Subcommand};
use commands::{browse, build};
use error::Result;

/// A cargo doc for coding agents
///
/// cargo-docmd generates markdown documentation from rustdoc JSON files,
/// optimized for consumption by coding agents. It provides both build
/// and browse capabilities.
#[derive(Parser)]
#[command(name = "cargo-docmd")]
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    /// Increase verbosity of output
    ///
    /// Use multiple times for more verbose output (e.g., -vv, -vvv).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Path to configuration file
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build markdown documentation from rustdoc JSON
    ///
    /// Generates rustdoc JSON using cargo +nightly and parses it to create
    /// markdown files suitable for coding agents. Output is placed in
    /// `$CARGO_TARGET_DIR/docmd`.
    Build {
        /// Crate name to build documentation for
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,
    },

    /// Browse crate documentation
    ///
    /// Displays crate documentation in a terminal-friendly format. Optionally,
    /// you can specify a specific item to display only that documentation.
    Browse {
        /// Crate name to browse
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,

        /// Optional specific item to display
        #[arg(short, long, value_name = "ITEM")]
        item: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Build { crate_name } => {
            build(crate_name)?;
        }
        Command::Browse { crate_name, item } => {
            browse(crate_name, item);
        }
    }

    Ok(())
}
