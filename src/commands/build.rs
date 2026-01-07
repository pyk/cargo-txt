//! Build command implementation.
//!
//! This module handles building documentation by executing cargo doc,
//! parsing the generated HTML files, and generating markdown documentation.

use crate::cargo;
use crate::error;
use crate::items;

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc,
/// parses the HTML files for type aliases, and generates markdown documentation.
pub fn build(crate_name: String) -> error::Result<()> {
    // Get cargo metadata and validate the crate
    let metadata = cargo::metadata()?;

    // Validate that the requested crate is an installed dependency
    let is_valid = metadata.packages[0]
        .dependencies
        .iter()
        .any(|dep| dep.name == crate_name);

    if !is_valid {
        return Err(error::BuildError::InvalidCrateName {
            requested: crate_name,
            available: metadata.packages[0]
                .dependencies
                .iter()
                .map(|dep| dep.name.clone())
                .collect(),
        }
        .into());
    }

    cargo::doc(&crate_name, &metadata.target_directory)?;

    let html_dir = get_html_dir(&crate_name, &metadata.target_directory)?;
    let output_dir = get_output_dir(&metadata.target_directory)?;

    create_output_directory(&output_dir)?;

    let type_alias_count = parse_html_directory(&html_dir, &output_dir)?;

    if type_alias_count > 0 {
        println!(
            "\nGenerated markdown documentation for {} type alias(es)",
            type_alias_count
        );
    } else {
        println!("\nNo type aliases found in documentation");
    }

    println!("Output directory: {}", output_dir.display());

    Ok(())
}

/// Get the HTML documentation directory for a crate.
///
/// This function constructs the path to the HTML documentation directory
/// based on the crate name and target directory configuration.
fn get_html_dir(crate_name: &str, target_dir: &str) -> error::Result<std::path::PathBuf> {
    let target_path = std::path::PathBuf::from(target_dir);
    let html_dir = target_path.join("doc").join(crate_name);

    if !html_dir.exists() {
        return Err(error::BuildError::DocNotGenerated {
            crate_name: crate_name.to_string(),
            expected_path: html_dir,
        }
        .into());
    }

    Ok(html_dir)
}

/// Get the output directory for markdown documentation.
///
/// This function reads the target directory from cargo metadata
/// and returns a path to the `docmd` subdirectory within it.
fn get_output_dir(target_dir: &str) -> error::Result<std::path::PathBuf> {
    let target_path = std::path::PathBuf::from(target_dir);
    let output_dir = target_path.join("docmd");

    Ok(output_dir)
}

/// Create the output directory if it doesn't exist.
///
/// This function creates the directory and all parent directories if they don't exist.
fn create_output_directory(path: &std::path::Path) -> error::Result<()> {
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

/// Parse HTML files and generate markdown documentation.
///
/// This function iterates through HTML files in the documentation directory,
/// identifies type aliases (files matching `type.*.html`), parses them using
/// the TypeAlias struct, and generates markdown documentation.
/// Recursively collect all HTML files from a directory.
///
/// This function walks through the directory tree starting at the given path
/// and collects all HTML file paths, handling subdirectories recursively.
fn collect_html_files_recursive(dir: &std::path::Path) -> error::Result<Vec<std::path::PathBuf>> {
    let mut html_files = Vec::new();
    let entries = error::wrap_with_path(std::fs::read_dir(dir), dir)?;

    for entry in entries {
        let Ok(entry) = entry else { continue };

        let path = entry.path();

        if path.is_dir() {
            // Recursively search subdirectories
            let sub_files = collect_html_files_recursive(&path)?;
            html_files.extend(sub_files);
        } else if path.is_file() {
            let is_html = path.extension().and_then(|e| e.to_str()) == Some("html");
            if is_html {
                html_files.push(path);
            }
        }
    }

    Ok(html_files)
}

/// Parse HTML files and generate markdown documentation.
///
/// This function recursively searches the HTML documentation directory for HTML files,
/// identifies type aliases (files matching `type.*.html`), parses them using
/// the TypeAlias struct, and generates markdown documentation.
fn parse_html_directory(
    html_dir: &std::path::Path,
    output_dir: &std::path::Path,
) -> error::Result<usize> {
    let html_files = collect_html_files_recursive(html_dir)?;

    let mut type_alias_count = 0;

    for path in html_files {
        // Process only type alias files (type.*.html)
        let file_name = path.file_name().and_then(|n| n.to_str());
        let is_type_alias = file_name
            .map(|name| name.starts_with("type.") && name.ends_with(".html"))
            .unwrap_or(false);

        if !is_type_alias {
            continue;
        }

        // Parse the HTML file
        let html_content = error::wrap_with_path(std::fs::read_to_string(&path), &path)?;

        let type_alias =
            error::wrap_with_path(items::type_alias::TypeAlias::from_str(&html_content), &path)?;

        // Generate markdown
        let markdown_content = type_alias.markdown();

        // Write markdown to output file
        let markdown_path = output_dir.join(format!("{}.md", type_alias.name));

        std::fs::write(&markdown_path, markdown_content).map_err(|error| {
            error::BuildError::markdown_write_failed(&markdown_path, error.to_string())
        })?;

        type_alias_count += 1;
    }

    Ok(type_alias_count)
}
