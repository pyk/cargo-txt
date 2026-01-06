//! Index page generator for crate documentation.
//!
//! This module generates the main index page that lists all public items
//! grouped by type with links to their detail pages. The index serves as a
//! navigation hub for browsing the crate documentation.

use crate::error::Result;
use crate::markdown::{SECTION_HEADER_LEVEL, utils};
use rustdoc_types::Crate;
use std::collections::BTreeMap;
use std::path::Path;

/// Generate index markdown file for a crate.
///
/// This function creates an index.md file in the output directory that contains
/// the crate name, documentation, item counts grouped by type, and links to
/// all public items.
pub fn generate_index(krate: &Crate, output_dir: &Path) -> Result<()> {
    // Get the root module for crate-level documentation
    let root_item = match krate.index.get(&krate.root) {
        Some(item) => item,
        None => return Ok(()),
    };

    // Get crate name from root item
    let crate_name = root_item.name.as_deref().unwrap_or("Unknown");

    // Group all items by their type
    let items_by_type = group_items_by_type(krate);

    // Build the index content
    let mut content = String::new();

    // Add crate title and documentation
    content.push_str(&utils::render_header(SECTION_HEADER_LEVEL, crate_name));
    content.push_str("\n\n");

    let crate_docs = utils::render_documentation(&root_item.docs);
    if !crate_docs.is_empty() {
        content.push_str(&crate_docs);
        content.push_str("\n\n");
    }

    // Add item counts
    content.push_str(&render_item_counts(&items_by_type));
    content.push_str("\n\n");

    // Add item lists
    content.push_str(&render_item_lists(&items_by_type));
    content.push_str("\n\n");

    // Add next actions section
    let next_actions = vec![format!(
        "View source: `cargo docmd browse --crate {}`",
        crate_name
    )];
    content.push_str(&utils::render_next_actions_section(&next_actions));

    // Write the index file
    let index_path = output_dir.join("index.md");
    utils::write_markdown_file(&index_path, &content)?;

    Ok(())
}

/// Group all items in the crate by their type.
///
/// This function organizes items for the index page, sorting them within each
/// group alphabetically by their name.
fn group_items_by_type(krate: &Crate) -> BTreeMap<String, Vec<String>> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (item_id, item) in &krate.index {
        // Skip the root module (crate root)
        if *item_id == krate.root {
            continue;
        }

        // Skip items without names (like impl blocks)
        let item_name = match &item.name {
            Some(name) => name.clone(),
            None => continue,
        };

        // Skip private items - only Public is considered public for documentation
        let is_public = matches!(item.visibility, rustdoc_types::Visibility::Public);
        if !is_public {
            continue;
        }

        // Get the type name as a string
        let type_name = utils::get_item_type_name(&item.inner).to_string();

        // Add to the appropriate group
        grouped
            .entry(type_name)
            .or_insert_with(Vec::new)
            .push(item_name);
    }

    // Sort items within each group
    for items in grouped.values_mut() {
        items.sort();
    }

    grouped
}

/// Render item counts as a markdown section.
fn render_item_counts(items_by_type: &BTreeMap<String, Vec<String>>) -> String {
    let mut result = String::new();

    result.push_str("## Item Counts\n\n");

    if items_by_type.is_empty() {
        result.push_str("No public items found.\n");
        return result;
    }

    let total_count: usize = items_by_type.values().map(|items| items.len()).sum();
    result.push_str(&format!("**Total**: {} public items\n\n", total_count));

    for (type_name, items) in items_by_type {
        result.push_str(&format!("- **{}**: {}\n", type_name, items.len()));
    }

    result
}

