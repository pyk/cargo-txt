//! Generates markdown documentation for type aliases.
//!
//! This module handles generating markdown files for Rust type alias items.
//! Type aliases provide alternative names for existing types, often used to
//! simplify complex types or improve readability. The generator extracts
//! the target type, generic parameters, and documentation to create
//! comprehensive documentation for coding agents.

use rustdoc_types::{
    GenericArg, GenericArgs, GenericParamDefKind, Generics, Id, Item, Type, TypeAlias, VariantKind,
};
use std::collections::HashMap;
use std::path::Path;

use crate::error;
use crate::markdown;

/// Generate markdown documentation for a type alias item.
///
/// This function generates markdown content including the target type, generics,
/// and documentation, and writes it to the output directory.
pub fn generate(
    item: &Item,
    alias_data: &TypeAlias,
    crate_index: &HashMap<Id, Item>,
    namespace: Option<&str>,
    output_dir: &Path,
) -> error::Result<()> {
    let content = generate_alias_content(item, alias_data, crate_index, namespace);
    let filename = markdown::utils::generate_filename(&format!("{}", item.id.0));
    let output_path = output_dir.join(&filename);

    markdown::utils::write_markdown_file(&output_path, &content)?;

    Ok(())
}

/// Generate the complete markdown content for a type alias.
///
/// This function assembles all sections of the type alias documentation including
/// the header, description, target type, generics, and next actions.
fn generate_alias_content(
    item: &Item,
    alias_data: &TypeAlias,
    crate_index: &HashMap<Id, Item>,
    namespace: Option<&str>,
) -> String {
    let mut content = String::new();

    let name = item.name.as_ref().map_or("Anonymous", String::as_str);
    content.push_str(&markdown::utils::render_header(
        markdown::ITEM_HEADER_LEVEL,
        &format!("Type Alias `{}`", name),
    ));
    content.push('\n');
    content.push('\n');

    // Namespace section
    if let Some(ns) = namespace {
        content.push_str(&format!("**Namespace:** `{}`", ns));
        content.push('\n');
        content.push('\n');
    }

    // Definition section
    let type_str = render_type(&alias_data.type_);
    let generics_str = if !alias_data.generics.params.is_empty() {
        let param_names: Vec<String> = alias_data
            .generics
            .params
            .iter()
            .map(|p| p.name.clone())
            .collect();
        format!("<{}>", param_names.join(", "))
    } else {
        String::new()
    };
    // For the definition, show the short form without the full path
    let short_type_str = match &alias_data.type_ {
        Type::ResolvedPath(path) => {
            let base_name = path.path.split("::").last().unwrap_or(&path.path);
            if let Some(boxed_args) = &path.args {
                format!("{}{}", base_name, render_generic_args(boxed_args))
            } else {
                base_name.to_string()
            }
        }
        _ => type_str,
    };
    let definition_code = format!("pub type {}{} = {};", name, generics_str, short_type_str);
    content.push_str("**Definition:**\n\n");
    content.push_str(&markdown::utils::render_code_block(
        &definition_code,
        Some("rust"),
    ));
    content.push_str("\n");

    // Aliased type section with full enum definition
    let aliased_type_section = generate_aliased_type_section(&alias_data.type_, crate_index);
    content.push_str(&aliased_type_section);

    // Documentation
    let docs = markdown::utils::render_documentation(&item.docs);
    if !docs.is_empty() {
        content.push_str("### Description\n\n");
        content.push_str(&docs);
        content.push('\n');
    }

    // Variants table if the aliased type is an enum
    let variants_table = generate_variants_table(&alias_data.type_, crate_index);
    if !variants_table.is_empty() {
        content.push('\n');
        content.push_str("---\n\n");
        content.push_str(&variants_table);
    }

    // Implementations section
    let implementations_section = generate_implementations_section(&alias_data.type_, crate_index);
    if !implementations_section.is_empty() {
        content.push_str(&implementations_section);
    }

    let next_actions = generate_next_actions(item);
    if !next_actions.is_empty() {
        content.push('\n');
        content.push_str(&next_actions);
    }

    content
}

