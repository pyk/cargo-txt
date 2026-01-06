//! Union generator for markdown documentation.
//!
//! This module handles generating markdown files for Rust union items.
//! Unions require special attention to safety considerations since they allow
//! unsafe access to their fields. The generator extracts field information,
//! visibility, and documentation to create comprehensive documentation for
//! coding agents.

use rustdoc_types::{Crate, Id, Item, ItemEnum, Union};
use std::collections::HashMap;
use std::path::Path;

use crate::error;
use crate::markdown;

/// Generate markdown documentation for a union item.
///
/// This function extracts union data from the provided item, generates
/// markdown content including fields, safety notes, and documentation, and writes
/// it to the output directory.
pub fn generate(krate: &Crate, item: &Item, output_dir: &Path) -> error::Result<()> {
    let union_data = extract_union_data(&item.inner)?;
    let item_map = &krate.index;

    let content = generate_union_content(item, union_data, item_map);
    let filename = markdown::utils::generate_filename(&format!("{}", item.id.0));
    let output_path = output_dir.join(&filename);

    markdown::utils::write_markdown_file(&output_path, &content)?;

    Ok(())
}

/// Extract union data from an ItemEnum variant.
///
/// This function unwraps the ItemEnum to get the Union data, returning
/// an error if the item is not a union.
fn extract_union_data(inner: &ItemEnum) -> error::Result<&Union> {
    match inner {
        ItemEnum::Union(union_data) => Ok(union_data),
        _ => Err(error::MarkdownError::ItemNotFound(format!(
            "Expected union item, found {:?}",
            inner
        ))
        .into()),
    }
}

/// Generate the complete markdown content for a union.
///
/// This function assembles all sections of the union documentation including
/// the header, description, safety note, fields, generics, and next actions.
fn generate_union_content(item: &Item, union_data: &Union, item_map: &HashMap<Id, Item>) -> String {
    let mut content = String::new();

    let name = item.name.as_ref().map_or("Anonymous", String::as_str);
    content.push_str(&markdown::utils::render_header(
        markdown::ITEM_HEADER_LEVEL,
        name,
    ));
    content.push('\n');

    let docs = markdown::utils::render_documentation(&item.docs);
    if !docs.is_empty() {
        content.push('\n');
        content.push_str(&docs);
        content.push('\n');
    }

    let safety_note = generate_safety_note();
    content.push('\n');
    content.push_str(&safety_note);

    let fields_section = generate_fields_section(&union_data.fields, item_map);
    if !fields_section.is_empty() {
        content.push('\n');
        content.push_str(&fields_section);
    }

    let generics_section = generate_generics_section(&union_data.generics);
    if !generics_section.is_empty() {
        content.push('\n');
        content.push_str(&generics_section);
    }

    let next_actions = generate_next_actions(item);
    if !next_actions.is_empty() {
        content.push('\n');
        content.push_str(&next_actions);
    }

    content
}

/// Generate the safety note for unions.
///
/// This function creates a prominent safety warning about unsafe access to
/// union fields.
fn generate_safety_note() -> String {
    let mut note = String::new();
    note.push_str(&markdown::utils::render_header(
        markdown::SECTION_HEADER_LEVEL,
        "Safety",
    ));
    note.push('\n');
    note.push_str(
        "**Important**: Accessing union fields requires unsafe code. Only access the field that was most recently written to. Reading from a different field results in undefined behavior.",
    );
    note.push('\n');
    note
}

/// Generate the fields section for a union.
///
/// This function renders all union fields with their types and documentation.
fn generate_fields_section(field_ids: &[Id], item_map: &HashMap<Id, Item>) -> String {
    if field_ids.is_empty() {
        return String::new();
    }

    let mut section = String::new();
    section.push_str(&markdown::utils::render_header(
        markdown::SECTION_HEADER_LEVEL,
        "Fields",
    ));
    section.push('\n');

    for field_id in field_ids {
        let field = match item_map.get(field_id) {
            Some(item) => item,
            None => continue,
        };

        let field_data = match &field.inner {
            ItemEnum::StructField(type_) => type_,
            _ => continue,
        };

        let name = field.name.as_ref().map_or("Unnamed", String::as_str);
        let type_str = markdown::utils::render_inline_code(&render_type(field_data));
        let visibility = render_visibility(&field.visibility);

        section.push_str("- ");
        section.push_str(&type_str);
        section.push(' ');
        section.push_str(name);

        if !visibility.is_empty() {
            section.push(' ');
            section.push_str(&visibility);
        }

        let field_docs = markdown::utils::render_documentation(&field.docs);
        if !field_docs.is_empty() {
            section.push_str(" - ");
            section.push_str(&field_docs);
        }

        section.push('\n');
    }

    section
}

