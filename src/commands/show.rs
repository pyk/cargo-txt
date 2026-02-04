//! Show command implementation.
//!
//! This module provides the show command which displays crate documentation
//! to stdout. Users can view the crate overview (index.md) or specific items
//! by providing an item identifier.

use std::fs;
use std::path::PathBuf;

use crate::commands::build::CrateDocMetadata;
use anyhow::{Context, Result, bail, ensure};
use serde_json;
use tracing::{debug, trace};

use crate::cargo;

/// Parsed item identifier containing library name and optional item.
#[derive(Debug)]
struct ItemIdentifier {
    lib_name: String,
    item: Option<String>,
}

/// Show and display crate documentation.
///
/// This function parses the item identifier, resolves the appropriate markdown file,
/// and prints its contents to stdout.
pub fn show(item_identifier: &str) -> Result<()> {
    debug!("Show command: item_identifier={}", item_identifier);

    let parsed = parse_item_identifier(item_identifier)?;
    trace!(
        "Parsed: lib_name={}, item={:?}",
        parsed.lib_name, parsed.item
    );

    let markdown_path = resolve_markdown_path(&parsed).with_context(|| {
        let metadata = cargo::metadata();
        match metadata {
            Ok(m) => {
                let available_crates: Vec<&str> = m.packages[0]
                    .dependencies
                    .iter()
                    .map(|dep| dep.name.as_str())
                    .collect();
                format!(
                    "Can't show '{}'. You should build one of the following crates first: {}",
                    item_identifier,
                    available_crates.join(", ")
                )
            }
            Err(_) => format!(
                "Can't show '{}'. You may need to build the crate documentation first",
                item_identifier
            ),
        }
    })?;
    debug!("Resolved markdown path: {:?}", markdown_path);

    let markdown_content = fs::read_to_string(&markdown_path)
        .with_context(|| format!("failed to read markdown file '{}'", markdown_path.display()))?;
    trace!("Read markdown file ({} bytes)", markdown_content.len());

    println!("{}", markdown_content);

    Ok(())
}

/// Parse an item identifier into library name and optional item.
///
/// Extracts the library name (first component before `::`) and the remaining
/// item identifier (if any) from the full item identifier string.
fn parse_item_identifier(item_identifier: &str) -> Result<ItemIdentifier> {
    let mut parts = item_identifier.split("::");

    let lib_name = match parts.next().filter(|s| !s.is_empty()) {
        Some(n) => n,
        None => bail!(
            "invalid item identifier '{}'. Expected format: <lib_name> or <lib_name>::<item> (e.g., 'serde' or 'serde::Error').",
            item_identifier
        ),
    };

    ensure!(!lib_name.is_empty(), "library name cannot be empty");

    let item: Vec<&str> = parts.collect();
    let item = if item.is_empty() {
        None
    } else {
        Some(item.join("::"))
    };

    trace!("Parsed item path: lib_name={}, item={:?}", lib_name, item);

    Ok(ItemIdentifier {
        lib_name: lib_name.to_string(),
        item,
    })
}

/// Resolve the markdown file path for a parsed item identifier.
///
/// If no item is specified, returns the path to index.md (crate overview).
/// If an item is specified, looks up the item in metadata.json and
/// returns the corresponding markdown file path.
fn resolve_markdown_path(parsed: &ItemIdentifier) -> Result<PathBuf> {
    let metadata = cargo::metadata()?;
    let docmd_dir = PathBuf::from(&metadata.target_directory).join("docmd");
    let lib_docmd_dir = docmd_dir.join(&parsed.lib_name);

    let parsed_item = match &parsed.item {
        None => {
            let index_md = lib_docmd_dir.join("index.md");
            trace!("No item specified, returning index.md: {:?}", index_md);
            return Ok(index_md);
        }
        Some(item) => item,
    };

    let metadata_path = lib_docmd_dir.join("metadata.json");
    let metadata_content = fs::read_to_string(&metadata_path)
        .with_context(|| format!("failed to read metadata file '{}'", metadata_path.display()))?;

    let crate_metadata: CrateDocMetadata =
        serde_json::from_str(&metadata_content).with_context(|| "failed to parse metadata.json")?;
    trace!(
        "Loaded metadata: crate_name={}, lib_name={}, items={}",
        crate_metadata.crate_name,
        crate_metadata.lib_name,
        crate_metadata.item_map.len()
    );

    let full_item_path = format!("{}::{}", parsed.lib_name, parsed_item);
    trace!("Looking up item path: {}", full_item_path);

    let relative_md_path = match crate_metadata.item_map.get(&full_item_path) {
        Some(p) => p,
        None => bail!(
            "could not resolve item path '{}'. The item may not exist. Try: `cargo txt list {}` to see all available items.",
            full_item_path,
            crate_metadata.lib_name
        ),
    };

    trace!("Found markdown path: {}", relative_md_path);

    let markdown_path = lib_docmd_dir.join(relative_md_path);

    debug!("Resolved markdown path: {:?}", markdown_path);

    Ok(markdown_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_crate_name() {
        let result = parse_item_identifier("serde").unwrap();
        assert_eq!(result.lib_name, "serde");
        assert_eq!(result.item, None);
    }

    #[test]
    fn parse_item_path_single_item() {
        let result = parse_item_identifier("serde::Error").unwrap();
        assert_eq!(result.lib_name, "serde");
        assert_eq!(result.item, Some("Error".to_string()));
    }

    #[test]
    fn parse_item_path_nested() {
        let result = parse_item_identifier("serde::ser::StdError").unwrap();
        assert_eq!(result.lib_name, "serde");
        assert_eq!(result.item, Some("ser::StdError".to_string()));
    }

    #[test]
    fn parse_item_path_deeply_nested() {
        let result = parse_item_identifier("serde::de::value::Error").unwrap();
        assert_eq!(result.lib_name, "serde");
        assert_eq!(result.item, Some("de::value::Error".to_string()));
    }

    #[test]
    fn parse_item_identifier_invalid_leading_separator() {
        let result = parse_item_identifier("::invalid");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item identifier '::invalid'"));
    }

    #[test]
    fn parse_item_path_empty_string() {
        let result = parse_item_identifier("");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item identifier ''"));
    }

    #[test]
    fn parse_item_path_only_separators() {
        let result = parse_item_identifier("::");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item identifier '::'"));
    }

    #[test]
    fn parse_item_path_trailing_separator() {
        let result = parse_item_identifier("serde::Error::").unwrap();
        assert_eq!(result.lib_name, "serde");
        assert_eq!(result.item, Some("Error::".to_string()));
    }

    #[test]
    fn error_message_preserves_user_input_format() {
        let parsed = ItemIdentifier {
            lib_name: "rustdoc-types".to_string(),
            item: Some("Abi".to_string()),
        };

        assert_eq!(parsed.lib_name, "rustdoc-types");

        let user_item_path = format!("{}::{}", parsed.lib_name, parsed.item.as_ref().unwrap());
        assert_eq!(user_item_path, "rustdoc-types::Abi");

        let parsed_underscores = ItemIdentifier {
            lib_name: "rustdoc_types".to_string(),
            item: Some("Abi".to_string()),
        };
        assert_eq!(parsed_underscores.lib_name, "rustdoc_types");
        let user_item_path_underscores = format!(
            "{}::{}",
            parsed_underscores.lib_name,
            parsed_underscores.item.as_ref().unwrap()
        );
        assert_eq!(user_item_path_underscores, "rustdoc_types::Abi");
    }
}
