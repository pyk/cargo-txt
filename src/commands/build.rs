//! Build command implementation.
//!
//! This module handles building documentation by executing cargo rustdoc,
//! parsing the generated JSON, and preparing the output directory for markdown generation.

use std::path::Path;

use crate::cargo::{check_nightly_installed, generate_rustdoc_json};
use crate::error::{BuildError, Error, Result};

/// Build markdown documentation from rustdoc JSON.
///
/// This function takes a crate name, generates rustdoc JSON using cargo +nightly,
/// parses it, and prepares the output directory for markdown generation.
pub fn build(crate_name: String) -> Result<()> {
    check_nightly_installed()?;

    let json_path = generate_rustdoc_json(&crate_name)?;

    let output_dir = get_output_dir()?;
    create_output_directory(&output_dir)?;

    let krate = parse_rustdoc_json(&json_path)?;

    log_item_summary(&krate);

    println!("Documentation built successfully for {}", crate_name);
    println!("Output directory: {}", output_dir.display());

    Ok(())
}

/// Get the output directory for documentation.
///
/// This function reads the `CARGO_TARGET_DIR` environment variable and
/// returns a path to the `docmd` subdirectory within it.
fn get_output_dir() -> Result<std::path::PathBuf> {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("target"));

    let output_dir = target_dir.join("docmd");
    create_output_directory(&output_dir)?;

    Ok(output_dir)
}

/// Create the output directory if it doesn't exist.
///
/// This function creates the directory and all parent directories if they don't exist.
fn create_output_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|error| BuildError::OutputDirCreationFailed {
            path: path.to_path_buf(),
            error: error.to_string(),
        })?;
    }
    Ok(())
}

/// Parse the rustdoc JSON file.
///
/// This function reads the JSON file from disk and deserializes it into a
/// rustdoc_types::Crate struct, providing detailed error messages if the file
/// is missing or cannot be parsed.
fn parse_rustdoc_json(json_path: &Path) -> Result<rustdoc_types::Crate> {
    let json_content = std::fs::read_to_string(json_path).map_err(|error| {
        let build_error = if error.kind() == std::io::ErrorKind::NotFound {
            BuildError::JsonNotFound(json_path.to_path_buf())
        } else {
            BuildError::JsonParseError {
                path: json_path.to_path_buf(),
                error: error.to_string(),
            }
        };
        Error::Build(build_error)
    })?;

    serde_json::from_str(&json_content).map_err(|error| {
        Error::Build(BuildError::JsonParseError {
            path: json_path.to_path_buf(),
            error: error.to_string(),
        })
    })
}

/// Log a summary of the items in the crate documentation.
///
/// This function counts items by type and prints the summary to stdout,
/// providing quick verification that the JSON was parsed successfully.
fn log_item_summary(krate: &rustdoc_types::Crate) {
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

    for item in krate.index.values() {
        let type_name = get_item_type_name(&item.inner);
        *counts.entry(type_name).or_insert(0) += 1;
    }

    println!("\nParsed {} items from documentation:", krate.index.len());

    let mut sorted_types: Vec<_> = counts.iter().collect();
    sorted_types.sort_by_key(|&(name, _)| *name);

    for (type_name, count) in sorted_types {
        println!("  {}: {}", type_name, count);
    }
    println!();
}

/// Get a human-readable name for an item type.
///
/// This function converts rustdoc_types::ItemEnum variants into their
/// human-readable string names for display purposes.
fn get_item_type_name(inner: &rustdoc_types::ItemEnum) -> &'static str {
    match inner {
        rustdoc_types::ItemEnum::Module(_) => "Module",
        rustdoc_types::ItemEnum::ExternCrate { .. } => "Extern Crate",
        rustdoc_types::ItemEnum::Use(_) => "Use Statement",
        rustdoc_types::ItemEnum::Union(_) => "Union",
        rustdoc_types::ItemEnum::Struct(_) => "Struct",
        rustdoc_types::ItemEnum::StructField(_) => "Struct Field",
        rustdoc_types::ItemEnum::Enum(_) => "Enum",
        rustdoc_types::ItemEnum::Variant(_) => "Variant",
        rustdoc_types::ItemEnum::Function(_) => "Function",
        rustdoc_types::ItemEnum::Trait(_) => "Trait",
        rustdoc_types::ItemEnum::TraitAlias(_) => "Trait Alias",
        rustdoc_types::ItemEnum::Impl(_) => "Impl Block",
        rustdoc_types::ItemEnum::TypeAlias(_) => "Type Alias",
        rustdoc_types::ItemEnum::Constant { .. } => "Constant",
        rustdoc_types::ItemEnum::Static(_) => "Static",
        rustdoc_types::ItemEnum::ExternType => "Extern Type",
        rustdoc_types::ItemEnum::Macro(_) => "Macro",
        rustdoc_types::ItemEnum::ProcMacro(_) => "Proc Macro",
        rustdoc_types::ItemEnum::Primitive(_) => "Primitive",
        rustdoc_types::ItemEnum::AssocConst { .. } => "Associated Constant",
        rustdoc_types::ItemEnum::AssocType { .. } => "Associated Type",
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Error Tests

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn parse_rustdoc_json_returns_error_for_missing_file() {
        let result = parse_rustdoc_json(Path::new("/nonexistent/path.json"));
        assert!(result.is_err());
    }
}
