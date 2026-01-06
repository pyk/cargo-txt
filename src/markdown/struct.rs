//! Struct generator for markdown documentation.
//!
//! This module handles generating markdown files for Rust struct items,
//! including plain structs, tuple structs, and unit structs. The generator
//! extracts field information, visibility, and documentation to create
//! comprehensive documentation for coding agents.

use rustdoc_types::{Crate, Id, Item, ItemEnum, Struct, StructKind};
use std::path::Path;

use crate::error;
use crate::markdown;

/// Generate markdown documentation for a struct item.
///
/// This function extracts struct data from the provided item, generates
/// markdown content including fields and documentation, and writes it to
/// the output directory.
pub fn generate(krate: &Crate, item: &Item, output_dir: &Path) -> error::Result<()> {
    let struct_data = extract_struct_data(&item.inner)?;
    let item_map = &krate.index;

    let content = generate_struct_content(item, struct_data, item_map);
    let filename = markdown::utils::generate_filename(&item.id.0.to_string());
    let output_path = output_dir.join(&filename);

    markdown::utils::write_markdown_file(&output_path, &content)?;

    Ok(())
}

/// Extract struct data from an ItemEnum variant.
///
/// This function unwraps the ItemEnum to get the Struct data, returning
/// an error if the item is not a struct.
fn extract_struct_data(inner: &ItemEnum) -> error::Result<&Struct> {
    match inner {
        ItemEnum::Struct(struct_data) => Ok(struct_data),
        _ => Err(error::MarkdownError::ItemNotFound(format!(
            "Expected struct item, found {:?}",
            inner
        ))
        .into()),
    }
}

