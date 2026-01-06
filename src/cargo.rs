//! Cargo command execution.
//!
//! This module provides functions for executing cargo rustdoc commands
//! with proper error handling and nightly toolchain verification.

use std::path::Path;

use crate::error;

/// Check if the nightly toolchain is available.
///
/// Runs `cargo +nightly --version` to verify nightly is installed.
pub fn nightly() -> error::Result<()> {
    let output = std::process::Command::new("cargo")
        .args(["+nightly", "--version"])
        .output();

    let Ok(result) = output else {
        return Err(error::BuildError::NightlyNotInstalled.into());
    };

    if result.status.success() {
        Ok(())
    } else {
        Err(error::BuildError::NightlyNotInstalled.into())
    }
}

/// Generate rustdoc JSON for a specific crate and return the file path.
///
/// This function executes `cargo +nightly rustdoc -p <crate> -- --output-format json -Z unstable-options`
/// to generate rustdoc JSON file for the specified crate, then returns the path to the JSON file.
/// The function searches for the JSON file using multiple strategies to handle different
/// naming conventions used by cargo and rustdoc.
pub fn rustdoc(crate_name: &str) -> error::Result<std::path::PathBuf> {
    let output = std::process::Command::new("cargo")
        .args([
            "+nightly",
            "rustdoc",
            "-p",
            crate_name,
            "--",
            "--output-format",
            "json",
            "-Z",
            "unstable-options",
        ])
        .output()
        .map_err(|e| error::BuildError::CargoExecutionFailed {
            crate_name: crate_name.to_string(),
            output: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(error::BuildError::CargoExecutionFailed {
            crate_name: crate_name.to_string(),
            output: stderr,
        }
        .into());
    }

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let doc_dir = std::path::PathBuf::from(target_dir).join("doc");

    find_json_file(&doc_dir, crate_name)
}

/// Find the rustdoc JSON file for a given crate name.
///
/// This function searches for the JSON file in the doc directory using multiple strategies:
/// - Exact match with the crate name
/// - Match with hyphens and underscores swapped
/// - Partial match (e.g., "docmd" matches "cargo_docmd.json")
///
/// Returns the path to the JSON file if found, or an error if no matching file exists.
fn find_json_file(doc_dir: &Path, crate_name: &str) -> error::Result<std::path::PathBuf> {
    // Generate candidate filenames
    let candidates = generate_json_filename_candidates(crate_name);

    // Try each candidate in order
    for candidate in &candidates {
        let path = doc_dir.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }

    // If no exact match, search for partial matches in the directory
    if let Some(matching_file) = find_partial_json_match(doc_dir, crate_name) {
        return Ok(matching_file);
    }

    // No match found - return error with the first candidate path
    let first_candidate = doc_dir.join(&candidates[0]);
    Err(error::BuildError::JsonNotFound(first_candidate).into())
}

/// Generate candidate JSON filenames for a crate name.
///
/// This function returns a list of possible JSON filenames in order of preference,
/// accounting for cargo's naming variations with hyphens and underscores.
fn generate_json_filename_candidates(crate_name: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    // Try exact match first
    candidates.push(format!("{}.json", crate_name));

    // Try with hyphens replaced by underscores
    if crate_name.contains('-') {
        candidates.push(format!("{}.json", crate_name.replace('-', "_")));
    }

    // Try with underscores replaced by hyphens
    if crate_name.contains('_') {
        candidates.push(format!("{}.json", crate_name.replace('_', "-")));
    }

    candidates
}

/// Find a JSON file that partially matches the crate name.
///
/// This function searches the doc directory for JSON files that contain the crate
/// name as a substring, allowing for matching when the binary name differs from
/// the package name (e.g., "docmd" matches "cargo_docmd.json").
fn find_partial_json_match(doc_dir: &Path, crate_name: &str) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(doc_dir).ok()?;

    let normalized_name = crate_name.replace(['-', '_'], "");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let file_name = entry.file_name();
        let file_stem = file_name.to_string_lossy();
        let normalized_file = file_stem.replace(['-', '_'], "");

        if normalized_file.contains(&normalized_name) {
            return Some(path);
        }
    }

    None
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    /////////////////////////////////////////////////////////////////////////////
    // Filename Generation Tests
    //
    // These tests verify filename matching logic that handles cargo's
    // naming variations (hyphens vs underscores). Unlike tests that just verify
    // language guarantees, these verify actual business logic for finding JSON files.

    #[test]
    fn json_filename_candidates_exact_match() {
        let candidates = generate_json_filename_candidates("serde");
        assert_eq!(candidates, vec!["serde.json"]);
    }

    #[test]
    fn json_filename_candidates_with_hyphens() {
        let candidates = generate_json_filename_candidates("cargo-docmd");
        assert_eq!(candidates, vec!["cargo-docmd.json", "cargo_docmd.json"]);
    }

    #[test]
    fn json_filename_candidates_with_underscores() {
        let candidates = generate_json_filename_candidates("cargo_docmd");
        assert_eq!(candidates, vec!["cargo_docmd.json", "cargo-docmd.json"]);
    }

    #[test]
    fn json_filename_candidates_mixed() {
        let candidates = generate_json_filename_candidates("some-crate_name");
        assert_eq!(
            candidates,
            vec![
                "some-crate_name.json",
                "some_crate_name.json",
                "some-crate-name.json"
            ]
        );
    }

    #[test]
    fn rustdoc_returns_error_for_invalid_crate() {
        let result = rustdoc("nonexistent_crate_12345_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn find_partial_json_match_returns_none_for_nonexistent_directory() {
        let result = find_partial_json_match(Path::new("/nonexistent/path"), "test");
        assert!(result.is_none());
    }
}
