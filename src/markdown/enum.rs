//! Enum generator for markdown documentation.
//!
//! This module handles generating markdown files for Rust enum items,
//! including simple enums, data-carrying variants, and explicit discriminants.
//! The generator extracts variant information, discriminants, and documentation
//! to create comprehensive documentation for coding agents.

use rustdoc_types::{Crate, Enum, Id, Item, ItemEnum, VariantKind};
use std::collections::HashMap;
use std::path::Path;

use crate::error;
use crate::markdown;

/// Generate markdown documentation for an enum item.
///
/// This function extracts enum data from the provided item, generates
/// markdown content including variants and documentation, and writes it to
/// the output directory.
pub fn generate(krate: &Crate, item: &Item, output_dir: &Path) -> error::Result<()> {
    let enum_data = extract_enum_data(&item.inner)?;
    let item_map = &krate.index;

    let content = generate_enum_content(item, enum_data, item_map);
    let filename = markdown::utils::generate_filename(&format!("{}", item.id.0));
    let output_path = output_dir.join(&filename);

    markdown::utils::write_markdown_file(&output_path, &content)?;

    Ok(())
}

/// Extract enum data from an ItemEnum variant.
///
/// This function unwraps the ItemEnum to get the Enum data, returning
/// an error if the item is not an enum.
fn extract_enum_data(inner: &ItemEnum) -> error::Result<&Enum> {
    match inner {
        ItemEnum::Enum(enum_data) => Ok(enum_data),
        _ => Err(error::MarkdownError::ItemNotFound(format!(
            "Expected enum item, found {:?}",
            inner
        ))
        .into()),
    }
}

/// Generate the complete markdown content for an enum.
///
/// This function assembles all sections of the enum documentation including
/// the header, description, variants, generics, and next actions.
fn generate_enum_content(item: &Item, enum_data: &Enum, item_map: &HashMap<Id, Item>) -> String {
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

    let variants_section = generate_variants_section(&enum_data.variants, item_map);
    if !variants_section.is_empty() {
        content.push('\n');
        content.push_str(&variants_section);
    }

    let generics_section = generate_generics_section(&enum_data.generics);
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

/// Generate the variants section for an enum.
///
/// This function renders all variants with their data types, discriminants,
/// and documentation.
fn generate_variants_section(variant_ids: &[Id], item_map: &HashMap<Id, Item>) -> String {
    if variant_ids.is_empty() {
        return String::new();
    }

    let mut section = String::new();
    section.push_str(&markdown::utils::render_header(
        markdown::SECTION_HEADER_LEVEL,
        "Variants",
    ));
    section.push('\n');

    for variant_id in variant_ids {
        let variant = match item_map.get(variant_id) {
            Some(item) => item,
            None => continue,
        };

        let variant_data = match &variant.inner {
            ItemEnum::Variant(variant_data) => variant_data,
            _ => continue,
        };

        let name = variant.name.as_ref().map_or("Anonymous", String::as_str);

        // Render variant name and type
        let type_info = render_variant_kind(&variant_data.kind, item_map);
        if type_info.is_empty() {
            section.push_str(&format!("- `{}`", name));
        } else {
            section.push_str(&format!("- `{}{}`", name, type_info));
        }

        // Render discriminant if present
        let discriminant = render_variant_discriminant(&variant_data.discriminant);
        if !discriminant.is_empty() {
            section.push(' ');
            section.push_str(&discriminant);
        }

        // Render variant documentation
        let variant_docs = markdown::utils::render_documentation(&variant.docs);
        if !variant_docs.is_empty() {
            section.push_str(" - ");
            section.push_str(&variant_docs);
        }

        section.push('\n');
    }

    section
}

/// Render the variant kind including associated data.
///
/// This function generates the type information for tuple and struct variants.
fn render_variant_kind(variant_kind: &VariantKind, item_map: &HashMap<Id, Item>) -> String {
    match variant_kind {
        VariantKind::Plain => String::new(),
        VariantKind::Tuple(field_ids) => render_tuple_variant_fields(field_ids, item_map),
        VariantKind::Struct { fields, .. } => render_struct_variant_fields(fields, item_map),
    }
}

/// Render tuple variant fields as a comma-separated list of types.
///
/// This function processes tuple variant fields and returns their types.
fn render_tuple_variant_fields(field_ids: &[Option<Id>], item_map: &HashMap<Id, Item>) -> String {
    let types: Vec<String> = field_ids
        .iter()
        .filter_map(|field_id_opt| {
            let field_id = field_id_opt.as_ref()?;
            let field = item_map.get(field_id)?;

            if !matches!(
                &field.inner,
                ItemEnum::Variant(_) | ItemEnum::StructField(_)
            ) {
                return None;
            }

            Some(render_variant_type(field))
        })
        .collect();

    if types.is_empty() {
        String::new()
    } else {
        format!("({})", types.join(", "))
    }
}

/// Render struct variant fields as a comma-separated list of name: type pairs.
///
/// This function processes struct variant fields and returns their names and types.
fn render_struct_variant_fields(field_ids: &[Id], item_map: &HashMap<Id, Item>) -> String {
    let fields: Vec<String> = field_ids
        .iter()
        .filter_map(|field_id| {
            let field = item_map.get(field_id)?;
            let name = field.name.as_ref()?;
            let type_str = render_variant_type(field);
            Some(format!("{}: {}", name, type_str))
        })
        .collect();

    if fields.is_empty() {
        String::new()
    } else {
        format!(" {{ {} }}", fields.join(", "))
    }
}

/// Render the type for a variant or field.
///
/// This function converts rustdoc types to string representation.
fn render_variant_type(item: &Item) -> String {
    match &item.inner {
        ItemEnum::Variant(_) => item
            .name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string()),
        ItemEnum::StructField(field_data) => render_field_type(field_data),
        _ => "Unknown".to_string(),
    }
}

