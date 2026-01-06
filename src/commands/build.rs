//! Build command implementation.
//!
//! This module handles building documentation by executing cargo rustdoc,
//! parsing the generated JSON, and preparing the output directory for markdown generation.

use rustdoc_types::{Crate, ItemEnum, Visibility};
use std::path::Path;

use crate::cargo;
use crate::error;
use crate::markdown;

/// Build markdown documentation from rustdoc JSON.
///
/// This function takes a crate name, generates rustdoc JSON using cargo +nightly,
/// parses it, and prepares the output directory for markdown generation.
pub fn build(crate_name: String) -> error::Result<()> {
    cargo::nightly()?;

    let json_path = cargo::rustdoc(&crate_name)?;

    let output_dir = get_output_dir()?;
    create_output_directory(&output_dir)?;

    let krate = parse_rustdoc_json(&json_path)?;

    log_item_summary(&krate);

    // Generate markdown files for all items
    generate_all_items(&krate, &crate_name, &output_dir)?;

    // Generate index page
    markdown::index::generate_index(&krate, &output_dir)?;

    println!("Documentation built successfully for {}", crate_name);
    println!("Output directory: {}", output_dir.display());

    Ok(())
}

/// Get the output directory for documentation.
///
/// This function reads the `CARGO_TARGET_DIR` environment variable and
/// returns a path to the `docmd` subdirectory within it.
fn get_output_dir() -> error::Result<std::path::PathBuf> {
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
fn create_output_directory(path: &Path) -> error::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|error| {
            error::BuildError::OutputDirCreationFailed {
                path: path.to_path_buf(),
                error: error.to_string(),
            }
        })?;
    }
    Ok(())
}

/// Parse the rustdoc JSON file.
///
/// This function reads the JSON file from disk and deserializes it into a
/// rustdoc_types::Crate struct, providing detailed error messages if the file
/// is missing or cannot be parsed.
fn parse_rustdoc_json(json_path: &Path) -> error::Result<Crate> {
    let json_content = std::fs::read_to_string(json_path).map_err(|error| {
        let build_error = match error.kind() {
            std::io::ErrorKind::NotFound => {
                error::BuildError::JsonNotFound(json_path.to_path_buf())
            }
            _ => error::BuildError::JsonParseError {
                path: json_path.to_path_buf(),
                error: error.to_string(),
            },
        };
        error::Error::Build(build_error)
    })?;

    serde_json::from_str(&json_content).map_err(|error| {
        error::Error::Build(error::BuildError::JsonParseError {
            path: json_path.to_path_buf(),
            error: error.to_string(),
        })
    })
}

/// Log a summary of the items in the crate documentation.
///
/// This function counts items by type and prints the summary to stdout,
/// providing quick verification that the JSON was parsed successfully.
fn log_item_summary(krate: &Crate) {
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

    for item in krate.index.values() {
        let type_name = markdown::utils::get_item_type_name(&item.inner);
        // TODO: avoid this pattern?
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

/// Generate markdown files for all public items in the crate.
///
/// This function iterates through all items in the crate index and generates
/// markdown documentation for supported item types (structs, enums, unions,
/// and type aliases).
fn generate_all_items(krate: &Crate, crate_name: &str, output_dir: &Path) -> error::Result<()> {
    let mut counts = std::collections::HashMap::new();

    for item in krate.index.values() {
        // Skip private items and impl blocks
        if !is_public_item(item) {
            continue;
        }

        match &item.inner {
            ItemEnum::Struct(_) => {
                markdown::r#struct::generate(krate, item, output_dir)?;
                // TODO: avoid this pattern?
                *counts.entry("struct").or_insert(0) += 1;
            }
            ItemEnum::Enum(_) => {
                markdown::r#enum::generate(krate, item, output_dir)?;
                *counts.entry("enum").or_insert(0) += 1;
            }
            ItemEnum::Union(_) => {
                markdown::union::generate(krate, item, output_dir)?;
                *counts.entry("union").or_insert(0) += 1;
            }
            ItemEnum::TypeAlias(alias_data) => {
                markdown::type_alias::generate(
                    item,
                    alias_data,
                    &krate.index,
                    Some(crate_name),
                    output_dir,
                )?;
                *counts.entry("type alias").or_insert(0) += 1;
            }
            _ => {
                // Skip other item types for now
            }
        }
    }

    // Log generation summary
    if !counts.is_empty() {
        println!("\nGenerated markdown files:");
        let mut sorted_counts: Vec<_> = counts.iter().collect();
        sorted_counts.sort_by_key(|&(name, _)| *name);
        for (type_name, count) in sorted_counts {
            println!("  {} {}", type_name, count);
        }
        println!();
    }

    Ok(())
}

/// Check if an item is public and should be documented.
///
/// This function determines if an item should be included in the documentation
/// by checking its visibility. Private items are excluded.
fn is_public_item(item: &rustdoc_types::Item) -> bool {
    matches!(item.visibility, Visibility::Public | Visibility::Crate)
}