/// Render visibility for a union field.
///
/// This function generates visibility text for fields, returning empty string
/// for non-public fields.
fn render_visibility(visibility: &rustdoc_types::Visibility) -> String {
    match visibility {
        rustdoc_types::Visibility::Public => "(pub)".to_string(),
        rustdoc_types::Visibility::Default => String::new(),
        rustdoc_types::Visibility::Crate => "(pub(crate))".to_string(),
        rustdoc_types::Visibility::Restricted { .. } => "(pub restricted)".to_string(),
    }
}

/// Render type for a field.
///
/// This function converts the rustdoc Type enum to a string representation.
fn render_type(type_: &rustdoc_types::Type) -> String {
    match type_ {
        rustdoc_types::Type::ResolvedPath(path) => path.path.clone(),
        rustdoc_types::Type::Primitive(name) => name.clone(),
        rustdoc_types::Type::Generic(name) => name.clone(),
        rustdoc_types::Type::Tuple(types) => {
            let types_str: Vec<String> = types.iter().map(render_type).collect();
            format!("({})", types_str.join(", "))
        }
        rustdoc_types::Type::Slice(inner_type) => {
            format!("[{}]", render_type(inner_type))
        }
        rustdoc_types::Type::Array { type_, len } => {
            format!("[{}; {}]", render_type(type_), len)
        }
        rustdoc_types::Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{} {}", mutability, render_type(type_))
        }
        rustdoc_types::Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mutability = if *is_mutable { "mut " } else { "" };
            let lifetime_str = lifetime
                .as_ref()
                .map_or_else(String::new, |l| format!("'{} ", l));
            format!("&{}{}{}", lifetime_str, mutability, render_type(type_))
        }
        rustdoc_types::Type::FunctionPointer(_) => "fn(...)".to_string(),
        rustdoc_types::Type::ImplTrait(_) => "impl Trait".to_string(),
        rustdoc_types::Type::DynTrait(_) => "dyn Trait".to_string(),
        rustdoc_types::Type::Infer => "_".to_string(),
        rustdoc_types::Type::QualifiedPath { .. } => "QualifiedPath".to_string(),
        rustdoc_types::Type::Pat { .. } => "Pattern".to_string(),
    }
}

/// Generate the generics section for a union.
///
/// This function displays generic type parameters if the union has any.
fn generate_generics_section(generics: &rustdoc_types::Generics) -> String {
    if generics.params.is_empty() {
        return String::new();
    }

    let mut section = String::new();
    section.push_str(&markdown::utils::render_header(
        markdown::SECTION_HEADER_LEVEL,
        "Generic Parameters",
    ));
    section.push('\n');

    for param in &generics.params {
        let name = &param.name;
        let kind_str = match &param.kind {
            rustdoc_types::GenericParamDefKind::Lifetime { .. } => "lifetime",
            rustdoc_types::GenericParamDefKind::Type { .. } => "type",
            rustdoc_types::GenericParamDefKind::Const { .. } => "const",
        };
        section.push_str(&format!("- `{}`: {}\n", name, kind_str));
    }

    section
}

