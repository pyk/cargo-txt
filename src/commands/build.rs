//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! converting the generated HTML to markdown, and writing the result.

use anyhow::{Context, Result, bail};
use log::{debug, info};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cargo;
use crate::html2md;

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc,
/// converts the generated HTML to markdown, and writes the result to the output directory.
pub fn build(crate_name: &str) -> Result<()> {
    debug!("Building documentation for crate: {}", crate_name);

    // Get cargo metadata and validate the crate
    let metadata = cargo::metadata()?;

    debug!("Target directory: {}", metadata.target_directory);

    // Create the available list once and check if crate exists
    let available_list: Vec<&str> = metadata.packages[0]
        .dependencies
        .iter()
        .map(|dep| dep.name.as_str())
        .collect();

    let crate_not_exists = !available_list.contains(&crate_name);
    if crate_not_exists {
        bail!(
            concat!(
                "Crate '{}' is not an installed dependency.\n",
                "\n",
                "Available crates: {}\n",
                "\n",
                "Only installed dependencies can be built. ",
                "Add the crate to Cargo.toml as a dependency first."
            ),
            crate_name,
            available_list.join(", ")
        );
    }

    // Generate HTML documentation
    info!("Running cargo doc --package {} --no-deps", crate_name);
    let html_dir = cargo::doc(crate_name)?;

    debug!("HTML directory: {:?}", html_dir);

    // Read the index.html file
    let html_path = html_dir.join("index.html");
    debug!("Reading HTML file: {:?}", html_path);
    let html_content = fs::read_to_string(&html_path)
        .with_context(|| format!("failed to read file '{}'", html_path.display()))?;

    // Convert HTML to markdown
    debug!("Converting HTML to markdown ({} bytes)", html_content.len());
    let markdown_content = html2md::convert(&html_content)?;

    debug!("Markdown content ({} bytes)", markdown_content.len());

    // Create output directory structure: target/docmd/<crate>/
    let output_dir = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(crate_name);

    if !output_dir.exists() {
        debug!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(&output_dir).with_context(|| {
            format!(
                "failed to create output directory '{}'",
                output_dir.display()
            )
        })?;
    }

    // Write markdown to index.md
    let index_path = output_dir.join("index.md");
    debug!("Writing markdown to: {:?}", index_path);
    fs::write(&index_path, markdown_content)
        .with_context(|| format!("failed to write markdown file '{}'", index_path.display()))?;

    info!("Successfully generated markdown");
    println!("Generated markdown: {}", index_path.display());

    // Process all.html
    info!("Processing all.html");
    let all_html_path = html_dir.join("all.html");

    let all_html_content = fs::read_to_string(&all_html_path)
        .with_context(|| format!("failed to read file '{}'", all_html_path.display()))?;

    debug!(
        "Converting all.html to markdown ({} bytes)",
        all_html_content.len()
    );
    let all_markdown_content = html2md::convert(&all_html_content)?;

    // Get the crate directory name (source of truth from cargo doc output)
    let crate_dir_name = html_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(crate_name);
    debug!("Crate directory name (source of truth): {}", crate_dir_name);

    // Format all.md with crate name heading and prefixed items
    let formatted_content = format_all_md(crate_dir_name, &all_markdown_content);

    let all_path = output_dir.join("all.md");
    debug!("Writing all.md to: {:?}", all_path);
    fs::write(&all_path, formatted_content)
        .with_context(|| format!("failed to write markdown file '{}'", all_path.display()))?;

    info!("Generated all.md");
    println!("Generated markdown: {}", all_path.display());

    info!("Extracting item mappings from all.html");
    let item_mappings = extract_item_mappings(crate_dir_name, &all_html_content)?;
    debug!("Found {} items to convert", item_mappings.len());

    // Generate markdown for each item
    for html_relative_path in item_mappings.values() {
        let html_path = html_dir.join(html_relative_path);
        let relative_md_path = PathBuf::from(html_relative_path).with_extension("md");
        let md_path = output_dir.join(&relative_md_path);

        debug!("Converting {:?} to {:?}", html_path, relative_md_path);

        let html_content = fs::read_to_string(&html_path)
            .with_context(|| format!("failed to read file '{}'", html_path.display()))?;

        let markdown_content = html2md::convert(&html_content)?;

        let parent = match md_path.parent() {
            Some(p) => p,
            None => bail!("md_path has no parent directory"),
        };

        if !parent.exists() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create output directory '{}'", parent.display())
            })?;
        }

        fs::write(&md_path, markdown_content)
            .with_context(|| format!("failed to write markdown file '{}'", md_path.display()))?;
    }

    info!("Generated markdown for {} items", item_mappings.len());

    // Save crate path name for use by show and list commands
    save_crate_path_name(&output_dir, crate_dir_name)?;

    Ok(())
}

