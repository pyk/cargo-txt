//! Generate command implementation.
//!
//! This module handles the generation of markdown documentation from rustdoc JSON files.

use std::path::PathBuf;

/// Generate markdown documentation from rustdoc JSON.
///
/// This function takes a crate name and output directory, then generates
/// markdown documentation suitable for coding agents. Currently a placeholder
/// that prints the received parameters.
pub fn generate(crate_name: String, output: PathBuf) {
    println!(
        "Generate command: crate={}, output={:?}",
        crate_name, output
    );
    println!("Not yet implemented");
}
