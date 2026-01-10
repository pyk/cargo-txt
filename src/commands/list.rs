//! List command implementation.
//!
//! This module provides the list command which displays the master index
//! of all items in a crate (all.md) to stdout. The list command only accepts
//! crate names and rejects paths with `::` to provide clear separation from
//! the show command.

use anyhow::{Context, Result, bail};
use log::{debug, trace};
use std::fs;
use std::path::PathBuf;

use crate::cargo;
use crate::commands::build;

/// List and display all items in a crate.
///
/// This function validates the crate name (rejecting paths with `::`),
/// ensures documentation is built, resolves the all.md file path,
/// and prints its contents to stdout.
pub fn list(crate_name: &str) -> Result<()> {
    debug!("List command: crate_name={}", crate_name);

    validate_crate_name(crate_name)?;

    build::if_needed(crate_name)?;

    let path_name = read_crate_path_name(crate_name)?;
    debug!("Using crate path name: {}", path_name);

    let markdown_path = resolve_all_md_path(crate_name)?;
    debug!("Resolved markdown path: {:?}", markdown_path);

    let markdown_content = fs::read_to_string(&markdown_path)
        .with_context(|| format!("failed to read markdown file '{}'", markdown_path.display()))?;
    trace!("Read markdown file ({} bytes)", markdown_content.len());

    println!("{}", markdown_content);

    Ok(())
}

/// Validate that the input is a simple crate name (no `::` separators).
///
/// Ensures the input is just a crate name and not a full item path.
fn validate_crate_name(crate_name: &str) -> Result<()> {
    if crate_name.contains("::") {
        bail!(
            "the list command only accepts crate names. Use 'cargo txt show {}' to view specific items.",
            crate_name
        );
    }

    if crate_name.is_empty() {
        bail!("crate name cannot be empty");
    }

    trace!("Validated crate name: {}", crate_name);
    Ok(())
}

/// Resolve the path to all.md for a crate.
///
/// Returns the path to the master index file (all.md) for the specified crate.
fn resolve_all_md_path(crate_name: &str) -> Result<PathBuf> {
    let metadata = cargo::metadata()?;
    let all_md_path = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(crate_name)
        .join("all.md");

    debug!("Resolved all.md path: {:?}", all_md_path);
    Ok(all_md_path)
}

/// Read crate path name from name file.
///
/// Reads the crate directory name (source of truth) from docmd/<crate>/name,
/// which was saved during build.
fn read_crate_path_name(crate_name: &str) -> Result<String> {
    let metadata = cargo::metadata()?;

    let name_file = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(crate_name)
        .join("name");

    let path_name = fs::read_to_string(&name_file)
        .with_context(|| format!("failed to read crate name file '{}'", name_file.display()))?
        .trim()
        .to_string();
    debug!("Read crate path name from file: {}", path_name);
    Ok(path_name)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    // validate_crate_name tests

    #[test]
    fn validate_simple_crate_name() {
        let result = validate_crate_name("serde");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_crate_name_with_dashes() {
        let result = validate_crate_name("serde-json");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_crate_name_with_underscores() {
        let result = validate_crate_name("serde_json");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_rejects_path_with_separator() {
        let result = validate_crate_name("serde::Error");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
        assert!(error_msg.contains("cargo txt show serde::Error"));
    }

    #[test]
    fn validate_rejects_nested_path() {
        let result = validate_crate_name("serde::ser::StdError");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
    }

    #[test]
    fn validate_rejects_empty_string() {
        let result = validate_crate_name("");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("cannot be empty"));
    }

    #[test]
    fn validate_rejects_leading_separator() {
        let result = validate_crate_name("::serde");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
    }

    #[test]
    fn validate_rejects_trailing_separator() {
        let result = validate_crate_name("serde::");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
    }

    #[test]
    fn validate_rejects_only_separators() {
        let result = validate_crate_name("::");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
    }

    #[test]
    fn validate_rejects_multiple_separators() {
        let result = validate_crate_name("serde::::Error");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("only accepts crate names"));
    }
}
