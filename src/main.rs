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
use commands::{build, list, show};

/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo txt")]
#[command(bin_name = "cargo txt")]
#[command(version = env!("CARGO_PKG_VERSION"))]
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

    let verbosity_level = args.verbosity.log_level_filter().to_string();
    let env = env_logger::Env::default().default_filter_or(verbosity_level);
    env_logger::Builder::from_env(env).init();

    match args.command {
        Command::Build { crate_name } => build(&crate_name)?,
        Command::Show { item_path } => show(&item_path)?,
        Command::List { lib_name } => list(&lib_name)?,
    }

    Ok(())
}
