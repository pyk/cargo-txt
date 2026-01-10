//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! converting the generated HTML to markdown, and writing the result.

use anyhow::{Context, Result, bail, ensure};
use log::{debug, info};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cargo;
use crate::html2md;

/// Metadata about a crate's documentation.
///
/// This struct contains information about the relationship between the
/// crate name (from Cargo.toml) and the library name (from cargo doc output),
/// as well as a mapping of item paths to their markdown files.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateDocMetadata {
    /// The dependency name from Cargo.toml (e.g., "rustdoc-types")
    pub crate_name: String,
    /// The root namespace name from cargo doc (e.g., "rustdoc_types")
    pub lib_name: String,
    /// A mapping of full Rust paths to markdown file paths
    pub item_map: HashMap<String, String>,
}

/// Cargo documentation output from HTML files.
///
/// Contains all HTML files read from cargo doc output and metadata
/// extracted from those files.
struct CargoDocOutput {
    /// The path to the cargo doc output directory
    path: PathBuf,
    /// A mapping of file paths to their HTML content
    files: HashMap<String, String>,
    /// Metadata extracted from all.html
    metadata: CrateDocMetadata,
}

/// Processed markdown documentation ready to be saved.
///
/// Contains transformed markdown files ready to be written to disk.
struct DocOutput {
    /// The path to the output directory
    path: PathBuf,
    /// A mapping of file paths to their markdown content
    files: HashMap<String, String>,
}

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc,
/// converts the generated HTML to markdown, and writes the result to the output directory.
pub fn build(crate_name: &str) -> Result<()> {
    debug!("Building documentation for crate: {}", crate_name);

    let cargo_metadata = cargo::metadata()?;

    debug!("Target directory: {}", cargo_metadata.target_directory);

    validate_crate_name(crate_name, &cargo_metadata)?;

    info!("Running cargo doc --package {} --no-deps", crate_name);

    let cargo_doc_output_dir = cargo::doc(crate_name)?;

    debug!("Cargo doc output directory: {:?}", cargo_doc_output_dir);

    let cargo_doc_output = read_cargo_doc_output(&cargo_doc_output_dir, crate_name)?;
    let doc_output = process_cargo_doc_output(cargo_doc_output)?;
    save_doc(doc_output)?;

    Ok(())
}

/// Validate that a crate name exists in the project dependencies.
///
/// Returns an error if the crate name is not found in the list of
/// available dependencies from cargo metadata.
fn validate_crate_name(crate_name: &str, cargo_metadata: &cargo::Metadata) -> Result<()> {
    let available_crates: Vec<&str> = cargo_metadata.packages[0]
        .dependencies
        .iter()
        .map(|dep| dep.name.as_str())
        .collect();

    ensure!(
        available_crates.contains(&crate_name),
        concat!(
            "Crate '{}' is not an installed dependency.\n",
            "\n",
            "Available crates: {}\n",
            "\n",
            "Only installed dependencies can be built. ",
            "Add the crate to Cargo.toml as a dependency first."
        ),
        crate_name,
        available_crates.join(", ")
    );
    Ok(())
}

/// Read cargo doc output directory and extract all HTML files and metadata.
///
/// This function reads all HTML files from the cargo doc output directory
/// and builds metadata by parsing the all.html file.
fn read_cargo_doc_output(cargo_doc_output_dir: &Path, crate_name: &str) -> Result<CargoDocOutput> {
    debug!("Reading cargo doc output from: {:?}", cargo_doc_output_dir);

    let index_html_path = cargo_doc_output_dir.join("index.html");
    ensure!(
        index_html_path.exists(),
        "index.html not found in cargo doc output directory '{}'",
        cargo_doc_output_dir.display()
    );

    let all_html_path = cargo_doc_output_dir.join("all.html");
    ensure!(
        all_html_path.exists(),
        "all.html not found in cargo doc output directory '{}'",
        cargo_doc_output_dir.display()
    );

    let mut files = HashMap::new();

    let all_html_content = fs::read_to_string(&all_html_path)
        .with_context(|| format!("failed to read file '{}'", all_html_path.display()))?;
    let all_html_relative = all_html_path
        .strip_prefix(cargo_doc_output_dir)?
        .to_string_lossy()
        .to_string();
    files.insert(all_html_relative, all_html_content.clone());

    let index_html_content = fs::read_to_string(&index_html_path)
        .with_context(|| format!("failed to read file '{}'", index_html_path.display()))?;
    let index_html_relative = index_html_path
        .strip_prefix(cargo_doc_output_dir)?
        .to_string_lossy()
        .to_string();
    files.insert(index_html_relative, index_html_content);

    let item_map = extract_item_mappings_from_html(&all_html_content)?;

    let lib_name = cargo_doc_output_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(crate_name);

    let metadata = CrateDocMetadata {
        crate_name: crate_name.to_string(),
        lib_name: lib_name.to_string(),
        item_map,
    };

    Ok(CargoDocOutput {
        path: cargo_doc_output_dir.to_path_buf(),
        files,
        metadata,
    })
}

