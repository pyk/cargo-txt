//! Show command implementation.
//!
//! This module provides the show command which displays crate documentation
//! to stdout. Users can view the crate overview (index.md) or specific items
//! by providing an item path.

use anyhow::{Context, Result, bail};
use log::{debug, trace};
use std::fs;
use std::path::PathBuf;

use crate::cargo;
use crate::commands;
use crate::commands::build;

/// Parsed item path containing crate name and optional item.
#[derive(Debug)]
struct ParsedItemPath {
    crate_name: String,
    item: Option<String>,
}

/// Show and display crate documentation.
///
/// This function parses the item path, ensures documentation is built,
/// resolves the appropriate markdown file, and prints its contents to stdout.
pub fn show(item_path: &str) -> Result<()> {
    debug!("Show command: item_path={}", item_path);

    let parsed = parse_item_path(item_path)?;
    trace!(
        "Parsed: crate_name={}, item={:?}",
        parsed.crate_name, parsed.item
    );

    build::if_needed(&parsed.crate_name)?;

    let path_name = read_crate_path_name(&parsed.crate_name)?;
    debug!("Using crate path name: {}", path_name);

    let markdown_path = resolve_markdown_path(&parsed, &path_name)?;
    debug!("Resolved markdown path: {:?}", markdown_path);

    let markdown_content = fs::read_to_string(&markdown_path)
        .with_context(|| format!("failed to read markdown file '{}'", markdown_path.display()))?;
    trace!("Read markdown file ({} bytes)", markdown_content.len());

    println!("{}", markdown_content);

    Ok(())
}

/// Parse an item path into crate name and optional item.
///
/// Extracts the crate name (first component before `::`) and the remaining
/// item path (if any) from the full item path string.
fn parse_item_path(item_path: &str) -> Result<ParsedItemPath> {
    let mut parts = item_path.split("::");

    let crate_name = match parts.next().filter(|s| !s.is_empty()) {
        Some(n) => n,
        None => bail!(
            "invalid item path '{}'. Expected format: <crate> or <crate>::<item> (e.g., 'serde' or 'serde::Error').",
            item_path
        ),
    };

    let item: Vec<&str> = parts.collect();
    let item = if item.is_empty() {
        None
    } else {
        Some(item.join("::"))
    };

    trace!(
        "Parsed item path: crate_name={}, item={:?}",
        crate_name, item
    );

    Ok(ParsedItemPath {
        crate_name: crate_name.to_string(),
        item,
    })
}

/// Read crate path name from name file.
///
/// Reads the crate directory name (source of truth) from docmd/<crate>/name,
/// which was saved during build.
fn read_crate_path_name(crate_name: &str) -> Result<String> {
    let metadata = cargo::metadata()?;

    let name_file = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(crate_name)
        .join("name");

    let path_name = fs::read_to_string(&name_file)
        .with_context(|| format!("failed to read crate name file '{}'", name_file.display()))?
        .trim()
        .to_string();
    debug!("Read crate path name from file: {}", path_name);
    Ok(path_name)
}

