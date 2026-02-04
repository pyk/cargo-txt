//! cargo-txt: A cargo doc for coding agents
//!
//! This tool converts rustdoc HTML output into markdown documentation designed
//! for coding agents to browse and understand crate APIs.

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use commands::{build, list, show};
use tracing::level_filters::LevelFilter;

mod cargo;
mod commands;
mod html2md;

/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo txt")]
#[command(bin_name = "cargo txt")]
#[command(version)]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

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
        /// Item identifier (e.g., 'serde', 'serde::Error', 'serde::ser::StdError')
        #[arg(value_name = "ITEM")]
        item_identifier: String,
    },

    /// List all items in a library.
    List {
        /// Library name (e.g., 'serde')
        #[arg(value_name = "LIBRARY")]
        lib_name: String,
    },
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "txt" {
        args.remove(1);
    }

    let args = Args::parse_from(&args);

    if args.verbosity.tracing_level_filter() == LevelFilter::TRACE {
        tracing_subscriber::fmt()
            .with_max_level(args.verbosity)
            .init();
    } else {
        tracing_subscriber::fmt()
            .compact()
            .without_time()
            .with_target(false)
            .with_max_level(args.verbosity)
            .init();
    }

    match args.command {
        Command::Build { crate_name } => build(&crate_name)?,
        Command::Show { item_identifier } => show(&item_identifier)?,
        Command::List { lib_name } => list(&lib_name)?,
    }

    Ok(())
}
