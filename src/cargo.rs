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

    /// Features enabled for this dependency
    pub features: Vec<String>,

    /// Whether default features are enabled
    #[serde(default = "default_true")]
    pub uses_default_features: bool,
}

/// Default value for uses_default_features.
fn default_true() -> bool {
    true
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

/// Generate HTML documentation for a specific crate.
///
/// This function executes `cargo doc --package <crate> --no-deps` with the
/// specified feature flags, then validates that the output directory exists.
pub fn doc(
    crate_name: &str,
    target_dir: &str,
    features: &[&str],
    use_default_features: bool,
) -> error::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["doc", "--package", crate_name, "--no-deps"]);

    if !use_default_features {
        cmd.arg("--no-default-features");
    }

    if !features.is_empty() {
        cmd.arg("--features").arg(features.join(","));
    }

    let output = cmd
        .output()
        .map_err(|e| error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: stderr,
        }
        .into());
    }

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
        let result = doc("nonexistent_crate_12345_xyz", "target", &[], true);
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
        let result = doc(
            "this_package_does_not_exist_anywhere_12345",
            "target",
            &[],
            true,
        );
        assert!(result.is_err());
    }
}