/// Render a field type from its rustdoc representation.
fn render_field_type(type_: &rustdoc_types::Type) -> String {
    match type_ {
        rustdoc_types::Type::ResolvedPath(path) => path.path.clone(),
        rustdoc_types::Type::Primitive(name) => name.clone(),
        rustdoc_types::Type::Generic(name) => name.clone(),
        rustdoc_types::Type::Tuple(types) => {
            let types_str: Vec<String> = types.iter().map(render_field_type).collect();
            format!("({})", types_str.join(", "))
        }
        rustdoc_types::Type::Slice(inner_type) => {
            format!("[{}]", render_field_type(inner_type))
        }
        rustdoc_types::Type::Array { type_, len } => {
            format!("[{}; {}]", render_field_type(type_), len)
        }
        rustdoc_types::Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mutability = if *is_mutable { "mut " } else { "" };
            let lifetime_str = lifetime
                .as_ref()
                .map_or_else(String::new, |l| format!("{} ", l));
            format!(
                "&{}{}{}",
                lifetime_str,
                mutability,
                render_field_type(type_)
            )
        }
        rustdoc_types::Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{} {}", mutability, render_field_type(type_))
        }
        rustdoc_types::Type::FunctionPointer(_) => "fn(...)".to_string(),
        rustdoc_types::Type::ImplTrait(_) => "impl Trait".to_string(),
        rustdoc_types::Type::DynTrait(_) => "dyn Trait".to_string(),
        rustdoc_types::Type::Infer => "_".to_string(),

        rustdoc_types::Type::QualifiedPath { .. } => "QualifiedPath".to_string(),
        rustdoc_types::Type::Pat { .. } => "Pattern".to_string(),
    }
}

/// Render the discriminant value for a variant.
///
/// This function returns the explicit discriminant value if present.
fn render_variant_discriminant(discriminant: &Option<rustdoc_types::Discriminant>) -> String {
    match discriminant {
        Some(discriminant_value) => format!("= {}", discriminant_value.expr),
        None => String::new(),
    }
}

/// Generate the generics section for an enum.
///
/// This function displays generic type parameters if the enum has any.
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

