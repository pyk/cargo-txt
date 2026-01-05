//! cargo-docmd: A cargo doc for coding agents
//!
//! This tool converts rustdoc JSON output into markdown documentation designed
//! for coding agents to browse and understand crate APIs.

mod commands;

use clap::{Parser, Subcommand};
use commands::{browse, generate};

/// A cargo doc for coding agents
///
/// cargo-docmd generates markdown documentation from rustdoc JSON files,
/// optimized for consumption by coding agents. It provides both generation
/// and interactive browsing capabilities.
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
    /// Generate markdown documentation from rustdoc JSON
    ///
    /// Converts rustdoc JSON output into markdown files suitable for coding agents.
    /// The generated documentation is optimized for API browsing and understanding.
    Generate {
        /// Crate name to generate documentation for
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,

        /// Output directory for generated markdown
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<std::path::PathBuf>,
    },

    /// Browse crate documentation interactively
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

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Generate { crate_name, output } => {
            generate(
                crate_name,
                output.unwrap_or_else(|| std::path::PathBuf::from("docs")),
            );
        }
        Command::Browse { crate_name, item } => {
            browse(crate_name, item);
        }
    }
}