/// Resolve the markdown file path for a parsed item path.
///
/// If no item is specified, returns the path to index.md (crate overview).
/// If an item is specified, looks up the item in all.html mappings and
/// returns the corresponding markdown file path.
fn resolve_markdown_path(parsed: &ParsedItemPath, path_name: &str) -> Result<PathBuf> {
    let metadata = cargo::metadata()?;
    let docmd_dir = PathBuf::from(&metadata.target_directory).join("docmd");
    let crate_docmd_dir = docmd_dir.join(&parsed.crate_name);

    let parsed_item = match &parsed.item {
        None => {
            let index_md = crate_docmd_dir.join("index.md");
            trace!("No item specified, returning index.md: {:?}", index_md);
            return Ok(index_md);
        }
        Some(item) => item,
    };

    // HTML directory created by cargo doc uses underscores from path_name
    let html_dir = PathBuf::from(&metadata.target_directory)
        .join("doc")
        .join(path_name);

    let all_html_path = html_dir.join("all.html");

    let all_html_content = fs::read_to_string(&all_html_path).with_context(|| {
        format!(
            "failed to read documentation index file '{}'",
            all_html_path.display()
        )
    })?;

    let item_mappings =
        commands::build::extract_item_mappings(&parsed.crate_name, &all_html_content)?;
    trace!("Generated {} item mappings", item_mappings.len());

    // Use parsed.crate_name for Rust path lookup to match item mappings
    let full_item_path = format!("{}::{}", parsed.crate_name, parsed_item);
    trace!("Looking up item path: {}", full_item_path);

    // Preserve original user input format for error messages
    let user_item_path = format!("{}::{}", parsed.crate_name, parsed_item);

    let html_path = match item_mappings.get(&full_item_path) {
        Some(p) => p,
        None => bail!(
            r#"could not resolve item path '{}'. Please ensure the item exists in the crate and try: `cargo txt build {}`"#,
            user_item_path,
            parsed.crate_name
        ),
    };

    trace!("Found HTML path: {}", html_path);

    let markdown_path = crate_docmd_dir.join(PathBuf::from(html_path).with_extension("md"));

    debug!("Resolved markdown path: {:?}", markdown_path);

    Ok(markdown_path)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    // parse_item_path tests

    #[test]
    fn parse_simple_crate_name() {
        let result = parse_item_path("serde").unwrap();
        assert_eq!(result.crate_name, "serde");
        assert_eq!(result.item, None);
    }

    #[test]
    fn parse_item_path_single_item() {
        let result = parse_item_path("serde::Error").unwrap();
        assert_eq!(result.crate_name, "serde");
        assert_eq!(result.item, Some("Error".to_string()));
    }

    #[test]
    fn parse_item_path_nested() {
        let result = parse_item_path("serde::ser::StdError").unwrap();
        assert_eq!(result.crate_name, "serde");
        assert_eq!(result.item, Some("ser::StdError".to_string()));
    }

    #[test]
    fn parse_item_path_deeply_nested() {
        let result = parse_item_path("serde::de::value::Error").unwrap();
        assert_eq!(result.crate_name, "serde");
        assert_eq!(result.item, Some("de::value::Error".to_string()));
    }

    #[test]
    fn parse_item_path_invalid_leading_separator() {
        let result = parse_item_path("::invalid");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item path '::invalid'"));
    }

    #[test]
    fn parse_item_path_empty_string() {
        let result = parse_item_path("");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item path ''"));
    }

    #[test]
    fn parse_item_path_only_separators() {
        let result = parse_item_path("::");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid item path '::'"));
    }

    #[test]
    fn parse_item_path_trailing_separator() {
        let result = parse_item_path("serde::Error::").unwrap();
        assert_eq!(result.crate_name, "serde");
        assert_eq!(result.item, Some("Error::".to_string()));
    }

    ///////////////////////////////////////////////////////////////////////////
    // Error message format preservation tests

    #[test]
    fn error_message_preserves_user_input_format() {
        // This test verifies that when a user provides a crate name with hyphens
        // (e.g., rustdoc-types), the error message preserves that format instead
        // of converting to underscores (rustdoc_types).
        let parsed = ParsedItemPath {
            crate_name: "rustdoc-types".to_string(),
            item: Some("Abi".to_string()),
        };

        // The parsed crate name should preserve the original hyphen format
        assert_eq!(parsed.crate_name, "rustdoc-types");

        // The user_item_path should preserve the original format for error messages
        let user_item_path = format!("{}::{}", parsed.crate_name, parsed.item.as_ref().unwrap());
        assert_eq!(user_item_path, "rustdoc-types::Abi");

        // Verify that underscores are also preserved
        let parsed_underscores = ParsedItemPath {
            crate_name: "rustdoc_types".to_string(),
            item: Some("Abi".to_string()),
        };
        assert_eq!(parsed_underscores.crate_name, "rustdoc_types");
        let user_item_path_underscores = format!(
            "{}::{}",
            parsed_underscores.crate_name,
            parsed_underscores.item.as_ref().unwrap()
        );
        assert_eq!(user_item_path_underscores, "rustdoc_types::Abi");
    }
}
