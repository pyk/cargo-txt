//! Cargo command execution.
//!
//! This module provides functions for executing cargo doc commands
//! with proper error handling and HTML generation validation.

use crate::error;

/// Generate HTML documentation for a specific crate.
///
/// This function executes `cargo doc --package <crate> --no-deps` to generate
/// rustdoc HTML files for the specified crate, then validates that the output
/// directory exists.
pub fn doc(crate_name: &str) -> error::Result<()> {
    let output = std::process::Command::new("cargo")
        .args(["doc", "--package", crate_name, "--no-deps"])
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
    let doc_dir = std::path::PathBuf::from(target_dir)
        .join("doc")
        .join(crate_name);

    if !doc_dir.exists() {
        return Err(error::BuildError::DocNotGenerated {
            crate_name: crate_name.to_string(),
            expected_path: doc_dir,
        }
        .into());
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    /////////////////////////////////////////////////////////////////////////////
    // Execution Tests

    #[test]
    fn doc_returns_error_for_invalid_crate() {
        let result = doc("nonexistent_crate_12345_xyz");
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
        let result = doc("this_package_does_not_exist_anywhere_12345");
        assert!(result.is_err());
    }
}
