//! Show command implementation.
//!
//! This module provides the show command which displays crate documentation
//! to stdout. Users can view either the entire crate documentation (all.md)
//! or specific items by providing an item path.

use anyhow::{Context, Result, bail};
use log::{debug, info, trace};
use std::fs;
use std::path::PathBuf;

use crate::cargo;
use crate::commands;

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

    build_if_needed(&parsed)?;

    let markdown_path = resolve_markdown_path(&parsed)?;
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

/// Resolve the markdown file path for a parsed item path.
///
/// If no item is specified, returns the path to all.md (master index).
/// If an item is specified, looks up the item in all.html mappings and
/// returns the corresponding markdown file path.
fn resolve_markdown_path(parsed: &ParsedItemPath) -> Result<PathBuf> {
    let metadata = cargo::metadata()?;
    let docmd_dir = PathBuf::from(&metadata.target_directory).join("docmd");
    let crate_docmd_dir = docmd_dir.join(&parsed.crate_name);

    let parsed_item = match &parsed.item {
        None => {
            let all_md = crate_docmd_dir.join("all.md");
            trace!("No item specified, returning all.md: {:?}", all_md);
            return Ok(all_md);
        }
        Some(item) => item,
    };

    let html_dir = PathBuf::from(&metadata.target_directory)
        .join("doc")
        .join(&parsed.crate_name);

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

    let full_item_path = format!("{}::{}", parsed.crate_name, parsed_item);
    trace!("Looking up item path: {}", full_item_path);

    let html_path = match item_mappings.get(&full_item_path) {
        Some(p) => p,
        None => bail!(
            r#"could not resolve item path '{}'. Please ensure the item exists in the crate and try: `cargo txt build {}`"#,
            full_item_path,
            parsed.crate_name
        ),
    };

    trace!("Found HTML path: {}", html_path);

    let markdown_path = crate_docmd_dir.join(PathBuf::from(html_path).with_extension("md"));

    debug!("Resolved markdown path: {:?}", markdown_path);

    Ok(markdown_path)
}

/// Build documentation if needed.
///
/// Checks if the all.md file exists for the crate. If not, triggers
/// a build to generate all markdown files.
fn build_if_needed(parsed: &ParsedItemPath) -> Result<()> {
    let metadata = cargo::metadata()?;
    let all_md_path = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(&parsed.crate_name)
        .join("all.md");

    if all_md_path.exists() {
        debug!("Documentation exists at {:?}, skipping build", all_md_path);
        return Ok(());
    }

    info!(
        "Documentation not found, running build for {}",
        parsed.crate_name
    );
    commands::build::build(&parsed.crate_name)?;

    Ok(())
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
}
