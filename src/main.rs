//! cargo-docmd: A cargo doc for coding agents
//!
//! This tool converts rustdoc HTML output into markdown documentation designed
//! for coding agents to browse and understand crate APIs.

mod cargo;
mod commands;
mod error;
mod items;

use clap::{Parser, Subcommand};
use commands::{browse, build};

/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo docmd")]
#[command(bin_name = "cargo docmd")]
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
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

    /// Browse crate documentation.
    Browse {
        /// Crate name to browse
        #[arg(value_name = "CRATE")]
        crate_name: String,

        /// Optional specific item to display
        #[arg(short, long, value_name = "ITEM")]
        item: Option<String>,
    },
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Build { crate_name } => build(crate_name)?,
        Command::Browse { crate_name, item } => browse(crate_name, item)?,
    }

    Ok(())
}
