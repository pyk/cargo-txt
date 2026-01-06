//! Common utilities for markdown generation.
//!
//! This module provides shared functions for formatting markdown, generating
//! filenames, and writing files. These utilities are used across all item-type
//! generators to maintain consistency in the output.

use crate::error::{MarkdownError, Result};
use rustdoc_types::ItemEnum;
use std::fs;
use std::path::Path;

/// Convert an ItemEnum to a human-readable type name.
///
/// This function converts rustdoc_types::ItemEnum variants into their
/// human-readable string names for display purposes.
pub fn get_item_type_name(inner: &ItemEnum) -> &'static str {
    match inner {
        ItemEnum::Module(_) => "Module",
        ItemEnum::ExternCrate { .. } => "Extern Crate",
        ItemEnum::Use(_) => "Use Statement",
        ItemEnum::Union(_) => "Union",
        ItemEnum::Struct(_) => "Struct",
        ItemEnum::StructField(_) => "Struct Field",
        ItemEnum::Enum(_) => "Enum",
        ItemEnum::Variant(_) => "Variant",
        ItemEnum::Function(_) => "Function",
        ItemEnum::Trait(_) => "Trait",
        ItemEnum::TraitAlias(_) => "Trait Alias",
        ItemEnum::Impl(_) => "Impl Block",
        ItemEnum::TypeAlias(_) => "Type Alias",
        ItemEnum::Constant { .. } => "Constant",
        ItemEnum::Static(_) => "Static",
        ItemEnum::ExternType => "Extern Type",
        ItemEnum::Macro(_) => "Macro",
        ItemEnum::ProcMacro(_) => "Proc Macro",
        ItemEnum::Primitive(_) => "Primitive",
        ItemEnum::AssocConst { .. } => "Associated Constant",
        ItemEnum::AssocType { .. } => "Associated Type",
    }
}

/// Generate a markdown filename from an item ID.
///
/// This function converts a rustdoc item ID into a deterministic, filesystem-safe
/// filename by replacing `::` with `-`, removing generic parameters, and adding
/// the `.md` extension.
///
/// # Examples
///
/// ```ignore
/// generate_filename("std::vec::Vec") -> "std-vec-Vec.md"
/// generate_filename("std::collections::HashMap<K, V>") -> "std-collections-HashMap.md"
/// ```
pub fn generate_filename(item_id: &str) -> String {
    // Find and remove generic parameters if present
    let base_name = if let Some(generic_start) = item_id.find('<') {
        &item_id[..generic_start]
    } else {
        item_id
    };

    // Replace :: with - and add .md extension
    let filename = base_name.replace("::", "-");
    format!("{}.md", filename)
}

/// Render a markdown header at the specified level.
///
/// This function generates a markdown header with the appropriate number of
/// hash characters based on the level.
pub fn render_header(level: usize, text: &str) -> String {
    let hashes = "#".repeat(level);
    format!("{} {}", hashes, text)
}

/// Render a markdown code block with optional language specification.
///
/// This function wraps the provided content in triple backticks and optionally
/// specifies a language for syntax highlighting.
pub fn render_code_block(content: &str, language: Option<&str>) -> String {
    let lang_spec = language.unwrap_or("");
    format!("```{}\n{}\n```", lang_spec, content)
}

/// Render inline code in markdown.
///
/// This function wraps the text in single backticks for inline code formatting.
pub fn render_inline_code(text: &str) -> String {
    format!("`{}`", text)
}

/// Render documentation text from rustdoc to markdown format.
///
/// This function converts rustdoc documentation strings into markdown format,
/// stripping leading `///` markers and handling empty or missing documentation.
pub fn render_documentation(docs: &Option<String>) -> String {
    let docs_string = match docs {
        Some(text) => text.clone(),
        None => return String::new(),
    };

    if docs_string.is_empty() {
        return String::new();
    }

    // Strip leading /// from each line
    docs_string
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("///") {
                trimmed[3..].trim_start()
            } else if trimmed.starts_with("//") {
                trimmed[2..].trim_start()
            } else {
                trimmed
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render a "Next Actions" section with the provided actions.
///
/// This function generates a standardized "Next Actions" section with a header
/// and a bulleted list of actions.
pub fn render_next_actions_section(actions: &[String]) -> String {
    if actions.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    result.push_str("## Next Actions\n\n");

    for action in actions {
        result.push_str("- ");
        result.push_str(action);
        result.push('\n');
    }

    result
}

/// Write markdown content to a file, creating parent directories as needed.
///
/// This function ensures the parent directory exists before writing the file,
/// providing clear error messages with full paths on failure.
pub fn write_markdown_file(path: &Path, content: &str) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_directory_exists(parent)?;
    }

    // Write the file
    fs::write(path, content)
        .map_err(|error| MarkdownError::FileWriteFailed(path.to_path_buf(), error.to_string()))?;

    Ok(())
}