/// Generate the aliased type section showing the actual type definition.
///
/// This function looks up the aliased type in the crate index and displays
/// its full definition, including variants if it's an enum.
fn generate_aliased_type_section(type_: &Type, crate_index: &HashMap<Id, Item>) -> String {
    let mut section = String::new();
    section.push_str("**Aliased Type:**\n\n");
    // Try to find the aliased type in the index
    let aliased_id = match type_ {
        Type::ResolvedPath(path) => Some(path.id),
        _ => None,
    };

    let aliased_item = match aliased_id.and_then(|id| crate_index.get(&id)) {
        Some(item) => item,
        None => return section,
    };

    // Generate the type definition based on what kind of item it is
    match &aliased_item.inner {
        rustdoc_types::ItemEnum::Enum(enum_data) => {
            let _name = aliased_item.name.as_ref().map_or("Enum", String::as_str);
            let enum_code =
                generate_enum_definition_code(aliased_item, enum_data, type_, crate_index);
            section.push_str(&markdown::utils::render_code_block(
                &enum_code,
                Some("rust"),
            ));
        }
        rustdoc_types::ItemEnum::Struct(struct_data) => {
            let _name = aliased_item.name.as_ref().map_or("Struct", String::as_str);
            let struct_code = generate_struct_definition_code(aliased_item, struct_data);
            section.push_str(&markdown::utils::render_code_block(
                &struct_code,
                Some("rust"),
            ));
        }
        _ => {
            // For other types, just show the type string
            let type_str = render_type(type_);
            section.push_str(&markdown::utils::render_code_block(&type_str, Some("rust")));
        }
    }

    section.push('\n');
    section
}

/// Generate Rust code for an enum definition.
fn generate_enum_definition_code(
    item: &Item,
    enum_data: &rustdoc_types::Enum,
    alias_type: &Type,
    _crate_index: &HashMap<Id, Item>,
) -> String {
    let name = item.name.as_ref().map_or("Enum", String::as_str);

    // Extract only generic type parameters (like T) from alias type
    let generics = match alias_type {
        Type::ResolvedPath(path) => {
            if let Some(boxed_args) = &path.args {
                render_generic_type_params(boxed_args)
            } else {
                String::new()
            }
        }
        _ => String::new(),
    };

    let mut code = if generics.is_empty() {
        format!("pub enum {} {{\n", name)
    } else {
        format!("pub enum {}{} {{\n", name, generics)
    };

    for variant_id in &enum_data.variants {
        if let Some(variant_item) = _crate_index.get(variant_id) {
            if let Some(variant_name) = &variant_item.name {
                // Add variant type from alias's generic arguments
                let variant_type = get_variant_type_from_alias(alias_type, variant_name);
                if variant_type.is_empty() {
                    code.push_str(&format!("    {},\n", variant_name));
                } else {
                    code.push_str(&format!("    {}({}),\n", variant_name, variant_type));
                }
            }
        }
    }

    code.push_str("}");
    code
}

/// Generate Rust code for a struct definition.
fn generate_struct_definition_code(item: &Item, _struct_data: &rustdoc_types::Struct) -> String {
    let _name = item.name.as_ref().map_or("Struct", String::as_str);
    format!("pub struct {{ ... }}")
}

/// Get a single variant's type from the alias type generic arguments.
fn get_variant_type_from_alias(alias_type: &Type, variant_name: &str) -> String {
    if let Type::ResolvedPath(path) = alias_type {
        if let Some(boxed_args) = &path.args {
            if let GenericArgs::AngleBracketed { args, .. } = boxed_args.as_ref() {
                // For Result<T, Error>, Ok has type T and Err has type Error
                if variant_name == "Ok" {
                    if let Some(GenericArg::Type(t)) = args.first() {
                        return render_type_plain(t);
                    }
                } else if variant_name == "Err" {
                    if args.len() > 1 {
                        if let Some(GenericArg::Type(t)) = args.get(1) {
                            return render_type_plain(t);
                        }
                    }
                }
            }
        }
    }
    String::new()
}