/// Generate the next actions section for a union.
///
/// This function provides actionable next steps for exploring the union.
fn generate_next_actions(item: &Item) -> String {
    let actions = vec![
        format!("View source: `cargo docmd browse --item {}`", item.id.0),
        "Find related unions: `cargo docmd browse --type union`".to_string(),
    ];

    markdown::utils::render_next_actions_section(&actions)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{GenericParamDef, GenericParamDefKind, Visibility};
    use std::collections::HashMap;

    fn create_test_item(name: &str, docs: Option<&str>) -> Item {
        Item {
            id: Id(0),
            crate_id: 0,
            name: Some(name.to_string()),
            span: None,
            visibility: Visibility::Public,
            docs: docs.map(String::from),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: ItemEnum::Union(Union {
                generics: rustdoc_types::Generics {
                    params: vec![],
                    where_predicates: vec![],
                },
                fields: vec![],
                has_stripped_fields: false,
                impls: vec![],
            }),
        }
    }

    fn create_union_field(type_name: &str) -> rustdoc_types::Type {
        rustdoc_types::Type::ResolvedPath(rustdoc_types::Path {
            path: type_name.to_string(),
            id: rustdoc_types::Id(0),
            args: None,
        })
    }

    /////////////////////////////////////////////////////////////////////////////
    // Safety Note Tests

    #[test]
    fn generate_safety_note_includes_warning() {
        let result = generate_safety_note();
        assert!(result.contains("Safety"));
        assert!(result.contains("unsafe code"));
        assert!(result.contains("undefined behavior"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Field Rendering Tests

    #[test]
    fn generate_fields_section_with_public_field() {
        let field_id = Id(1);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            Item {
                id: field_id,
                crate_id: 0,
                name: Some("integer".to_string()),
                span: None,
                visibility: Visibility::Public,
                docs: None,
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_union_field("i64")),
            },
        );

        let result = generate_fields_section(&[field_id], &item_map);
        assert!(result.contains("`i64`"));
        assert!(result.contains("integer"));
        assert!(result.contains("(pub)"));
    }

    #[test]
    fn generate_fields_section_with_private_field() {
        let field_id = Id(2);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            Item {
                id: field_id,
                crate_id: 0,
                name: Some("private_field".to_string()),
                span: None,
                visibility: Visibility::Default,
                docs: Some("A pointer to text data".to_string()),
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_union_field("f64")),
            },
        );

        let result = generate_fields_section(&[field_id], &item_map);
        assert!(result.contains("`f64`"));
        assert!(result.contains("private_field"));
        assert!(!result.contains("(pub)"));
    }

    #[test]
    fn generate_fields_section_with_documentation() {
        let field_id = Id(3);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            Item {
                id: field_id,
                crate_id: 0,
                name: Some("documented".to_string()),
                span: None,
                visibility: Visibility::Public,
                docs: Some("A pointer to text data".to_string()),
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(rustdoc_types::Type::RawPointer {
                    is_mutable: false,
                    type_: Box::new(rustdoc_types::Type::Primitive("u8".to_string())),
                }),
            },
        );

        let result = generate_fields_section(&[field_id], &item_map);
        assert!(result.contains("*const"));
        assert!(result.contains("documented"));
        assert!(result.contains("A pointer to text data"));
    }

    #[test]
    fn generate_fields_section_empty() {
        let result = generate_fields_section(&[], &HashMap::new());
        assert!(result.is_empty());
    }

    /////////////////////////////////////////////////////////////////////////////
    // Type Rendering Tests

    #[test]
    fn render_type_raw_pointer_const() {
        let type_ = rustdoc_types::Type::RawPointer {
            is_mutable: false,
            type_: Box::new(rustdoc_types::Type::Primitive("u8".to_string())),
        };
        let result = render_type(&type_);
        assert_eq!(result, "*const u8");
    }

    #[test]
    fn render_type_raw_pointer_mut() {
        let type_ = rustdoc_types::Type::RawPointer {
            is_mutable: true,
            type_: Box::new(rustdoc_types::Type::Primitive("i32".to_string())),
        };
        let result = render_type(&type_);
        assert_eq!(result, "*mut i32");
    }

    #[test]
    fn render_type_reference_mut() {
        let type_ = rustdoc_types::Type::BorrowedRef {
            lifetime: None,
            is_mutable: true,
            type_: Box::new(rustdoc_types::Type::Primitive("str".to_string())),
        };
        let result = render_type(&type_);
        assert_eq!(result, "&mut str");
    }

    /////////////////////////////////////////////////////////////////////////////
    // Generics Section Tests

    #[test]
    fn generate_generics_section_empty() {
        let generics = rustdoc_types::Generics {
            params: vec![],
            where_predicates: vec![],
        };
        let result = generate_generics_section(&generics);
        assert!(result.is_empty());
    }

    #[test]
    fn generate_generics_section_with_type_param() {
        let generics = rustdoc_types::Generics {
            params: vec![GenericParamDef {
                name: "T".to_string(),
                kind: GenericParamDefKind::Type {
                    bounds: vec![],
                    default: None,
                    is_synthetic: false,
                },
            }],
            where_predicates: vec![],
        };
        let result = generate_generics_section(&generics);
        assert!(result.contains("Generic Parameters"));
        assert!(result.contains("`T`"));
        assert!(result.contains("type"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Next Actions Tests

    #[test]
    fn generate_next_actions_includes_view_source() {
        let item = create_test_item("TestUnion", None);
        let result = generate_next_actions(&item);
        assert!(result.contains("View source:"));
        assert!(result.contains("cargo docmd browse --item"));
    }

    #[test]
    fn generate_next_actions_includes_related() {
        let item = create_test_item("TestUnion", Some(""));
        let result = generate_next_actions(&item);
        assert!(result.contains("Find related unions"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Content Generation Tests

    #[test]
    fn generate_union_content_includes_safety_note() {
        let item = create_test_item("TestUnion", Some("A test union"));
        let union_data = Union {
            generics: rustdoc_types::Generics {
                params: vec![],
                where_predicates: vec![],
            },
            fields: vec![],
            has_stripped_fields: false,
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_union_content(&item, &union_data, &item_map);
        assert!(result.contains("# TestUnion"));
        assert!(result.contains("A test union"));
        assert!(result.contains("Safety"));
        assert!(result.contains("unsafe"));
    }

    #[test]
    fn generate_union_content_with_generics() {
        let item = create_test_item("GenericUnion", None);
        let union_data = Union {
            generics: rustdoc_types::Generics {
                params: vec![GenericParamDef {
                    name: "T".to_string(),
                    kind: GenericParamDefKind::Type {
                        bounds: vec![],
                        default: None,
                        is_synthetic: false,
                    },
                }],
                where_predicates: vec![],
            },
            fields: vec![],
            has_stripped_fields: false,
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_union_content(&item, &union_data, &item_map);
        assert!(result.contains("Generic Parameters"));
        assert!(result.contains("`T`"));
        assert!(result.contains("Safety"));
    }
}
