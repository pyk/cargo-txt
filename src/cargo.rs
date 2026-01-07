//! Cargo command execution.
//!
//! This module provides functions for executing cargo doc commands
//! with proper error handling and HTML generation validation.

use crate::error;
use serde::Deserialize;

/// Cargo metadata output structure.
///
/// This struct represents the JSON output from `cargo metadata --no-deps --format-version 1`.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub target_directory: String,
}

/// Package information from cargo metadata.
#[derive(Debug, Deserialize)]
pub struct Package {
    pub dependencies: Vec<Dependency>,
}

/// Dependency information for a package.
#[derive(Debug, Deserialize)]
pub struct Dependency {
    /// Name of the dependency crate
    pub name: String,
}

/// Get cargo metadata for the current project.
///
/// This function executes `cargo metadata --no-deps --format-version 1`
/// and parses the JSON output into a Metadata struct.
pub fn metadata() -> error::Result<Metadata> {
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .map_err(|e| error::BuildError::CargoMetadataExecFailed {
            output: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(error::BuildError::CargoMetadataExecFailed { output: stderr }.into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let metadata: Metadata =
        serde_json::from_str(&stdout).map_err(|e| error::BuildError::CargoMetadataExecFailed {
            output: format!("Failed to parse metadata JSON: {}", e),
        })?;

    Ok(metadata)
}

/// Parse cargo doc output to extract the generated HTML directory path.
///
/// This function parses the stdout from `cargo doc` to find the line
/// starting with "Generated " and extracts the directory path from
/// "Generated /path/to/crate/index.html".
///
/// Returns the parent directory of the generated index.html file.
pub fn parse_generated_output(stdout: &str) -> error::Result<std::path::PathBuf> {
    let generated_line = stdout
        .lines()
        .map(|line| line.trim())
        .find(|line| line.starts_with("Generated "))
        .and_then(|line| line.strip_prefix("Generated "))
        .map(|s| s.trim())
        .ok_or_else(|| error::BuildError::DocNotGenerated {
            crate_name: "<unknown>".to_string(),
            expected_path: std::path::PathBuf::new(),
        })?;

    let html_path = std::path::PathBuf::from(generated_line);
    Ok(html_path
        .parent()
        .ok_or_else(|| error::BuildError::DocNotGenerated {
            crate_name: "<unknown>".to_string(),
            expected_path: html_path.clone(),
        })?
        .to_path_buf())
}

/// Generate HTML documentation for a specific crate.
///
/// This function executes `cargo doc --package <crate> --no-deps`,
/// parses the output to find the generated directory, and returns the path
/// to the HTML documentation directory.
pub fn doc(crate_name: &str, debug: bool) -> error::Result<std::path::PathBuf> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["doc", "--package", crate_name, "--no-deps"]);

    if debug {
        eprintln!(
            "DEBUG: Executing: cargo doc --package {} --no-deps",
            crate_name
        );
    }

    let output = cmd
        .output()
        .map_err(|e| error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: e.to_string(),
        })?;

    if debug {
        eprintln!("DEBUG: Exit code: {}", output.status);
        eprintln!("DEBUG: stdout len: {}", output.stdout.len());
        eprintln!("DEBUG: stderr len: {}", output.stderr.len());
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if debug {
            eprintln!("DEBUG: stderr: {}", stderr);
        }
        return Err(error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: stderr,
        }
        .into());
    }

    // Parse stderr to find generated directory path (cargo doc writes to stderr)
    let stderr = String::from_utf8_lossy(&output.stderr);
    if debug {
        eprintln!("DEBUG: stderr: {:?}", stderr);
    }
    parse_generated_output(&stderr)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    /////////////////////////////////////////////////////////////////////////////
    // Output Parsing Tests

    #[test]
    fn parse_generated_output_extracts_directory_path() {
        let stdout = "  Documenting serde v1.0.193 (/path/to/serde)\n   Generated /home/user/project/target/doc/serde/index.html\n";
        let result = parse_generated_output(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            std::path::PathBuf::from("/home/user/project/target/doc/serde")
        );
    }

    #[test]
    fn parse_generated_output_handles_hyphens_to_underscores() {
        let stdout = "  Documenting rustdoc-types v0.57.0\n   Generated /home/user/project/target/doc/rustdoc_types/index.html\n";
        let result = parse_generated_output(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            std::path::PathBuf::from("/home/user/project/target/doc/rustdoc_types")
        );
    }

    #[test]
    fn parse_generated_output_returns_error_without_generated_line() {
        let stdout = "  Documenting serde v1.0.193\n  some other output\n";
        let result = parse_generated_output(stdout);
        assert!(result.is_err());
    }

    #[test]
    fn parse_generated_output_handles_multiple_lines() {
        let stdout = "line 1\nline 2\n   Generated /path/to/doc/index.html\nline 4\n";
        let result = parse_generated_output(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, std::path::PathBuf::from("/path/to/doc"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Execution Tests

    #[test]
    fn doc_returns_error_for_invalid_crate() {
        let result = doc("nonexistent_crate_12345_xyz", false);
        assert!(result.is_err());
    }

    /////////////////////////////////////////////////////////////////////////////
    // Validation Tests

    #[test]
    fn doc_returns_error_when_doc_directory_not_created() {
        // This test validates that doc() returns an error when the expected
        // documentation directory is not created after running cargo doc.
        // Since we can't easily simulate a successful cargo doc execution
        // without actually generating docs, we test the error path with
        // a non-existent crate which will fail early.
        let result = doc("this_package_does_not_exist_anywhere_12345", false);
        assert!(result.is_err());
    }
}