/// Render a type from its rustdoc representation without backticks.
///
/// This function converts the rustdoc Type enum to a string representation,
/// handling complex types with generics, references, and pointers.
fn render_type_plain(type_: &Type) -> String {
    match type_ {
        Type::ResolvedPath(path) => {
            let base_name = path.path.split("::").last().unwrap_or(&path.path);
            if let Some(boxed_args) = &path.args {
                format!("{}{}", base_name, render_generic_args(boxed_args))
            } else {
                base_name.to_string()
            }
        }
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(_) => "fn(...)".to_string(),
        Type::Tuple(types) => {
            let inner: Vec<String> = types.iter().map(render_type_plain).collect();
            format!("({})", inner.join(", "))
        }
        Type::Slice(type_) => format!("[{}]", render_type_plain(type_)),
        Type::Array { type_, len } => format!("[{}; {}]", render_type_plain(type_), len),
        Type::Pat { type_, .. } => render_type_plain(type_),
        Type::RawPointer { is_mutable, type_ } => {
            if *is_mutable {
                format!("*mut {}", render_type_plain(type_))
            } else {
                format!("*const {}", render_type_plain(type_))
            }
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mut result = String::from("&");
            if let Some(lt) = lifetime {
                result.push_str(lt);
                result.push(' ');
            }
            if *is_mutable {
                result.push_str("mut ");
            }
            result.push_str(&render_type_plain(type_));
            result
        }
        Type::ImplTrait(_) => "impl Trait".to_string(),
        Type::Infer => "_".to_string(),
        Type::DynTrait(_) => "dyn Trait".to_string(),
        Type::QualifiedPath { .. } => "<qualified path>".to_string(),
    }
}

/// Generate a variants table for enum types.
///
/// This function creates a markdown table showing all variants of an enum
/// with their types and descriptions.
fn generate_variants_table(type_: &Type, crate_index: &HashMap<Id, Item>) -> String {
    let aliased_id = match type_ {
        Type::ResolvedPath(path) => path.id,
        _ => return String::new(),
    };

    let aliased_item = match crate_index.get(&aliased_id) {
        Some(item) => item,
        None => return String::new(),
    };

    let enum_data = match &aliased_item.inner {
        rustdoc_types::ItemEnum::Enum(data) => data,
        _ => return String::new(),
    };

    let mut table = String::new();
    table.push_str("## Variants\n\n");
    table.push_str("| Variant | Type    | Description                |\n");
    table.push_str("| ------- | ------- | -------------------------- |\n");

    for variant_id in &enum_data.variants {
        if let Some(variant_item) = crate_index.get(variant_id) {
            let variant_name = variant_item.name.as_ref().map_or("?", String::as_str);
            let variant_data = match &variant_item.inner {
                rustdoc_types::ItemEnum::Variant(v) => v,
                _ => continue,
            };

            let variant_type = match &variant_data.kind {
                VariantKind::Plain => "N/A".to_string(),
                VariantKind::Tuple(_) => {
                    // Get the type from the alias's generic arguments
                    let vt = get_variant_type_from_alias(type_, variant_name);
                    if vt.is_empty() { "T".to_string() } else { vt }
                }
                VariantKind::Struct { fields, .. } => {
                    let field_names: Vec<String> = fields
                        .iter()
                        .filter_map(|fid| crate_index.get(fid).and_then(|f| f.name.clone()))
                        .collect();
                    field_names.join(", ")
                }
            };

            let desc = variant_item.docs.as_ref().map_or_else(
                || String::new(),
                |d| d.lines().next().unwrap_or("").to_string(),
            );
            table.push_str(&format!(
                "| `{}`    | `{}`     | {} |\n",
                variant_name,
                variant_type.trim_matches('`'),
                desc
            ));
        }
    }

    table.push('\n');
    table
}

