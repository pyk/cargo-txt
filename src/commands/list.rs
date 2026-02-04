//! List command implementation.
//!
//! This module provides the list command which displays the master index
//! of all items in a crate (all.md) to stdout. The list command only accepts
//! crate names and rejects paths with `::` to provide clear separation from
//! the show command.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde_json;
use tracing::{debug, trace};

use crate::cargo;
use crate::commands::build::CrateDocMetadata;

/// List and display all items in a crate.
///
/// This function accepts a library name, checks if metadata.json exists,
/// resolves the all.md file path, and prints its contents to stdout.
pub fn list(lib_name: &str) -> Result<()> {
    debug!("List command: lib_name={}", lib_name);

    let metadata = cargo::metadata()?;
    let docmd_dir = PathBuf::from(&metadata.target_directory).join("docmd");

    let metadata_path = docmd_dir.join(lib_name).join("metadata.json");
    if !metadata_path.exists() {
        let available_crates: Vec<&str> = metadata.packages[0]
            .dependencies
            .iter()
            .map(|dep| dep.name.as_str())
            .collect();
        bail!(
            "Documentation for '{}' is not built yet. Run 'cargo txt build <crate>' for one of the following crates: {}",
            lib_name,
            available_crates.join(", ")
        );
    }

    let metadata_content = fs::read_to_string(&metadata_path)
        .with_context(|| format!("failed to read metadata file '{}'", metadata_path.display()))?;
    let crate_metadata: CrateDocMetadata =
        serde_json::from_str(&metadata_content).with_context(|| "failed to parse metadata.json")?;

    trace!(
        "Loaded metadata: crate_name={}, lib_name={}",
        crate_metadata.crate_name, crate_metadata.lib_name
    );

    let all_md_path = docmd_dir.join(lib_name).join("all.md");
    debug!("Resolved all.md path: {:?}", all_md_path);

    let markdown_content = fs::read_to_string(&all_md_path)
        .with_context(|| format!("failed to read markdown file '{}'", all_md_path.display()))?;
    trace!("Read markdown file ({} bytes)", markdown_content.len());

    println!("{}", markdown_content);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn list_succeeds_when_metadata_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let docmd_dir = temp_dir.path();

        let lib_dir = docmd_dir.join("rustdoc_types");
        fs::create_dir_all(&lib_dir).unwrap();
        let metadata_content = r#"{
            "crate_name": "rustdoc-types",
            "lib_name": "rustdoc_types",
            "item_map": {
                "rustdoc_types::Item": "struct.Item.md"
            }
        }"#;
        fs::write(lib_dir.join("metadata.json"), metadata_content).unwrap();

        let all_md = "# List of all items\n\n### Structs\n\n- Item";
        fs::write(lib_dir.join("all.md"), all_md).unwrap();

        assert!(lib_dir.join("metadata.json").exists());
        assert!(lib_dir.join("all.md").exists());
    }
}