/// Ensure a directory exists, creating it recursively if necessary.
///
/// This function creates the directory and any missing parent directories,
/// handling the case where the directory already exists gracefully.
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    // Check if directory already exists
    if path.exists() {
        return Ok(());
    }

    // Create directory recursively
    fs::create_dir_all(path).map_err(|error| {
        MarkdownError::DirectoryCreationFailed(path.to_path_buf(), error.to_string())
    })?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use rustdoc_types::ItemEnum;
    use std::fs;
    use tempfile::tempdir;

    /////////////////////////////////////////////////////////////////////////////
    // Filename Generation Tests

    #[test]
    fn filename_simple_path() {
        let result = generate_filename("std::vec::Vec");
        assert_eq!(result, "std-vec-Vec.md");
    }

    #[test]
    fn filename_with_generics() {
        let result = generate_filename("std::collections::HashMap<K, V>");
        assert_eq!(result, "std-collections-HashMap.md");
    }

    #[test]
    fn filename_deeply_nested() {
        let result = generate_filename("serde::de::Deserialize");
        assert_eq!(result, "serde-de-Deserialize.md");
    }

    #[test]
    fn filename_method() {
        let result = generate_filename("serde::Serialize::serialize");
        assert_eq!(result, "serde-Serialize-serialize.md");
    }

    #[test]
    fn filename_single_segment() {
        let result = generate_filename("MyStruct");
        assert_eq!(result, "MyStruct.md");
    }

    #[test]
    fn filename_complex_generics() {
        let result = generate_filename("std::result::Result<T, E>");
        assert_eq!(result, "std-result-Result.md");
    }

    /////////////////////////////////////////////////////////////////////////////
    // Rendering Tests

    #[test]
    fn render_header_level_one() {
        let result = render_header(1, "Title");
        assert_eq!(result, "# Title");
    }

    #[test]
    fn render_header_level_two() {
        let result = render_header(2, "Section");
        assert_eq!(result, "## Section");
    }

    #[test]
    fn render_header_level_three() {
        let result = render_header(3, "Subsection");
        assert_eq!(result, "### Subsection");
    }

    #[test]
    fn render_code_block_with_language() {
        let result = render_code_block("let x = 42;", Some("rust"));
        assert_eq!(result, "```rust\nlet x = 42;\n```");
    }

    #[test]
    fn render_code_block_without_language() {
        let result = render_code_block("some text", None);
        assert_eq!(result, "```\nsome text\n```");
    }

    #[test]
    fn render_code_block_multiline() {
        let result = render_code_block("line1\nline2\nline3", Some("rust"));
        assert_eq!(result, "```rust\nline1\nline2\nline3\n```");
    }

    #[test]
    fn render_inline_code_wraps_text() {
        let result = render_inline_code("Vec<T>");
        assert_eq!(result, "`Vec<T>`");
    }

    #[test]
    fn render_documentation_with_triple_slash() {
        let docs = Some("/// This is documentation.\n/// Second line.".to_string());
        let result = render_documentation(&docs);
        assert_eq!(result, "This is documentation.\nSecond line.");
    }

    #[test]
    fn render_documentation_with_double_slash() {
        let docs = Some("// Single slash comment\n// Another line".to_string());
        let result = render_documentation(&docs);
        assert_eq!(result, "Single slash comment\nAnother line");
    }

    #[test]
    fn render_documentation_none() {
        let docs: Option<String> = None;
        let result = render_documentation(&docs);
        assert!(result.is_empty());
    }

    #[test]
    fn render_documentation_empty_string() {
        let docs = Some(String::new());
        let result = render_documentation(&docs);
        assert!(result.is_empty());
    }

    #[test]
    fn render_documentation_no_markers() {
        let docs = Some("Plain documentation text".to_string());
        let result = render_documentation(&docs);
        assert_eq!(result, "Plain documentation text");
    }

    #[test]
    fn render_next_actions_single() {
        let actions = vec!["Action one".to_string()];
        let result = render_next_actions_section(&actions);
        assert_eq!(result, "## Next Actions\n\n- Action one\n");
    }

    #[test]
    fn render_next_actions_multiple() {
        let actions = vec![
            "First action".to_string(),
            "Second action".to_string(),
            "Third action".to_string(),
        ];
        let result = render_next_actions_section(&actions);
        assert_eq!(
            result,
            "## Next Actions\n\n- First action\n- Second action\n- Third action\n"
        );
    }

    #[test]
    fn render_next_actions_empty() {
        let actions: Vec<String> = vec![];
        let result = render_next_actions_section(&actions);
        assert!(result.is_empty());
    }

    /////////////////////////////////////////////////////////////////////////////
    // Error Handling Tests

    #[test]
    fn error_handling_write_file_failed_includes_path() {
        let temp_dir = tempdir().unwrap();
        let _file_path = temp_dir.path().join("test.md");

        // Try to write to a directory instead of a file
        let result = write_markdown_file(&temp_dir.path(), "content");

        assert!(result.is_err());
        match result {
            Err(Error::Markdown(MarkdownError::FileWriteFailed(path, _))) => {
                assert_eq!(path, temp_dir.path());
            }
            _ => panic!("Expected FileWriteFailed error"),
        }
    }

    #[test]
    fn error_handling_directory_creation_failed_includes_path() {
        // Try to create a directory in /dev/null which will fail
        let invalid_path = Path::new("/dev/null/test_dir");

        let result = ensure_directory_exists(invalid_path);

        assert!(result.is_err());
        match result {
            Err(Error::Markdown(MarkdownError::DirectoryCreationFailed(path, _))) => {
                assert_eq!(path, invalid_path);
            }
            _ => panic!("Expected DirectoryCreationFailed error"),
        }
    }

    #[test]
    fn error_handling_write_file_creates_parent_directories() {
        let temp_dir = tempdir().unwrap();
        let nested_path = temp_dir.path().join("level1/level2/test.md");

        write_markdown_file(&nested_path, "test content").unwrap();

        assert!(nested_path.exists());
        let content = fs::read_to_string(&nested_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn error_handling_ensure_directory_handles_existing() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");

        // Create directory
        fs::create_dir(&dir_path).unwrap();

        // Should succeed without error
        ensure_directory_exists(&dir_path).unwrap();

        assert!(dir_path.exists());
    }

    #[test]
    fn error_handling_write_file_success() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("success.md");

        write_markdown_file(&file_path, "successful content").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "successful content");
    }
}