/// Generate the implementations section for the aliased type.
///
/// This function groups implementations by category and displays them
/// in a structured format.
fn generate_implementations_section(type_: &Type, crate_index: &HashMap<Id, Item>) -> String {
    let aliased_id = match type_ {
        Type::ResolvedPath(path) => path.id,
        _ => {
            // Return generic message when type is not a resolved path
            let mut section = String::new();
            section.push_str("---\n\n");
            section.push_str("## Implementations\n\n");
            section.push_str("This type inherits all implementations from the aliased type.\n\n");
            return section;
        }
    };

    let aliased_item = match crate_index.get(&aliased_id) {
        Some(item) => item,
        None => {
            // Return generic message when aliased type not found
            let mut section = String::new();
            section.push_str("---\n\n");
            section.push_str("## Implementations\n\n");
            section.push_str("This type inherits all implementations from the aliased type.\n\n");
            return section;
        }
    };

    let enum_data = match &aliased_item.inner {
        rustdoc_types::ItemEnum::Enum(data) => data,
        _ => return String::new(),
    };

    let mut section = String::new();
    section.push_str("---\n\n");
    section.push_str("## Implementations\n\n");

    // Collect all impl items
    let impl_items: Vec<&Item> = enum_data
        .impls
        .iter()
        .filter_map(|id| crate_index.get(id))
        .collect();

    section.push_str("This type inherits all implementations from ");
    section.push_str(
        &aliased_item
            .name
            .as_ref()
            .map_or("the aliased type", String::as_str),
    );
    section.push_str(".\n\n");

    if impl_items.is_empty() {
        section.push_str("No implementations found.\n");
        return section;
    }

    section.push_str(
        &aliased_item
            .name
            .as_ref()
            .map_or("the aliased type", String::as_str),
    );
    section.push_str(".\n\n");

    // Group implementations by category
    let mut grouped_impls: HashMap<String, Vec<String>> = HashMap::new();
    let mut trait_impls: Vec<(String, Vec<String>)> = Vec::new();

    for impl_item in &impl_items {
        let impl_data = match &impl_item.inner {
            rustdoc_types::ItemEnum::Impl(imp) => imp,
            _ => continue,
        };

        // Get trait name if this is a trait impl
        if let Some(trait_) = &impl_data.trait_ {
            let trait_name = trait_.path.clone();
            let methods: Vec<String> = impl_data
                .items
                .iter()
                .filter_map(|id| {
                    crate_index.get(id).and_then(|item| {
                        item.name
                            .as_ref()
                            .map(|name| format!("pub fn {}(...)", name))
                    })
                })
                .collect();

            if !methods.is_empty() {
                trait_impls.push((trait_name, methods));
            }
        } else {
            // Inherent impl - group by functionality
            let methods: Vec<String> = impl_data
                .items
                .iter()
                .filter_map(|id| {
                    crate_index.get(id).and_then(|item| {
                        item.name
                            .as_ref()
                            .map(|name| format!("pub fn {}(...)", name))
                    })
                })
                .collect();

            // Categorize based on method names
            for method in &methods {
                let category = categorize_method(method);
                grouped_impls
                    .entry(category.to_string())
                    .or_insert_with(Vec::new)
                    .push(method.clone());
            }
        }
    }

    // Display inherent implementations by category
    let categories = vec![
        "Inspectors",
        "Converters",
        "Transformers",
        "Combinators",
        "Extractors (Unwrap)",
        "Unsafe",
        "Iterators",
    ];

    for category in &categories {
        if let Some(methods) = grouped_impls.get(*category) {
            section.push_str(&format!("### {}\n\n", category));
            for method in methods {
                section.push_str(&format!("{}\n", method));
            }
            section.push('\n');
        }
    }

    // Display trait implementations
    if !trait_impls.is_empty() {
        section.push_str("## Trait Implementations\n\n");
        for (trait_name, methods) in &trait_impls {
            section.push_str(&format!("- **`{}`**: ", trait_name));
            for method in methods {
                section.push_str(&format!("`{}`, ", method));
            }
            section.push_str("\n");
        }
        section.push('\n');
    }

    section
}

/// Categorize a method based on its name pattern.
fn categorize_method(method: &str) -> &'static str {
    if method.contains("is_ok") || method.contains("is_err") {
        "Inspectors"
    } else if method.contains("ok()") || method.contains("err()") || method.contains("as_ref") {
        "Converters"
    } else if method.contains("map") || method.contains("inspect") {
        "Transformers"
    } else if method.contains("and") || method.contains("or") {
        "Combinators"
    } else if method.contains("unwrap") || method.contains("expect") {
        "Extractors (Unwrap)"
    } else if method.contains("unsafe") {
        "Unsafe"
    } else if method.contains("iter") {
        "Iterators"
    } else {
        "Other"
    }
}