/// Generate the complete markdown content for a struct.
///
/// This function assembles all sections of the struct documentation including
/// the header, description, fields, and next actions.
fn generate_struct_content(
    item: &Item,
    struct_data: &Struct,
    item_map: &std::collections::HashMap<Id, Item>,
) -> String {
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

    let fields_section = generate_fields_section(&struct_data.kind, item_map);
    if !fields_section.is_empty() {
        content.push('\n');
        content.push_str(&fields_section);
    }

    let generics_section = generate_generics_section(&struct_data.generics);
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

/// Generate the fields section for a struct.
///
/// This function handles all struct kinds (plain, tuple, unit) and generates
/// appropriate field documentation.
fn generate_fields_section(
    struct_kind: &StructKind,
    item_map: &std::collections::HashMap<Id, Item>,
) -> String {
    let mut section = String::new();

    let fields = match struct_kind {
        StructKind::Plain { fields, .. } => render_plain_fields(fields, item_map),
        StructKind::Tuple(fields) => render_tuple_fields(fields, item_map),
        StructKind::Unit => return String::new(),
    };

    if !fields.is_empty() {
        section.push_str(&markdown::utils::render_header(
            markdown::SECTION_HEADER_LEVEL,
            "Fields",
        ));
        section.push('\n');
        section.push_str(&fields);
    }

    section
}

/// Render plain (named) fields for a struct.
///
/// This function generates a bullet list of named fields with their types,
/// visibility, and documentation.
fn render_plain_fields(field_ids: &[Id], item_map: &std::collections::HashMap<Id, Item>) -> String {
    let mut fields = String::new();

    for field_id in field_ids {
        let field = match item_map.get(field_id) {
            Some(item) => item,
            None => continue,
        };

        let field_data = match &field.inner {
            ItemEnum::StructField(field_data) => field_data,
            _ => continue,
        };

        let name = field.name.as_ref().map_or("Unnamed", String::as_str);
        let type_str = markdown::utils::render_inline_code(&render_type(field_data));
        let visibility = render_visibility(&field.visibility);

        fields.push_str("- ");
        fields.push_str(&type_str);
        fields.push(' ');
        fields.push_str(name);

        if !visibility.is_empty() {
            fields.push(' ');
            fields.push_str(&visibility);
        }

        let field_docs = markdown::utils::render_documentation(&field.docs);
        if !field_docs.is_empty() {
            fields.push_str(" - ");
            fields.push_str(&field_docs);
        }

        fields.push('\n');
    }

    fields
}

/// Render tuple (unnamed) fields for a struct.
///
/// This function generates a list of positional tuple fields with their types
/// and documentation.
fn render_tuple_fields(
    field_ids: &[Option<Id>],
    item_map: &std::collections::HashMap<Id, Item>,
) -> String {
    let mut fields = String::new();

    for (index, field_id_opt) in field_ids.iter().enumerate() {
        let field_id = match field_id_opt {
            Some(id) => id,
            None => {
                fields.push_str(&format!("- {}: Hidden field\n", index));
                continue;
            }
        };

        let field = match item_map.get(field_id) {
            Some(item) => item,
            None => continue,
        };

        let field_type = match &field.inner {
            ItemEnum::StructField(type_) => type_,
            _ => continue,
        };

        let type_str = markdown::utils::render_inline_code(&render_type(field_type));
        fields.push_str(&format!("- {}: {}", index, type_str));

        let field_docs = markdown::utils::render_documentation(&field.docs);
        if !field_docs.is_empty() {
            fields.push_str(" - ");
            fields.push_str(&field_docs);
        }

        fields.push('\n');
    }

    fields
}

/// Render visibility for a struct field.
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

/// Generate the generics section for a struct.
///
/// This function displays generic type parameters if the struct has any.
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

/// Generate the next actions section for a struct.
///
/// This function provides actionable next steps for exploring the struct.
fn generate_next_actions(item: &Item) -> String {
    let actions = vec![
        format!("View source: `cargo docmd browse --item {}`", item.id.0),
        "Find related structs: `cargo docmd browse --type struct`".to_string(),
    ];

    markdown::utils::render_next_actions_section(&actions)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{GenericParamDef, GenericParamDefKind, Id, Visibility};
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
            inner: ItemEnum::Struct(Struct {
                kind: StructKind::Plain {
                    fields: vec![],
                    has_stripped_fields: false,
                },
                generics: rustdoc_types::Generics {
                    params: vec![],
                    where_predicates: vec![],
                },
                impls: vec![],
            }),
        }
    }

    fn create_struct_field(type_name: &str) -> rustdoc_types::Type {
        rustdoc_types::Type::ResolvedPath(rustdoc_types::Path {
            path: type_name.to_string(),
            id: rustdoc_types::Id(0),
            args: None,
        })
    }

    /////////////////////////////////////////////////////////////////////////////
    // Field Rendering Tests

    #[test]
    fn render_fields_plain_struct_with_visibility() {
        let field_id = Id(1);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            Item {
                id: field_id,
                crate_id: 0,
                name: Some("x".to_string()),
                span: None,
                visibility: Visibility::Public,
                docs: None,
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_struct_field("i32")),
            },
        );

        let result = render_plain_fields(&[field_id], &item_map);
        assert!(result.contains("`i32`"));
        assert!(result.contains("x"));
        assert!(result.contains("(pub)"));
    }

    #[test]
    fn render_fields_private_struct_field() {
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
                docs: None,
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_struct_field("String")),
            },
        );

        let result = render_plain_fields(&[field_id], &item_map);
        assert!(result.contains("`String`"));
        assert!(result.contains("private_field"));
        assert!(!result.contains("(pub)"));
    }

    #[test]
    fn render_fields_with_documentation() {
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
                docs: Some("This field is important".to_string()),
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_struct_field("bool")),
            },
        );

        let result = render_plain_fields(&[field_id], &item_map);
        assert!(result.contains("`bool`"));
        assert!(result.contains("documented"));
        assert!(result.contains("This field is important"));
    }

    #[test]
    fn render_tuple_fields_positional() {
        let field_id = Id(4);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            Item {
                id: field_id,
                crate_id: 0,
                name: None,
                span: None,
                visibility: Visibility::Public,
                docs: None,
                links: HashMap::new(),
                attrs: vec![],
                deprecation: None,
                inner: ItemEnum::StructField(create_struct_field("f64")),
            },
        );

        let result = render_tuple_fields(&[Some(field_id)], &item_map);
        assert!(result.contains("0:"));
        assert!(result.contains("`f64`"));
    }

    #[test]
    fn render_tuple_fields_hidden() {
        let result = render_tuple_fields(&[None], &HashMap::new());
        assert!(result.contains("0: Hidden field"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Type Rendering Tests

    #[test]
    fn render_type_primitive() {
        let type_ = rustdoc_types::Type::Primitive("u32".to_string());
        let result = render_type(&type_);
        assert_eq!(result, "u32");
    }

    #[test]
    fn render_type_generic() {
        let type_ = rustdoc_types::Type::Generic("T".to_string());
        let result = render_type(&type_);
        assert_eq!(result, "T");
    }

    #[test]
    fn render_type_resolved_path() {
        let type_ = rustdoc_types::Type::ResolvedPath(rustdoc_types::Path {
            path: "std::vec::Vec".to_string(),
            id: rustdoc_types::Id(0),
            args: None,
        });
        let result = render_type(&type_);
        assert_eq!(result, "std::vec::Vec");
    }

    #[test]
    fn render_type_tuple() {
        let type_ = rustdoc_types::Type::Tuple(vec![
            rustdoc_types::Type::Primitive("i32".to_string()),
            rustdoc_types::Type::Primitive("String".to_string()),
        ]);
        let result = render_type(&type_);
        assert_eq!(result, "(i32, String)");
    }

    #[test]
    fn render_type_array() {
        let type_ = rustdoc_types::Type::Array {
            type_: Box::new(rustdoc_types::Type::Primitive("u8".to_string())),
            len: "32".to_string(),
        };
        let result = render_type(&type_);
        assert_eq!(result, "[u8; 32]");
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

    #[test]
    fn generate_generics_section_with_lifetime() {
        let generics = rustdoc_types::Generics {
            params: vec![GenericParamDef {
                name: "'a".to_string(),
                kind: GenericParamDefKind::Lifetime { outlives: vec![] },
            }],
            where_predicates: vec![],
        };
        let result = generate_generics_section(&generics);
        assert!(result.contains("`'a`"));
        assert!(result.contains("lifetime"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Next Actions Tests

    #[test]
    fn generate_next_actions_includes_view_source() {
        let item = create_test_item("TestStruct", None);
        let result = generate_next_actions(&item);
        assert!(result.contains("View source:"));
        assert!(result.contains("cargo docmd browse --item"));
    }

    #[test]
    fn generate_next_actions_includes_related() {
        let item = create_test_item("TestStruct", None);
        let result = generate_next_actions(&item);
        assert!(result.contains("Find related structs"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Content Generation Tests

    #[test]
    fn generate_struct_content_plain_struct() {
        let item = create_test_item("PlainStruct", Some("A plain struct"));
        let struct_data = Struct {
            kind: StructKind::Plain {
                fields: vec![],
                has_stripped_fields: false,
            },
            generics: rustdoc_types::Generics {
                params: vec![],
                where_predicates: vec![],
            },
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_struct_content(&item, &struct_data, &item_map);
        assert!(result.contains("# PlainStruct"));
        assert!(result.contains("A plain struct"));
        assert!(!result.contains("Fields"));
    }

    #[test]
    fn generate_struct_content_with_generics() {
        let item = create_test_item("GenericStruct", None);
        let struct_data = Struct {
            kind: StructKind::Unit,
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
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_struct_content(&item, &struct_data, &item_map);
        assert!(result.contains("Generic Parameters"));
        assert!(result.contains("`T`"));
    }
}