/// Render item lists with links to detail pages.
fn render_item_lists(items_by_type: &BTreeMap<String, Vec<String>>) -> String {
    let mut result = String::new();

    for (type_name, items) in items_by_type {
        if items.is_empty() {
            continue;
        }

        result.push_str(&utils::render_header(SECTION_HEADER_LEVEL + 1, type_name));
        result.push_str("\n\n");

        for item_name in items {
            let filename = utils::generate_filename(item_name);
            result.push_str(&format!("- [{}]({})\n", item_name, filename));
        }

        result.push('\n');
    }

    result
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{Crate, Id, Item, ItemEnum, Target, Visibility};
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn create_test_crate() -> Crate {
        let mut index = HashMap::new();

        // Add root module
        index.insert(
            Id(0),
            Item {
                id: Id(0),
                crate_id: 0,
                name: Some("test_crate".to_string()),
                visibility: Visibility::Public,
                inner: ItemEnum::Module(rustdoc_types::Module {
                    is_crate: false,
                    items: Vec::new(),
                    is_stripped: false,
                }),
                docs: Some("Test crate documentation".to_string()),
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        // Add a struct
        index.insert(
            Id(1),
            Item {
                id: Id(1),
                crate_id: 0,
                name: Some("MyStruct".to_string()),
                visibility: Visibility::Public,
                inner: ItemEnum::Struct(rustdoc_types::Struct {
                    kind: rustdoc_types::StructKind::Plain {
                        fields: Vec::new(),
                        has_stripped_fields: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: Vec::new(),
                        where_predicates: Vec::new(),
                    },
                    impls: Vec::new(),
                }),
                docs: Some("A test struct".to_string()),
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        // Add a function
        index.insert(
            Id(2),
            Item {
                id: Id(2),
                crate_id: 0,
                name: Some("my_function".to_string()),
                visibility: Visibility::Public,
                inner: ItemEnum::Function(rustdoc_types::Function {
                    sig: rustdoc_types::FunctionSignature {
                        inputs: Vec::new(),
                        output: Some(rustdoc_types::Type::Generic("String".to_string())),
                        is_c_variadic: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: Vec::new(),
                        where_predicates: Vec::new(),
                    },
                    header: rustdoc_types::FunctionHeader {
                        is_const: false,
                        is_async: false,
                        is_unsafe: false,
                        abi: rustdoc_types::Abi::Rust,
                    },
                    has_body: false,
                }),
                docs: None,
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        // Add a private item (should be excluded)
        index.insert(
            Id(3),
            Item {
                id: Id(3),
                crate_id: 0,
                name: Some("PrivateStruct".to_string()),
                visibility: Visibility::Default,
                inner: ItemEnum::Struct(rustdoc_types::Struct {
                    kind: rustdoc_types::StructKind::Plain {
                        fields: Vec::new(),
                        has_stripped_fields: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: Vec::new(),
                        where_predicates: Vec::new(),
                    },
                    impls: Vec::new(),
                }),
                docs: None,
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        Crate {
            root: Id(0),
            crate_version: Some("0.1.0".to_string()),
            includes_private: false,
            index,
            paths: HashMap::new(),
            external_crates: HashMap::new(),
            format_version: 0,
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_string(),
                target_features: Vec::new(),
            },
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Grouping Tests

    #[test]
    fn grouping_includes_public_only() {
        let krate = create_test_crate();
        let grouped = group_items_by_type(&krate);

        // Should have 2 types: Struct and Function
        assert_eq!(grouped.len(), 2);
        assert!(grouped.contains_key("Struct"));
        assert!(grouped.contains_key("Function"));

        // Struct group should only have MyStruct (not PrivateStruct)
        let structs = grouped.get("Struct").unwrap();
        assert_eq!(structs.len(), 1);
        assert!(structs.contains(&"MyStruct".to_string()));

        // Function group should have my_function
        let functions = grouped.get("Function").unwrap();
        assert_eq!(functions.len(), 1);
        assert!(functions.contains(&"my_function".to_string()));
    }

    #[test]
    fn grouping_sorts_items() {
        let mut krate = create_test_crate();

        // Add more structs in reverse alphabetical order
        krate.index.insert(
            Id(4),
            Item {
                id: Id(4),
                crate_id: 0,
                name: Some("ZStruct".to_string()),
                visibility: Visibility::Public,
                inner: ItemEnum::Struct(rustdoc_types::Struct {
                    kind: rustdoc_types::StructKind::Plain {
                        fields: Vec::new(),
                        has_stripped_fields: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: Vec::new(),
                        where_predicates: Vec::new(),
                    },
                    impls: Vec::new(),
                }),
                docs: None,
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        krate.index.insert(
            Id(5),
            Item {
                id: Id(5),
                crate_id: 0,
                name: Some("AStruct".to_string()),
                visibility: Visibility::Public,
                inner: ItemEnum::Struct(rustdoc_types::Struct {
                    kind: rustdoc_types::StructKind::Plain {
                        fields: Vec::new(),
                        has_stripped_fields: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: Vec::new(),
                        where_predicates: Vec::new(),
                    },
                    impls: Vec::new(),
                }),
                docs: None,
                attrs: Vec::new(),
                span: None,
                links: HashMap::new(),
                deprecation: None,
            },
        );

        let grouped = group_items_by_type(&krate);
        let structs = grouped.get("Struct").unwrap();

        // Should be sorted alphabetically
        assert_eq!(
            structs,
            &vec![
                "AStruct".to_string(),
                "MyStruct".to_string(),
                "ZStruct".to_string()
            ]
        );
    }

    /////////////////////////////////////////////////////////////////////////////
    // Index Generation Tests

    #[test]
    fn generation_creates_file() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        assert!(index_path.exists());
    }

    #[test]
    fn generation_includes_crate_name() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        let content = std::fs::read_to_string(&index_path).unwrap();

        assert!(content.contains("## test_crate"));
    }

    #[test]
    fn generation_includes_documentation() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        let content = std::fs::read_to_string(&index_path).unwrap();

        assert!(content.contains("Test crate documentation"));
    }

    #[test]
    fn generation_includes_item_counts() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        let content = std::fs::read_to_string(&index_path).unwrap();

        assert!(content.contains("## Item Counts"));
        assert!(content.contains("**Total**: 2 public items"));
        assert!(content.contains("- **Struct**: 1"));
        assert!(content.contains("- **Function**: 1"));
    }

    #[test]
    fn generation_includes_item_links() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        let content = std::fs::read_to_string(&index_path).unwrap();

        assert!(content.contains("[MyStruct](MyStruct.md)"));
        assert!(content.contains("[my_function](my_function.md)"));
    }

    #[test]
    fn generation_includes_next_actions() {
        let krate = create_test_crate();
        let temp_dir = tempdir().unwrap();

        generate_index(&krate, temp_dir.path()).unwrap();

        let index_path = temp_dir.path().join("index.md");
        let content = std::fs::read_to_string(&index_path).unwrap();

        assert!(content.contains("## Next Actions"));
        assert!(content.contains("`cargo docmd browse --crate test_crate`"));
    }
}