/// Render a type from its rustdoc representation.
///
/// This function converts the rustdoc Type enum to a string representation,
/// handling complex types with generics, references, and pointers.
fn render_type(type_: &Type) -> String {
    match type_ {
        Type::ResolvedPath(path) => render_resolved_path_with_generics(path),
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(_) => "fn(...)".to_string(),
        Type::Tuple(types) => {
            let inner: Vec<String> = types.iter().map(render_type).collect();
            format!("({})", inner.join(", "))
        }
        Type::Slice(type_) => format!("[{}]", render_type(type_)),
        Type::Array { type_, len } => format!("[{}; {}]", render_type(type_), len),
        Type::Pat { type_, .. } => render_type(type_),
        Type::RawPointer { is_mutable, type_ } => {
            if *is_mutable {
                format!("*mut {}", render_type(type_))
            } else {
                format!("*const {}", render_type(type_))
            }
        }
        Type::ImplTrait(_) => "impl Trait".to_string(),
        Type::Infer => "_".to_string(),
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mut result = String::from("&");
            if let Some(lt) = lifetime {
                result.push_str(lt);
                result.push(' ');
            }
            if *is_mutable {
                result.push_str("mut ");
            }
            result.push_str(&render_type(type_));
            result
        }
        Type::DynTrait(_) => "dyn Trait".to_string(),
        Type::QualifiedPath { .. } => "<qualified path>".to_string(),
    }
}

/// Render a resolved path with generic arguments.
///
/// This helper function formats a path type with its generic parameters.
fn render_resolved_path_with_generics(path: &rustdoc_types::Path) -> String {
    let args_vec = match &path.args {
        Some(boxed_args) => match boxed_args.as_ref() {
            GenericArgs::AngleBracketed { args, .. } => args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArg::Type(t) => Some(render_type(t)),
                    GenericArg::Lifetime(l) => Some(l.to_string()),
                    GenericArg::Const(_) => Some("const".to_string()),
                    _ => None,
                })
                .collect(),
            GenericArgs::Parenthesized { .. } => vec!["(...)".to_string()],
            GenericArgs::ReturnTypeNotation => vec!["(...) -> _".to_string()],
        },
        None => vec![],
    };

    if args_vec.is_empty() {
        path.path.clone()
    } else {
        format!("{}<{}>", path.path, args_vec.join(", "))
    }
}

/// Render generic type parameters (not concrete types) as a string.
///
/// This helper function formats only generic type parameters (like T, U) from
/// generic arguments, filtering out concrete types (like Error).
fn render_generic_type_params(args: &GenericArgs) -> String {
    match args {
        GenericArgs::AngleBracketed { args, .. } => {
            let param_strs: Vec<String> = args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArg::Type(Type::Generic(name)) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            if param_strs.is_empty() {
                String::new()
            } else {
                format!("<{}>", param_strs.join(", "))
            }
        }
        GenericArgs::Parenthesized { .. } => String::new(),
        GenericArgs::ReturnTypeNotation => String::new(),
    }
}

/// Render generic arguments as a string.
///
/// This helper function formats generic argument collections.
fn render_generic_args(args: &GenericArgs) -> String {
    match args {
        GenericArgs::AngleBracketed { args, .. } => {
            let arg_strs: Vec<String> = args
                .iter()
                .map(|arg| match arg {
                    GenericArg::Type(t) => render_type_plain(t),
                    GenericArg::Lifetime(l) => l.clone(),
                    GenericArg::Const(_) => "const".to_string(),
                    _ => "?".to_string(),
                })
                .collect();

            if arg_strs.is_empty() {
                String::new()
            } else {
                format!("<{}>", arg_strs.join(", "))
            }
        }
        GenericArgs::Parenthesized { .. } => "(...)".to_string(),
        GenericArgs::ReturnTypeNotation => "(..) -> _".to_string(),
    }
}

/// Generate the generics section for a type alias.
///
/// This function displays generic type parameters if the alias has any.
fn generate_generics_section(generics: &Generics) -> String {
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
            GenericParamDefKind::Lifetime { .. } => "lifetime",
            GenericParamDefKind::Type { .. } => "type",
            GenericParamDefKind::Const { .. } => "const",
        };
        section.push_str(&format!("- `{}`: {}\n", name, kind_str));
    }

    section
}

