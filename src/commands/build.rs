//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! converting the generated HTML to markdown, and writing the result.

use log::{debug, info};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cargo;
use crate::error;
use crate::html2md;

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc,
/// converts the generated HTML to markdown, and writes the result to the output directory.
pub fn build(crate_name: String) -> error::Result<()> {
    debug!("Building documentation for crate: {}", crate_name);

    // Get cargo metadata and validate the crate
    let metadata = cargo::metadata()?;

    debug!("Target directory: {}", metadata.target_directory);

    // Find the dependency for the requested crate
    metadata.packages[0]
        .dependencies
        .iter()
        .find(|dep| dep.name == crate_name)
        .ok_or_else(|| error::BuildError::InvalidCrateName {
            requested: crate_name.clone(),
            available: metadata.packages[0]
                .dependencies
                .iter()
                .map(|dep| dep.name.clone())
                .collect(),
        })?;

    // Generate HTML documentation
    info!("Running cargo doc --package {} --no-deps", crate_name);
    let html_dir = cargo::doc(&crate_name)?;

    debug!("HTML directory: {:?}", html_dir);

    // Read the index.html file
    let html_path = html_dir.join("index.html");

    debug!("Reading HTML file: {:?}", html_path);

    let html_content =
        std::fs::read_to_string(&html_path).map_err(|error| error::BuildError::FileReadFailed {
            path: html_path.clone(),
            source: Box::new(error),
        })?;

    // Convert HTML to markdown
    debug!("Converting HTML to markdown ({} bytes)", html_content.len());
    let markdown_content = html2md::convert(&html_content)?;

    debug!("Markdown content ({} bytes)", markdown_content.len());

    // Create output directory structure: target/docmd/<crate>/
    let output_dir = std::path::PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(&crate_name);

    if !output_dir.exists() {
        debug!("Creating output directory: {:?}", output_dir);
        std::fs::create_dir_all(&output_dir).map_err(|error| {
            error::BuildError::OutputDirCreationFailed {
                path: output_dir.clone(),
                source: Box::new(error),
            }
        })?;
    }

    // Write markdown to index.md
    let index_path = output_dir.join("index.md");
    debug!("Writing markdown to: {:?}", index_path);
    std::fs::write(&index_path, markdown_content).map_err(|error| {
        error::BuildError::MarkdownWriteFailed {
            path: index_path.clone(),
            source: Box::new(error),
        }
    })?;

    info!("Successfully generated markdown");
    println!("Generated markdown: {}", index_path.display());

    // Process all.html
    info!("Processing all.html");
    let all_html_path = html_dir.join("all.html");

    let all_html_content = std::fs::read_to_string(&all_html_path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            error::BuildError::DocIndexNotFound {
                path: all_html_path.clone(),
                source: Box::new(error),
            }
        } else {
            error::BuildError::FileReadFailed {
                path: all_html_path.clone(),
                source: Box::new(error),
            }
        }
    })?;

    debug!(
        "Converting all.html to markdown ({} bytes)",
        all_html_content.len()
    );
    let all_markdown_content = html2md::convert(&all_html_content)?;

    let all_path = output_dir.join("all.md");
    debug!("Writing all.md to: {:?}", all_path);
    std::fs::write(&all_path, all_markdown_content).map_err(|error| {
        error::BuildError::MarkdownWriteFailed {
            path: all_path.clone(),
            source: Box::new(error),
        }
    })?;

    info!("Generated all.md");
    println!("Generated markdown: {}", all_path.display());

    // Extract item mappings from all.html (href -> full Rust path)
    info!("Extracting item mappings from all.html");
    let item_mappings = extract_item_mappings(&crate_name, &all_html_content)?;
    debug!("Found {} items to convert", item_mappings.len());

    // Generate markdown for each item
    for html_relative_path in item_mappings.values() {
        let html_path = html_dir.join(html_relative_path);
        let relative_md_path = PathBuf::from(html_relative_path).with_extension("md");
        let md_path = output_dir.join(&relative_md_path);

        debug!("Converting {:?} to {:?}", html_path, relative_md_path);

        let html_content = std::fs::read_to_string(&html_path).map_err(|error| {
            error::BuildError::FileReadFailed {
                path: html_path.to_path_buf(),
                source: Box::new(error),
            }
        })?;

        let markdown_content = html2md::convert(&html_content)?;

        // Create parent directories if needed
        if let Some(parent) = md_path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent).map_err(|error| {
                error::BuildError::OutputDirCreationFailed {
                    path: parent.to_path_buf(),
                    source: Box::new(error),
                }
            })?;
        }

        std::fs::write(&md_path, markdown_content).map_err(|error| {
            error::BuildError::MarkdownWriteFailed {
                path: md_path.to_path_buf(),
                source: Box::new(error),
            }
        })?;
    }

    info!("Generated markdown for {} items", item_mappings.len());

    Ok(())
}

/// Extract item mappings from all.html.
///
/// Parses the all.html file to extract mappings between full Rust paths
/// (e.g., `serde::Error`) and their corresponding HTML file paths
/// (e.g., `struct.Error.html`).
///
/// Returns a HashMap mapping full Rust paths to HTML file paths.
pub fn extract_item_mappings(
    crate_name: &str,
    html: &str,
) -> error::Result<HashMap<String, String>> {
    let mut mappings = HashMap::new();

    let document = Html::parse_document(html);
    let selector = Selector::parse("ul.all-items li a").map_err(|e| {
        error::Error::HtmlSelectorParseFailed {
            selector: "ul.all-items li a".to_string(),
            error: e.to_string(),
        }
    })?;

    for element in document.select(&selector) {
        let href =
            element
                .value()
                .attr("href")
                .ok_or_else(|| error::Error::HtmlElementNotFound {
                    selector: "ul.all-items li a[href]".to_string(),
                })?;

        let text = element.text().collect::<String>();

        // Build full Rust path by prefixing with crate name
        let full_path = format!("{}::{}", crate_name, text);

        mappings.insert(full_path, href.to_string());
    }

    if mappings.is_empty() {
        return Err(error::Error::HtmlElementNotFound {
            selector: "ul.all-items".to_string(),
        });
    }

    Ok(mappings)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

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
}