/// Generate the next actions section for an enum.
///
/// This function provides actionable next steps for exploring the enum.
fn generate_next_actions(item: &Item) -> String {
    let actions = vec![
        format!("View source: `cargo docmd browse --item {}`", item.id.0),
        "Find related enums: `cargo docmd browse --type enum`".to_string(),
    ];

    markdown::utils::render_next_actions_section(&actions)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{GenericParamDef, GenericParamDefKind, Variant, Visibility};
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
            inner: ItemEnum::Enum(Enum {
                generics: rustdoc_types::Generics {
                    params: vec![],
                    where_predicates: vec![],
                },
                variants: vec![],
                has_stripped_variants: false,
                impls: vec![],
            }),
        }
    }

    fn create_test_variant(name: &str, kind: VariantKind) -> Item {
        Item {
            id: Id(0),
            crate_id: 0,
            name: Some(name.to_string()),
            span: None,
            visibility: Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: ItemEnum::Variant(Variant {
                kind,
                discriminant: None,
            }),
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Variant Rendering Tests

    #[test]
    fn render_variant_kind_plain() {
        let kind = VariantKind::Plain;
        let item_map = HashMap::new();
        let result = render_variant_kind(&kind, &item_map);
        assert!(result.is_empty());
    }

    #[test]
    fn render_variant_kind_tuple() {
        let field_id = Id(1);
        let mut item_map = HashMap::new();

        item_map.insert(
            field_id.clone(),
            create_test_variant("dummy", VariantKind::Plain),
        );

        let kind = VariantKind::Tuple(vec![Some(field_id)]);
        let result = render_variant_kind(&kind, &item_map);
        assert!(result.contains("("));
        assert!(result.contains(")"));
    }

    #[test]
    fn render_variant_kind_struct() {
        let field_id = Id(1);
        let mut item_map = HashMap::new();

        let mut field_item = create_test_variant("dummy", VariantKind::Plain);
        field_item.name = Some("x".to_string());
        field_item.inner = ItemEnum::StructField(rustdoc_types::Type::Primitive("i32".to_string()));

        item_map.insert(field_id.clone(), field_item);

        let kind = VariantKind::Struct {
            fields: vec![field_id],
            has_stripped_fields: false,
        };
        let result = render_variant_kind(&kind, &item_map);
        assert!(result.contains("{"));
        assert!(result.contains("}"));
        assert!(result.contains("x:"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Discriminant Rendering Tests

    #[test]
    fn render_variant_discriminant_none() {
        let discriminant = None;
        let result = render_variant_discriminant(&discriminant);
        assert!(result.is_empty());
    }

    #[test]
    fn render_variant_discriminant_with_value() {
        let discriminant = Some(rustdoc_types::Discriminant {
            expr: "42".to_string(),
            value: "42".to_string(),
        });
        let result = render_variant_discriminant(&discriminant);
        assert!(result.contains("= 42"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Field Type Rendering Tests

    #[test]
    fn render_field_type_primitive() {
        let type_ = rustdoc_types::Type::Primitive("u32".to_string());
        let result = render_field_type(&type_);
        assert_eq!(result, "u32");
    }

    #[test]
    fn render_field_type_generic() {
        let type_ = rustdoc_types::Type::Generic("T".to_string());
        let result = render_field_type(&type_);
        assert_eq!(result, "T");
    }

    #[test]
    fn render_field_type_reference() {
        let type_ = rustdoc_types::Type::BorrowedRef {
            lifetime: Some("'a".to_string()),
            is_mutable: false,
            type_: Box::new(rustdoc_types::Type::Primitive("str".to_string())),
        };
        let result = render_field_type(&type_);
        assert_eq!(result, "&'a str");
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
        let item = create_test_item("TestEnum", Some(""));
        let result = generate_next_actions(&item);
        assert!(result.contains("View source:"));
        assert!(result.contains("cargo docmd browse --item"));
    }

    #[test]
    fn generate_next_actions_includes_related() {
        let item = create_test_item("TestEnum", Some(""));
        let result = generate_next_actions(&item);
        assert!(result.contains("Find related enums"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Content Generation Tests

    #[test]
    fn generate_enum_content_simple_enum() {
        let item = create_test_item("SimpleEnum", Some("A simple enum"));
        let enum_data = Enum {
            generics: rustdoc_types::Generics {
                params: vec![],
                where_predicates: vec![],
            },
            variants: vec![],
            has_stripped_variants: false,
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_enum_content(&item, &enum_data, &item_map);
        assert!(result.contains("# SimpleEnum"));
        assert!(result.contains("A simple enum"));
        assert!(!result.contains("Variants"));
    }

    #[test]
    fn generate_enum_content_with_generics() {
        let item = create_test_item("GenericEnum", None);
        let enum_data = Enum {
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
            variants: vec![],
            has_stripped_variants: false,
            impls: vec![],
        };
        let item_map = HashMap::new();

        let result = generate_enum_content(&item, &enum_data, &item_map);
        assert!(result.contains("Generic Parameters"));
        assert!(result.contains("`T`"));
    }
}