/// Generate the next actions section for a type alias.
///
/// This function provides actionable next steps for exploring the type alias.
fn generate_next_actions(item: &Item) -> String {
    let actions = vec![
        format!("View source: `cargo docmd browse --item {}`", item.id.0),
        "Find related aliases: `cargo docmd browse --type type-alias`".to_string(),
    ];

    markdown::utils::render_next_actions_section(&actions)
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::ItemEnum;
    use rustdoc_types::{GenericArg, GenericParamDef, GenericParamDefKind, VariantKind};

    fn create_test_item(
        id: rustdoc_types::Id,
        name: &str,
        docs: Option<&str>,
        inner: ItemEnum,
    ) -> Item {
        Item {
            id,
            crate_id: 0,
            name: Some(name.to_string()),
            span: None,
            visibility: rustdoc_types::Visibility::Public,
            docs: docs.map(String::from),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner,
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    // serde_json::Result Use Case Test

    #[test]
    fn generate_serde_json_result_documentation() {
        // Create the serde_json::Result<T> type alias
        let result_alias = TypeAlias {
            type_: Type::ResolvedPath(rustdoc_types::Path {
                path: "core::result::Result".to_string(),
                id: rustdoc_types::Id(100), // ID of the core Result enum
                args: Some(Box::new(GenericArgs::AngleBracketed {
                    args: vec![
                        GenericArg::Type(Type::Generic("T".to_string())),
                        GenericArg::Type(Type::ResolvedPath(rustdoc_types::Path {
                            path: "Error".to_string(),
                            id: rustdoc_types::Id(200),
                            args: None,
                        })),
                    ],
                    constraints: vec![],
                })),
            }),
            generics: Generics {
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
        };

        let result_alias_item = create_test_item(
            rustdoc_types::Id(1),
            "Result",
            Some("Alias for a `Result` with the error type `serde_json::Error`."),
            ItemEnum::TypeAlias(result_alias),
        );

        // Create the core::result::Result<T, E> enum
        let core_result_item = create_test_item(
            rustdoc_types::Id(100),
            "Result",
            None,
            ItemEnum::Enum(rustdoc_types::Enum {
                variants: vec![rustdoc_types::Id(101), rustdoc_types::Id(102)],
                generics: Generics {
                    params: vec![
                        GenericParamDef {
                            name: "T".to_string(),
                            kind: GenericParamDefKind::Type {
                                bounds: vec![],
                                default: None,
                                is_synthetic: false,
                            },
                        },
                        GenericParamDef {
                            name: "E".to_string(),
                            kind: GenericParamDefKind::Type {
                                bounds: vec![],
                                default: None,
                                is_synthetic: false,
                            },
                        },
                    ],
                    where_predicates: vec![],
                },
                impls: vec![],
                has_stripped_variants: false,
            }),
        );

        // Create Ok variant
        let ok_variant = create_test_item(
            rustdoc_types::Id(101),
            "Ok",
            Some("Contains success value"),
            ItemEnum::Variant(rustdoc_types::Variant {
                kind: VariantKind::Tuple(vec![Some(rustdoc_types::Id(103))]),
                discriminant: None,
            }),
        );

        // Create Err variant
        let err_variant = create_test_item(
            rustdoc_types::Id(102),
            "Err",
            Some("Contains error value"),
            ItemEnum::Variant(rustdoc_types::Variant {
                kind: VariantKind::Tuple(vec![Some(rustdoc_types::Id(104))]),
                discriminant: None,
            }),
        );

        // Build the crate index
        let mut crate_index = HashMap::new();
        crate_index.insert(core_result_item.id, core_result_item);
        crate_index.insert(ok_variant.id, ok_variant);
        crate_index.insert(err_variant.id, err_variant);

        // Generate the markdown
        let alias_data = match &result_alias_item.inner {
            ItemEnum::TypeAlias(data) => data,
            _ => panic!("Expected TypeAlias"),
        };
        let result = generate_alias_content(
            &result_alias_item,
            alias_data,
            &crate_index,
            Some("serde_json"),
        );

        // Assert against the expected format
        let expected = "# Type Alias `Result`

**Namespace:** `serde_json`

**Definition:**

```rust
pub type Result<T> = Result<T, Error>;
```

**Aliased Type:**

```rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

### Description

Alias for a `Result` with the error type `serde_json::Error`.

---

## Variants

| Variant | Type    | Description                |
| ------- | ------- | -------------------------- |
| `Ok`    | `T`     | Contains the success value |
| `Err`   | `Error` | Contains the error value   |

---

## Implementations

This type inherits all implementations from `Result`.

No implementations found.

## Next Actions

- View source: `cargo docmd browse --item 1`
- Find related aliases: `cargo docmd browse --type type-alias`
";

        assert_eq!(result, expected);
    }
}