/// Extract raw item mappings from all.html content (without prefixing).
///
/// Parses the all.html file to extract mappings between item names
/// (e.g., `Error`) and their corresponding HTML file paths
/// (e.g., `struct.Error.html`).
///
/// Returns a HashMap mapping item names to HTML file paths.
fn extract_item_mappings_from_html(html: &str) -> Result<HashMap<String, String>> {
    let mut mappings = HashMap::new();

    let document = Html::parse_document(html);
    let selector = match Selector::parse("ul.all-items li a") {
        Ok(s) => s,
        Err(e) => bail!("failed to parse HTML selector for item mappings: {}", e),
    };

    for element in document.select(&selector) {
        let href = match element.value().attr("href") {
            Some(h) => h,
            None => bail!("href attribute not found in item link"),
        };

        let text: String = element.text().collect();

        mappings.insert(text, href.to_string());
    }

    if mappings.is_empty() {
        bail!("failed to find item mappings in documentation - no items found");
    }

    Ok(mappings)
}

/// Process cargo doc output and convert to markdown.
///
/// Transforms HTML files to markdown format and builds the output
/// structure ready to be saved.
fn process_cargo_doc_output(cargo_doc_output: CargoDocOutput) -> Result<DocOutput> {
    debug!("Processing cargo doc output");

    let mut files = HashMap::new();
    let lib_name = &cargo_doc_output.metadata.lib_name;
    let item_map = &cargo_doc_output.metadata.item_map;

    let index_html_key = "index.html";
    let Some(index_html_content) = cargo_doc_output.files.get(index_html_key) else {
        bail!("index.html not found in cargo doc output files");
    };
    let index_markdown = html2md::convert(index_html_content)?;
    files.insert("index.md".to_string(), index_markdown);
    debug!("Converted index.html to index.md");

    let all_html_key = "all.html";
    let Some(all_html_content) = cargo_doc_output.files.get(all_html_key) else {
        bail!("all.html not found in cargo doc output files");
    };
    let all_markdown_raw = html2md::convert(all_html_content)?;
    let all_markdown_formatted = format_all_md(lib_name, &all_markdown_raw);
    files.insert("all.md".to_string(), all_markdown_formatted);
    debug!("Converted all.html to all.md");

    let mut updated_item_map = HashMap::new();

    for (item_name, html_path) in item_map {
        let full_item_path = format!("{}::{}", lib_name, item_name);
        debug!("Converting item: {}", full_item_path);

        let full_html_path = cargo_doc_output.path.join(html_path);
        let html_content = fs::read_to_string(&full_html_path)
            .with_context(|| format!("failed to read HTML file '{}'", full_html_path.display()))?;

        let markdown_content = html2md::convert(&html_content)?;

        let md_path = PathBuf::from(html_path).with_extension("md");
        let md_key = md_path.to_string_lossy().to_string();

        files.insert(md_key, markdown_content);
        updated_item_map.insert(full_item_path, md_path.to_string_lossy().to_string());
    }

    info!("Converted {} items to markdown", files.len() - 2);

    let updated_metadata = CrateDocMetadata {
        crate_name: cargo_doc_output.metadata.crate_name.clone(),
        lib_name: cargo_doc_output.metadata.lib_name.clone(),
        item_map: updated_item_map,
    };

    let metadata_json = serde_json::to_string_pretty(&updated_metadata)
        .with_context(|| "failed to serialize metadata to JSON")?;
    files.insert("metadata.json".to_string(), metadata_json);
    debug!("Added metadata.json with updated item_map");

    let docmd_base_path = match cargo_doc_output.path.parent() {
        Some(val) => val,
        None => bail!("cargo doc output path has no parent directory"),
    };
    let docmd_base_path = match docmd_base_path.parent() {
        Some(val) => val,
        None => bail!("doc base path has no parent directory"),
    };
    let output_path = docmd_base_path.join("docmd").join(lib_name);

    Ok(DocOutput {
        path: output_path,
        files,
    })
}

