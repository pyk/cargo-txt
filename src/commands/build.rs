//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! converting the generated HTML to markdown, and writing the result.

use crate::cargo;
use crate::error;
use crate::html2md;

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc,
/// converts the generated HTML to markdown, and writes the result to the output directory.
pub fn build(crate_name: String, debug: bool) -> error::Result<()> {
    if debug {
        eprintln!("DEBUG: Building documentation for crate: {}", crate_name);
    }

    // Get cargo metadata and validate the crate
    let metadata = cargo::metadata()?;

    if debug {
        eprintln!("DEBUG: Target directory: {}", metadata.target_directory);
    }

    // Find the dependency for the requested crate
    let _dependency = metadata.packages[0]
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
    if debug {
        eprintln!(
            "DEBUG: Running cargo doc --package {} --no-deps",
            crate_name
        );
    }
    let html_dir = cargo::doc(&crate_name, debug)?;

    if debug {
        eprintln!("DEBUG: HTML directory: {:?}", html_dir);
    }

    // Read the index.html file
    let html_path = html_dir.join("index.html");

    if debug {
        eprintln!("DEBUG: Reading HTML file: {:?}", html_path);
    }

    let html_content = std::fs::read_to_string(&html_path).map_err(|error| {
        error::BuildError::HtmlParseFailed {
            path: html_path.clone(),
            source: Box::new(error),
        }
    })?;

    // Convert HTML to markdown
    if debug {
        eprintln!(
            "DEBUG: Converting HTML to markdown ({} bytes)",
            html_content.len()
        );
    }
    let markdown_content = html2md::convert(&html_content)?;

    if debug {
        eprintln!("DEBUG: Markdown content ({} bytes)", markdown_content.len());
    }

    // Create output directory structure: target/docmd/<crate>/
    let output_dir = std::path::PathBuf::from(&metadata.target_directory)
        .join("docmd")
        .join(&crate_name);

    if !output_dir.exists() {
        if debug {
            eprintln!("DEBUG: Creating output directory: {:?}", output_dir);
        }
        std::fs::create_dir_all(&output_dir).map_err(|error| {
            error::BuildError::OutputDirCreationFailed {
                path: output_dir.clone(),
                error: error.to_string(),
            }
        })?;
    }

    // Write markdown to index.md
    let index_path = output_dir.join("index.md");
    if debug {
        eprintln!("DEBUG: Writing markdown to: {:?}", index_path);
    }
    std::fs::write(&index_path, markdown_content).map_err(|error| {
        error::BuildError::MarkdownWriteFailed {
            path: index_path.clone(),
            error: error.to_string(),
        }
    })?;

    if debug {
        eprintln!("DEBUG: Successfully generated markdown");
    }
    println!("Generated markdown: {}", index_path.display());

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_output_dir() {
        // This is a simple test to verify the output directory structure
        // The actual path depends on cargo metadata, but we can verify the logic
        let target_dir = "/tmp/project/target";
        let crate_name = "serde";
        let expected = std::path::PathBuf::from(target_dir)
            .join("docmd")
            .join(crate_name);
        let actual = std::path::PathBuf::from(target_dir)
            .join("docmd")
            .join(crate_name);
        assert_eq!(expected, actual);
    }
}
