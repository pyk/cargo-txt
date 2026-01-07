//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! converting the generated HTML to markdown, and writing the result.

use log::{debug, info};

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

    let html_content = std::fs::read_to_string(&html_path).map_err(|error| {
        error::BuildError::HtmlParseFailed {
            path: html_path.clone(),
            source: Box::new(error),
        }
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
                error: error.to_string(),
            }
        })?;
    }

    // Write markdown to index.md
    let index_path = output_dir.join("index.md");
    debug!("Writing markdown to: {:?}", index_path);
    std::fs::write(&index_path, markdown_content).map_err(|error| {
        error::BuildError::MarkdownWriteFailed {
            path: index_path.clone(),
            error: error.to_string(),
        }
    })?;

    info!("Successfully generated markdown");
    println!("Generated markdown: {}", index_path.display());

    Ok(())
}