/// Save documentation output to disk.
///
/// Writes all markdown files to the output directory, creating
/// subdirectories as needed.
fn save_doc(doc_output: DocOutput) -> Result<()> {
    debug!("Saving documentation to: {:?}", doc_output.path);

    if !doc_output.path.exists() {
        fs::create_dir_all(&doc_output.path).with_context(|| {
            format!(
                "failed to create output directory '{}'",
                doc_output.path.display()
            )
        })?;
    }

    for (relative_path, content) in &doc_output.files {
        let full_path = doc_output.path.join(relative_path);

        let parent = match full_path.parent() {
            Some(p) => p,
            None => bail!("file path has no parent directory"),
        };
        if !parent.exists() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory '{}'", parent.display()))?;
        }

        fs::write(&full_path, content)
            .with_context(|| format!("failed to write file '{}'", full_path.display()))?;

        debug!("Generated markdown: {}", full_path.display());
    }

    let lib_name = doc_output
        .path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let total_files = doc_output.files.len();
    let item_count = total_files.saturating_sub(3);

    println!(
        "✓ Built documentation for {} ({} items)",
        lib_name, item_count
    );
    println!("  Run `cargo txt list {}` to see all items", lib_name);

    info!("Successfully saved documentation");
    Ok(())
}

/// Format all.md content with crate name as H1 heading and prefixed items.
///
/// This function post-processes the raw markdown content from all.html to:
/// - Add crate name as H1 heading at the top
/// - Convert existing H1 "# List of all items" to a paragraph
/// - Prefix all list items with crate name
/// - Append usage instructions at the end
///
/// * `crate_name` - The name of the crate (e.g., "serde")
/// * `content` - The raw markdown content from all.html
///
/// # Examples
///
/// ```
/// let raw = "# List of all items\n\n### Structs\n\n- Error\n";
/// let formatted = format_all_md("serde", raw);
/// // formatted starts with "# serde\n\nList of all items\n\n### Structs\n\n- serde::Error\n"
/// ```
fn format_all_md(crate_name: &str, content: &str) -> String {
    let mut result = Vec::new();
    let mut lines = content.lines();

    result.push(format!("# {}", crate_name));
    result.push(String::new());

    let Some(first_line) = lines.next() else {
        return result.join("\n");
    };

    if first_line.starts_with("# List of all items") {
        result.push(first_line[2..].to_string());
    } else {
        result.push(first_line.to_string());
    }

    let mut first_items: Vec<String> = Vec::new();
    let mut in_section = false;

    for line in lines {
        if line.starts_with("### ") {
            in_section = true;
            result.push(line.to_string());
        } else {
            match line.strip_prefix("- ") {
                Some(item) => {
                    result.push(format!("- {}::{}", crate_name, item));

                    if in_section {
                        first_items.push(format!("{}::{}", crate_name, item));
                        in_section = false;
                    }
                }
                None => {
                    result.push(line.to_string());
                }
            }
        }
    }

    result.push(String::new());
    result.push("## Usage".to_string());
    result.push(String::new());
    result.push("To view documentation for a specific item, use the `show` command:".to_string());
    result.push(String::new());
    result.push("```shell".to_string());
    result.push("cargo txt show <ITEM_PATH>".to_string());
    result.push("```".to_string());
    result.push(String::new());
    result.push("Examples:".to_string());
    result.push(String::new());
    result.push("```shell".to_string());

    for item in first_items.iter().take(3) {
        result.push(format!("cargo txt show {}", item));
    }

    if first_items.is_empty() {
        result.push(format!("cargo txt show {}::SomeItem", crate_name));
    }

    result.push("```".to_string());

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_all_md_comprehensive() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config\n\n### Traits\n\n- Serialize\n- Deserialize\n\n### Enums\n\n- Value";

        let result = format_all_md("serde", content);

        assert_eq!(&result[..7], "# serde");

        assert!(result.contains("\nList of all items\n"));
        assert!(!result.contains("# List of all items"));

        assert!(result.contains("\n### Structs\n"));
        assert!(result.contains("\n### Traits\n"));
        assert!(result.contains("\n### Enums\n"));

        assert!(result.contains("- serde::Error"));
        assert!(result.contains("- serde::Config"));
        assert!(result.contains("- serde::Serialize"));
        assert!(result.contains("- serde::Deserialize"));
        assert!(result.contains("- serde::Value"));

        assert!(result.contains("\n## Usage\n"));
        assert!(result.contains("cargo txt show <ITEM_PATH>"));
        assert!(result.contains("cargo txt show serde::Error"));
        assert!(result.contains("cargo txt show serde::Serialize"));
        assert!(result.contains("cargo txt show serde::Value"));

        assert!(!result.contains("cargo txt show serde::SomeStruct"));
        assert!(!result.contains("cargo txt show serde::SomeTrait"));
        assert!(!result.contains("cargo txt show serde::SomeEnum"));
    }

    #[test]
    fn format_all_md_uses_crate_name_as_is() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config";
        let result = format_all_md("rustdoc_types", content);

        assert_eq!(&result[..15], "# rustdoc_types");

        assert!(result.contains("- rustdoc_types::Error"));
        assert!(result.contains("- rustdoc_types::Config"));
    }

    #[test]
    fn format_all_md_preserves_non_list_lines() {
        let content = "# List of all items\n\n### Structs\n\nSome text\n\n- Error";
        let result = format_all_md("serde", content);

        assert!(result.contains("### Structs"));
        assert!(result.contains("\nSome text\n"));
        assert!(result.contains("- serde::Error"));
    }

    #[test]
    fn format_all_md_usage_instructions_fallback() {
        let content = "# List of all items\n\nNo items here";
        let result = format_all_md("my_crate", content);

        assert!(result.contains("cargo txt show my_crate::SomeItem"));

        assert!(!result.contains("cargo txt show my_crate::Error"));
        assert!(!result.contains("cargo txt show my_crate::Struct"));
    }

    #[test]
    fn format_all_md_usage_instructions_limit_to_3_sections() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config\n\n### Traits\n\n- Serialize\n- Deserialize\n\n### Enums\n\n- Value\n\n### Constants\n\n- VERSION";
        let result = format_all_md("serde", content);

        assert!(result.contains("cargo txt show serde::Error"));
        assert!(result.contains("cargo txt show serde::Serialize"));
        assert!(result.contains("cargo txt show serde::Value"));

        assert!(!result.contains("cargo txt show serde::VERSION"));

        assert!(result.contains("- serde::VERSION"));
    }

    #[test]
    fn extract_item_mappings_from_html_simple() {
        let html = r#"
            <html>
                <body>
                    <h3 id="structs">Structs</h3>
                    <ul class="all-items">
                        <li><a href="struct.Error.html">Error</a></li>
                        <li><a href="struct.Config.html">Config</a></li>
                    </ul>
                </body>
            </html>
        "#;

        let mappings = extract_item_mappings_from_html(html).unwrap();
        assert_eq!(mappings.len(), 2);
        assert_eq!(
            mappings.get("Error"),
            Some(&"struct.Error.html".to_string())
        );
        assert_eq!(
            mappings.get("Config"),
            Some(&"struct.Config.html".to_string())
        );
    }

    #[test]
    fn extract_item_mappings_from_html_nested_paths() {
        let html = r#"
            <html>
                <body>
                    <h3 id="structs">Structs</h3>
                    <ul class="all-items">
                        <li><a href="de/struct.IgnoredAny.html">de::IgnoredAny</a></li>
                        <li><a href="ser/trait.StdError.html">ser::StdError</a></li>
                        <li><a href="de/value/struct.Error.html">de::value::Error</a></li>
                    </ul>
                </body>
            </html>
        "#;

        let mappings = extract_item_mappings_from_html(html).unwrap();
        assert_eq!(mappings.len(), 3);
        assert_eq!(
            mappings.get("de::IgnoredAny"),
            Some(&"de/struct.IgnoredAny.html".to_string())
        );
        assert_eq!(
            mappings.get("ser::StdError"),
            Some(&"ser/trait.StdError.html".to_string())
        );
        assert_eq!(
            mappings.get("de::value::Error"),
            Some(&"de/value/struct.Error.html".to_string())
        );
    }

    #[test]
    fn extract_item_mappings_from_html_multiple_sections() {
        let html = r#"
            <html>
                <body>
                    <h3 id="structs">Structs</h3>
                    <ul class="all-items">
                        <li><a href="struct.Error.html">Error</a></li>
                    </ul>
                    <h3 id="traits">Traits</h3>
                    <ul class="all-items">
                        <li><a href="trait.Serialize.html">Serialize</a></li>
                        <li><a href="trait.Deserialize.html">Deserialize</a></li>
                    </ul>
                    <h3 id="enums">Enums</h3>
                    <ul class="all-items">
                        <li><a href="enum.Value.html">Value</a></li>
                    </ul>
                </body>
            </html>
        "#;

        let mappings = extract_item_mappings_from_html(html).unwrap();
        assert_eq!(mappings.len(), 4);
        assert_eq!(
            mappings.get("Error"),
            Some(&"struct.Error.html".to_string())
        );
        assert_eq!(
            mappings.get("Serialize"),
            Some(&"trait.Serialize.html".to_string())
        );
        assert_eq!(
            mappings.get("Deserialize"),
            Some(&"trait.Deserialize.html".to_string())
        );
        assert_eq!(mappings.get("Value"), Some(&"enum.Value.html".to_string()));
    }

    #[test]
    fn extract_item_mappings_from_html_empty_html() {
        let html = r#"
            <html>
                <body>
                    <p>No items here</p>
                </body>
            </html>
        "#;

        let result = extract_item_mappings_from_html(html);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("failed to find item mappings"));
    }

    #[test]
    fn extract_item_mappings_from_html_no_links() {
        let html = r#"
            <html>
                <body>
                    <ul class="all-items">
                        <li>Text without link</li>
                    </ul>
                </body>
            </html>
        "#;
        let result = extract_item_mappings_from_html(html);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("failed to find item mappings"));
    }
}
