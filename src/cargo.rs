//! Cargo command execution.
//!
//! This module provides functions for executing cargo rustdoc commands
//! with proper error handling and nightly toolchain verification.

use std::process::Command;

use crate::error::{BuildError, Result};

/// Check if the nightly toolchain is available.
///
/// Runs `cargo +nightly --version` to verify nightly is installed.
pub fn check_nightly_installed() -> Result<()> {
    let output = Command::new("cargo")
        .args(["+nightly", "--version"])
        .output()
        .map_err(|e| BuildError::CargoExecutionFailed {
            crate_name: "rustup check".to_string(),
            output: e.to_string(),
        })?;

    if !output.status.success() {
        return Err(BuildError::NightlyNotInstalled.into());
    }

    Ok(())
}

/// Generate rustdoc JSON for a specific crate and return the expected file path.
///
/// This function executes `cargo +nightly rustdoc -p <crate> -- --output-format json -Z unstable-options`
/// to generate the rustdoc JSON file for the specified crate, then returns the path
/// to the expected JSON file location.
pub fn generate_rustdoc_json(crate_name: &str) -> Result<std::path::PathBuf> {
    let output = Command::new("cargo")
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
        .map_err(|e| BuildError::CargoExecutionFailed {
            crate_name: crate_name.to_string(),
            output: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(BuildError::CargoExecutionFailed {
            crate_name: crate_name.to_string(),
            output: stderr,
        }
        .into());
    }

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    Ok(std::path::PathBuf::from(target_dir)
        .join("doc")
        .join(format!("{}.json", crate_name)))
}

///////////////////////////////////////////////////////////////////////////////
/// Execution Tests

#[cfg(test)]
mod execution_tests {
    use super::*;

    #[test]
    fn generate_rustdoc_json_returns_error_for_invalid_crate() {
        let result = generate_rustdoc_json("nonexistent_crate_12345_xyz");
        assert!(result.is_err());
    }
}