/// Save crate path name to a file in crate directory.
///
/// Stores the crate directory name (source of truth from cargo doc) in
/// docmd/<crate>/name for use by show and list commands.
fn save_crate_path_name(output_dir: &Path, path_name: &str) -> Result<()> {
    let name_path = output_dir.join("name");

    fs::write(&name_path, path_name)
        .with_context(|| format!("failed to write crate name file '{}'", name_path.display()))?;

    debug!("Saved crate path name to {:?}", name_path);

    Ok(())
}

/// Extract item mappings from all.html.
///
/// Parses the all.html file to extract mappings between full Rust paths
/// (e.g., `serde::Error`) and their corresponding HTML file paths
/// (e.g., `struct.Error.html`).
///
/// Returns a HashMap mapping full Rust paths to HTML file paths.
pub fn extract_item_mappings(crate_name: &str, html: &str) -> Result<HashMap<String, String>> {
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

        // Build full Rust path by prefixing with crate name
        let full_path = format!("{}::{}", crate_name, text);

        mappings.insert(full_path, href.to_string());
    }

    if mappings.is_empty() {
        bail!("failed to find item mappings in documentation - no items found");
    }

    Ok(mappings)
}

/// Build documentation for a crate if needed.
///
/// Checks if the all.md file exists for the crate. If not, triggers
/// a build to generate all markdown files. Accepts both underscore
/// Checks if the documentation needs to be built.
///
/// Checks if the all.md file exists for the crate. If not, triggers
/// a build to generate all markdown files.
pub fn if_needed(crate_name: &str) -> Result<()> {
    let metadata = cargo::metadata()?;

    let all_md_path = PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(crate_name)
        .join("all.md");

    if all_md_path.exists() {
        debug!("Documentation exists at {:?}, skipping build", all_md_path);
        return Ok(());
    }

    info!("Documentation not found, running build for {}", crate_name);
    build(crate_name)?;

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
/// # Arguments
///
/// * `crate_name` - The name of the crate (e.g., "serde")
/// * `content` - The raw markdown content from all.html
///
/// # Returns
///
/// Formatted markdown content ready to be written to all.md
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

    // Add crate name as H1 at the start
    result.push(format!("# {}", crate_name));
    result.push(String::new());

    // Process first line
    let Some(first_line) = lines.next() else {
        // No lines, just return crate name and usage instructions
        return result.join("\n");
    };

    // Convert "# List of all items" to "List of all items"
    if first_line.starts_with("# List of all items") {
        result.push(first_line[2..].to_string());
    } else {
        result.push(first_line.to_string());
    }

    // Track first items from each section for usage examples
    let mut first_items: Vec<String> = Vec::new();
    let mut in_section = false;

    // Process remaining lines
    for line in lines {
        // Check if we're entering a new section (### SectionName)
        if line.starts_with("### ") {
            in_section = true;
            result.push(line.to_string());
        } else {
            match line.strip_prefix("- ") {
                Some(item) => {
                    result.push(format!("- {}::{}", crate_name, item));

                    // Collect first item from each section
                    if in_section {
                        first_items.push(format!("{}::{}", crate_name, item));
                        in_section = false; // Only collect first item per section
                    }
                }
                None => {
                    result.push(line.to_string());
                }
            }
        }
    }

    // Add usage instructions at the end
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

    // Add examples using first items from sections (up to 3)
    for item in first_items.iter().take(3) {
        result.push(format!("cargo txt show {}", item));
    }

    // Fallback to generic examples if no items found
    if first_items.is_empty() {
        result.push(format!("cargo txt show {}::SomeItem", crate_name));
    }

    result.push("```".to_string());

    result.join("\n")
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    // format_all_md tests

    #[test]
    fn format_all_md_adds_crate_name_heading() {
        let content = "# List of all items\n\nContent here";
        let result = format_all_md("serde", content);
        assert!(result.starts_with("# serde\n"));
    }

    #[test]
    fn format_all_md_converts_h1_to_paragraph() {
        let content = "# List of all items\n\nMore content";
        let result = format_all_md("serde", content);
        assert!(result.contains("\nList of all items\n"));
        assert!(!result.contains("# List of all items"));
    }

    #[test]
    fn format_all_md_prefixes_simple_items() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config";
        let result = format_all_md("serde", content);
        assert!(result.contains("- serde::Error"));
        assert!(result.contains("- serde::Config"));
    }

    #[test]
    fn format_all_md_prefixes_nested_items() {
        let content = "# List of all items\n\n### Structs\n\n- de::IgnoredAny\n- ser::StdError";
        let result = format_all_md("serde", content);
        assert!(result.contains("- serde::de::IgnoredAny"));
        assert!(result.contains("- serde::ser::StdError"));
    }

    #[test]
    fn format_all_md_multiple_sections() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n\n### Traits\n\n- Serialize\n- Deserialize\n\n### Enums\n\n- Value";
        let result = format_all_md("serde", content);
        assert!(result.contains("- serde::Error"));
        assert!(result.contains("- serde::Serialize"));
        assert!(result.contains("- serde::Deserialize"));
        assert!(result.contains("- serde::Value"));
    }

    #[test]
    fn format_all_md_appends_usage_instructions() {
        let content = "# List of all items\n\n### Structs\n\n- Error";
        let result = format_all_md("serde", content);
        assert!(result.contains("## Usage"));
        assert!(result.contains("cargo txt show <ITEM_PATH>"));
        assert!(result.contains("cargo txt show serde::Error"));
    }

    #[test]
    fn format_all_md_preserves_non_list_lines() {
        let content = "# List of all items\n\n### Structs\n\nSome text\n\n- Error";
        let result = format_all_md("serde", content);
        assert!(result.contains("### Structs"));
        assert!(result.contains("Some text"));
        assert!(result.contains("- serde::Error"));
    }

    #[test]
    fn format_all_md_uses_crate_name_as_is() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config";
        let result = format_all_md("rustdoc_types", content);
        // Crate name is used as-is for paths (source of truth from cargo doc)
        assert!(result.contains("- rustdoc_types::Error"));
        assert!(result.contains("- rustdoc_types::Config"));
        // H1 heading uses the same crate name
        assert!(result.starts_with("# rustdoc_types"));
    }

    #[test]
    fn format_all_md_usage_instructions_use_first_items() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config";
        let result = format_all_md("serde", content);
        // Usage instructions should use the first item from each section
        assert!(result.contains("cargo txt show serde::Error"));
        // Should NOT contain generic placeholders
        assert!(!result.contains("cargo txt show serde::SomeStruct"));
        assert!(!result.contains("cargo txt show serde::SomeTrait"));
        assert!(!result.contains("cargo txt show serde::SomeEnum"));
    }

    #[test]
    fn format_all_md_usage_instructions_with_crate_name() {
        let content = "# List of all items\n\n### Structs\n\n- Error";
        let result = format_all_md("my_crate", content);
        // Usage instructions use the first item from sections
        assert!(result.contains("cargo txt show my_crate::Error"));
        // Should NOT contain generic placeholders
        assert!(!result.contains("cargo txt show my_crate::SomeStruct"));
        assert!(!result.contains("cargo txt show my_crate::SomeTrait"));
        assert!(!result.contains("cargo txt show my_crate::SomeEnum"));
        // H1 heading uses the same crate name
        assert!(result.starts_with("# my_crate"));
        // Items also use the same crate name prefix
        assert!(result.contains("- my_crate::Error"));
    }

    #[test]
    fn format_all_md_usage_instructions_multiple_sections() {
        let content = "# List of all items\n\n### Structs\n\n- Error\n- Config\n\n### Traits\n\n- Serialize\n- Deserialize\n\n### Enums\n\n- Value\n\n### Constants\n\n- VERSION";
        let result = format_all_md("serde", content);
        // Usage instructions should include first items from each section (up to 3)
        assert!(result.contains("cargo txt show serde::Error"));
        assert!(result.contains("cargo txt show serde::Serialize"));
        assert!(result.contains("cargo txt show serde::Value"));
        // Should NOT include the 4th section's first item (Constants)
        assert!(!result.contains("cargo txt show serde::VERSION"));
        // Should still list all items in the sections
        assert!(result.contains("- serde::Error"));
        assert!(result.contains("- serde::Config"));
        assert!(result.contains("- serde::Serialize"));
        assert!(result.contains("- serde::Deserialize"));
        assert!(result.contains("- serde::Value"));
        assert!(result.contains("- serde::VERSION"));
    }

    #[test]
    fn format_all_md_usage_instructions_fallback_no_sections() {
        let content = "# List of all items\n\nNo items here";
        let result = format_all_md("my_crate", content);
        // When no sections with items exist, fall back to generic example
        assert!(result.contains("cargo txt show my_crate::SomeItem"));
        // Should NOT contain any section-specific items
        assert!(!result.contains("cargo txt show my_crate::Error"));
        assert!(!result.contains("cargo txt show my_crate::Struct"));
    }

    ///////////////////////////////////////////////////////////////////////////
    // extract_item_mappings tests

    #[test]
    fn extract_item_mappings_simple() {
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

        let mappings = extract_item_mappings("serde", html).unwrap();
        assert_eq!(mappings.len(), 2);
        assert_eq!(
            mappings.get("serde::Error"),
            Some(&"struct.Error.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::Config"),
            Some(&"struct.Config.html".to_string())
        );
    }

    #[test]
    fn extract_item_mappings_nested_paths() {
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

        let mappings = extract_item_mappings("serde", html).unwrap();
        assert_eq!(mappings.len(), 3);
        assert_eq!(
            mappings.get("serde::de::IgnoredAny"),
            Some(&"de/struct.IgnoredAny.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::ser::StdError"),
            Some(&"ser/trait.StdError.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::de::value::Error"),
            Some(&"de/value/struct.Error.html".to_string())
        );
    }

    #[test]
    fn extract_item_mappings_multiple_sections() {
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

        let mappings = extract_item_mappings("serde", html).unwrap();
        assert_eq!(mappings.len(), 4);
        assert_eq!(
            mappings.get("serde::Error"),
            Some(&"struct.Error.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::Serialize"),
            Some(&"trait.Serialize.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::Deserialize"),
            Some(&"trait.Deserialize.html".to_string())
        );
        assert_eq!(
            mappings.get("serde::Value"),
            Some(&"enum.Value.html".to_string())
        );
    }

    #[test]
    fn extract_item_mappings_empty_html() {
        let html = r#"
            <html>
                <body>
                    <p>No items here</p>
                </body>
            </html>
        "#;

        let result = extract_item_mappings("serde", html);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("failed to find item mappings"));
    }

    #[test]
    fn extract_item_mappings_no_links() {
        let html = r#"
            <html>
                <body>
                    <h3 id="structs">Structs</h3>
                    <ul class="all-items">
                        <li>No links here</li>
                    </ul>
                </body>
            </html>
        "#;

        let result = extract_item_mappings("serde", html);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("failed to find item mappings"));
    }

    #[test]
    fn extract_item_mappings_creates_full_paths() {
        let html = r#"
            <html>
                <body>
                    <h3 id="traits">Traits</h3>
                    <ul class="all-items">
                        <li><a href="trait.MyTrait.html">MyTrait</a></li>
                    </ul>
                </body>
            </html>
        "#;

        let mappings = extract_item_mappings("mycrate", html).unwrap();
        assert!(mappings.contains_key("mycrate::MyTrait"));
        assert_eq!(
            mappings.get("mycrate::MyTrait"),
            Some(&"trait.MyTrait.html".to_string())
        );
    }

    ///////////////////////////////////////////////////////////////////////////
    // if_needed tests
    // Note: Full integration tests for if_needed would require:
    // - Setting up a temporary directory structure
    // - Mocking cargo metadata and cargo doc
    // - Creating/clearing all.md files to test both paths
    // These would be better suited as integration tests in tests/
    // The function is tested indirectly through show and list commands
}
