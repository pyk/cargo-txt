//! Cargo command execution.
//!
//! This module provides functions for executing cargo doc commands
//! with proper error handling and HTML generation validation.

use anyhow::{Context, Result, bail};
use log::{debug, trace};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

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
pub fn metadata() -> Result<Metadata> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to execute cargo metadata command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        bail!("failed to execute cargo metadata command:\n{}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let metadata: Metadata =
        serde_json::from_str(&stdout).context("failed to parse cargo metadata JSON")?;

    Ok(metadata)
}

/// Generate HTML documentation for a specific crate.
///
/// This function executes `cargo doc --package <crate> --no-deps`,
/// parses the output to find the generated directory, and returns the path
/// to the HTML documentation directory.
pub fn doc(crate_name: &str) -> Result<PathBuf> {
    let mut cmd = Command::new("cargo");
    cmd.args(["doc", "--package", crate_name, "--no-deps"]);

    debug!("Executing: cargo doc --package {} --no-deps", crate_name);

    let output = cmd.output().context(format!(
        "failed to execute cargo doc for crate '{}'",
        crate_name
    ))?;

    trace!("Exit code: {}", output.status);
    trace!("stdout len: {}", output.stdout.len());
    trace!("stderr len: {}", output.stderr.len());

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        debug!("stderr: {}", stderr);
        bail!(
            "failed to execute cargo doc for crate '{}':\n{}",
            crate_name,
            stderr
        );
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    debug!("stderr: {:?}", stderr);
    doc_output_dir(&stderr)
}

/// Parse cargo doc output to extract the generated HTML directory path.
///
/// This function parses the stdout from `cargo doc` to find the line
/// starting with "Generated " and extracts the directory path from
/// "Generated /path/to/crate/index.html".
///
/// Returns the parent directory of the generated index.html file.
fn doc_output_dir(stdout: &str) -> Result<PathBuf> {
    let generated_line = match stdout
        .lines()
        .map(|line| line.trim())
        .find(|line| line.starts_with("Generated "))
        .and_then(|line| line.strip_prefix("Generated "))
        .map(|s| s.trim())
    {
        Some(line) => line,
        None => {
            let output_preview = if stdout.len() > 500 {
                format!("{}...", &stdout[..500])
            } else {
                stdout.to_string()
            };
            bail!(
                "failed to parse cargo doc output - could not find 'Generated' line. Output preview:\n{}",
                output_preview
            )
        }
    };

    let html_path = PathBuf::from(generated_line);
    match html_path.parent() {
        Some(parent) => Ok(parent.to_path_buf()),
        None => {
            let line_preview = if generated_line.len() > 200 {
                format!("{}...", &generated_line[..200])
            } else {
                generated_line.to_string()
            };
            bail!(
                "failed to parse cargo doc output - Generated line found but has no parent directory: {}",
                line_preview
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_output_dir_extracts_directory_path() {
        let stdout = "  Documenting serde v1.0.193 (/path/to/serde)\n   Generated /home/user/project/target/doc/serde/index.html\n";
        let result = doc_output_dir(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            std::path::PathBuf::from("/home/user/project/target/doc/serde")
        );
    }

    #[test]
    fn doc_output_dir_handles_hyphens_to_underscores() {
        let stdout = "  Documenting rustdoc-types v0.57.0\n   Generated /home/user/project/target/doc/rustdoc_types/index.html\n";
        let result = doc_output_dir(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            std::path::PathBuf::from("/home/user/project/target/doc/rustdoc_types")
        );
    }

    #[test]
    fn doc_output_dir_returns_error_without_generated_line() {
        let stdout = "  Documenting serde v1.0.193\n  some other output\n";
        let result = doc_output_dir(stdout);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("failed to parse cargo doc output"));
        assert!(error_msg.contains("could not find 'Generated' line"));
    }

    #[test]
    fn doc_output_dir_handles_multiple_lines() {
        let stdout = "line 1\nline 2\n   Generated /path/to/doc/index.html\nline 4\n";
        let result = doc_output_dir(stdout);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, std::path::PathBuf::from("/path/to/doc"));
    }
}
