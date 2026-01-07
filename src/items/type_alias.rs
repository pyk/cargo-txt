//! Type alias parsing from rustdoc HTML documentation.
//!
//! This module provides functionality to parse type alias documentation from
//! rustdoc-generated HTML files and convert them to markdown format. The
//! parsing extracts all relevant information including the alias declaration,
//! documentation, aliased type, variants (for enums), and implementations.

use scraper::{Html, Selector};

use crate::error;

/// A type alias from rustdoc HTML documentation.
///
/// This structure contains all information extracted from a type alias
/// documentation page, stored as strings to preserve the exact formatting
/// from the HTML.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    /// The name of the type alias (e.g., "Result")
    pub name: String,

    /// The full type alias declaration (e.g., "pub type Result<T> = Result<T, Error>;")
    pub declaration: String,

    /// Documentation description from the docblock
    pub doc: String,

    /// The full aliased type definition (enum or struct)
    /// Example: "pub enum Result<T> { Ok(T), Err(Error), }"
    pub aliased_type: String,

    /// Enum variants (if aliased type is an enum)
    pub variants: Vec<Variant>,

    /// Inherent implementations (impl without a trait)
    pub implementations: Vec<Implementation>,

    /// Trait implementations (impl for a specific trait)
    pub trait_implementations: Vec<Implementation>,
}

/// A variant in an enum definition.
///
/// Extracts the variant signature and documentation as strings,
/// preserving the exact formatting from the HTML.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The variant signature (e.g., "Ok(T)")
    pub signature: String,

    /// Documentation for this variant
    pub doc: String,
}

/// An implementation block for a type alias.
///
/// This represents either an inherent implementation or a trait
/// implementation, extracted as strings from the HTML.
#[derive(Debug, Clone, PartialEq)]
pub struct Implementation {
    /// The implementation signature (e.g., "impl<T, E> Result<&T, E>")
    pub signature: String,

    /// Functions, methods, and associated items in this implementation
    pub functions: Vec<Function>,
}

/// A function within an implementation block.
///
/// Extracts the function signature and documentation as strings,
/// preserving the exact formatting from the HTML.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// The function signature
    /// Example: "pub const fn copied(self) -> Result<T, E> where T: Copy"
    pub signature: String,

    /// Documentation for this function
    pub doc: String,
}

impl TypeAlias {
    /// Parse a type alias from HTML string.
    ///
    /// This method extracts all information from a rustdoc HTML page for a type
    /// alias, including the name, declaration, documentation, aliased type,
    /// variants, and implementations. The HTML content is provided as a string
    /// and returns a `Result` containing the parsed `TypeAlias` or an
    /// `HtmlExtractError` if required HTML elements are not found or if the
    /// HTML structure is unexpected.
    pub fn from_str(html_str: &str) -> error::Result<Self> {
        let document = Html::parse_document(html_str);

        let name = extract_name(&document)?;
        let declaration = extract_declaration(&document)?;
        let doc = extract_doc(&document)?;
        let aliased_type = extract_aliased_type(&document)?;
        let variants = extract_variants(&document);
        let implementations = extract_implementations(&document, false);
        let trait_implementations = extract_implementations(&document, true);

        Ok(TypeAlias {
            name,
            declaration,
            doc,
            aliased_type,
            variants,
            implementations,
            trait_implementations,
        })
    }

    /// Generate markdown representation of the type alias.
    ///
    /// This method generates markdown documentation following the format
    /// specification, including all fields from the type alias structure, and
    /// returns the markdown as a string.
    pub fn markdown(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("# Type Alias `{}`\n\n", self.name));

        output.push_str("```rust\n");
        output.push_str(&self.declaration);
        output.push_str("\n```\n\n");

        output.push_str(&self.doc);
        output.push_str("\n\n");

        if !self.aliased_type.is_empty() {
            output.push_str("## Aliased Type\n\n");
            output.push_str("```rust\n");
            output.push_str(&self.aliased_type);
            output.push_str("\n```\n\n");
        }

        if !self.variants.is_empty() {
            output.push_str("## Variants\n\n");
            for variant in &self.variants {
                output.push_str(&format!("- `{}`: {}\n", variant.signature, variant.doc));
            }
            output.push('\n');
        }

        if !self.implementations.is_empty() {
            output.push_str("## Implementations\n\n");
            for implementation in &self.implementations {
                generate_implementation_markdown(implementation, &mut output);
            }
        }

        if !self.trait_implementations.is_empty() {
            output.push_str("## Trait Implementations\n\n");
            for trait_implementation in &self.trait_implementations {
                generate_implementation_markdown(trait_implementation, &mut output);
            }
        }

        output
    }
}

/// Extract the type alias name from the HTML.
///
/// The name is found in the main heading inside a `<span class="type">` element.
fn extract_name(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("h1 .type").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "h1 .type".to_string(),
            error: error.to_string(),
        }
    })?;

    let element = document.select(&selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "h1 .type".to_string(),
        }
    })?;

    Ok(element.text().collect::<String>().trim().to_string())
}

/// Extract the type alias declaration from the HTML.
///
/// The declaration is found in the first `<pre class="rust item-decl">` element.
fn extract_declaration(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("pre.rust.item-decl").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "pre.rust.item-decl".to_string(),
            error: error.to_string(),
        }
    })?;

    let element = document.select(&selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "pre.rust.item-decl".to_string(),
        }
    })?;

    let code_selector =
        Selector::parse("code").map_err(|error| error::HtmlExtractError::SelectorParseFailed {
            selector: "code".to_string(),
            error: error.to_string(),
        })?;
    let code_element = element.select(&code_selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "pre.rust.item-decl code".to_string(),
        }
    })?;

    Ok(code_element.text().collect::<String>().trim().to_string())
}

/// Extract the documentation text from the HTML.
///
/// The documentation is found in the first `<div class="docblock">` element.
fn extract_doc(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("div.docblock").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "div.docblock".to_string(),
            error: error.to_string(),
        }
    })?;

    let element = document.select(&selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "div.docblock".to_string(),
        }
    })?;

    Ok(element.text().collect::<String>().trim().to_string())
}

/// Extract the aliased type definition from the HTML.
///
/// The aliased type is found in the element immediately following the section
/// with id="aliased-type", which is a `<pre class="rust item-decl">` element.
fn extract_aliased_type(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("#aliased-type + pre.rust.item-decl").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "#aliased-type + pre.rust.item-decl".to_string(),
            error: error.to_string(),
        }
    })?;

    let element = document.select(&selector).next();

    let Some(element) = element else {
        return Ok(String::new());
    };

    let code_selector =
        Selector::parse("code").map_err(|error| error::HtmlExtractError::SelectorParseFailed {
            selector: "code".to_string(),
            error: error.to_string(),
        })?;
    let code_element = element.select(&code_selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "#aliased-type + pre.rust.item-decl code".to_string(),
        }
    })?;

    Ok(code_element.text().collect::<String>().trim().to_string())
}

/// Extract all enum variants from the HTML.
///
/// Variants are found in `<div class="variants">` section with individual
/// `<section class="variant">` elements. Each variant section is followed by
/// a sibling `<div class="docblock">` element containing the variant's documentation.
fn extract_variants(document: &Html) -> Vec<Variant> {
    let Ok(variants_div_selector) = Selector::parse("div.variants") else {
        return Vec::new();
    };

    let Ok(variant_selector) = Selector::parse("section.variant") else {
        return Vec::new();
    };

    let Ok(docblock_selector) = Selector::parse("div.docblock") else {
        return Vec::new();
    };

    let mut variants = Vec::new();

    for variants_div in document.select(&variants_div_selector) {
        let variant_sections: Vec<_> = variants_div.select(&variant_selector).collect();
        let docblocks: Vec<_> = variants_div.select(&docblock_selector).collect();

        for (i, variant_element) in variant_sections.iter().enumerate() {
            let Ok(signature) = extract_variant_signature(variant_element) else {
                continue;
            };

            let doc = if i < docblocks.len() {
                docblocks[i].text().collect::<String>().trim().to_string()
            } else {
                String::new()
            };

            variants.push(Variant { signature, doc });
        }
    }

    variants
}

/// Extract the variant signature from a variant element.
///
/// The signature is found in the `<h3 class="code-header">` element.
fn extract_variant_signature(element: &scraper::ElementRef) -> error::Result<String> {
    let selector = Selector::parse("h3.code-header").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "h3.code-header".to_string(),
            error: error.to_string(),
        }
    })?;

    let header_element = element.select(&selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "h3.code-header".to_string(),
        }
    })?;

    Ok(header_element.text().collect::<String>())
}

/// Extract implementations from the HTML.
///
/// This is a placeholder for future implementation. Currently returns an empty
/// vector as implementations are loaded via JavaScript in modern rustdoc HTML.
fn extract_implementations(_document: &Html, _is_trait: bool) -> Vec<Implementation> {
    Vec::new()
}

/// Generate markdown for an implementation block.
///
/// This helper function formats an implementation and its functions as markdown.
fn generate_implementation_markdown(implementation: &Implementation, output: &mut String) {
    output.push_str(&format!("### `{}`\n\n", implementation.signature));

    for function in &implementation.functions {
        output.push_str(&format!("#### `{}`\n\n", function.signature));
        output.push_str(&function.doc);
        output.push_str("\n\n");
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    /// Load test HTML file for serde_json::Result type alias.
    ///
    /// This helper function reads the HTML file generated by `cargo doc`.
    /// Panics with helpful error message if file is not found.
    fn load_test_html() -> String {
        let html_path = "target/doc/serde_json/type.Result.html";
        std::fs::read_to_string(html_path).unwrap_or_else(|_| {
            panic!(
                "Test HTML file not found at '{}'.\n\n\
                 Please generate it first by running:\n\
                 cargo doc --package serde_json --no-deps",
                html_path
            )
        })
    }

    #[test]
    fn parsing_from_str_parse_complete_type_alias_from_html() {
        let html = load_test_html();
        let result = TypeAlias::from_str(html.as_str());

        assert!(result.is_ok());

        let type_alias = result.unwrap();
        assert_eq!(type_alias.name, "Result");
        assert_eq!(
            type_alias.declaration,
            "pub type Result<T> = Result<T, Error>;"
        );
        assert_eq!(
            type_alias.doc,
            "Alias for a Result with the error type serde_json::Error."
        );
        assert_eq!(
            type_alias.aliased_type,
            "pub enum Result<T> {\n    Ok(T),\n    Err(Error),\n}"
        );
        assert_eq!(type_alias.variants.len(), 2);
        assert_eq!(type_alias.variants[0].signature, "Ok(T)");
        assert_eq!(type_alias.variants[0].doc, "Contains the success value");
        assert_eq!(type_alias.variants[1].signature, "Err(Error)");
        assert_eq!(type_alias.variants[1].doc, "Contains the error value");
    }

    /////////////////////////////////////////////////////////////////////////////
    // Markdown Generation Tests

    #[test]
    fn markdown_generates_heading_from_type_alias() {
        let type_alias = TypeAlias {
            name: "Result".to_string(),
            declaration: "pub type Result<T> = Result<T, Error>;".to_string(),
            doc: "Documentation text.".to_string(),
            aliased_type: String::new(),
            variants: Vec::new(),
            implementations: Vec::new(),
            trait_implementations: Vec::new(),
        };

        let markdown = type_alias.markdown();
        assert!(markdown.contains("# Type Alias `Result`"));
    }

    #[test]
    fn markdown_generates_declaration_section_from_type_alias() {
        let type_alias = TypeAlias {
            name: "Result".to_string(),
            declaration: "pub type Result<T> = Result<T, Error>;".to_string(),
            doc: "Documentation text.".to_string(),
            aliased_type: String::new(),
            variants: Vec::new(),
            implementations: Vec::new(),
            trait_implementations: Vec::new(),
        };

        let markdown = type_alias.markdown();
        assert!(markdown.contains("```rust"));
        assert!(markdown.contains("pub type Result<T> = Result<T, Error>;"));
    }

    #[test]
    fn markdown_generates_variants_section_from_type_alias() {
        let type_alias = TypeAlias {
            name: "Result".to_string(),
            declaration: "pub type Result<T> = Result<T, Error>;".to_string(),
            doc: "Documentation text.".to_string(),
            aliased_type: String::new(),
            variants: vec![Variant {
                signature: "Ok(T)".to_string(),
                doc: "Success value".to_string(),
            }],
            implementations: Vec::new(),
            trait_implementations: Vec::new(),
        };

        let markdown = type_alias.markdown();
        assert!(markdown.contains("## Variants"));
        assert!(markdown.contains("`Ok(T)`"));
        assert!(markdown.contains("Success value"));
    }

    /////////////////////////////////////////////////////////////////////////////
    // Error Tests

    #[test]
    fn error_from_str_returns_error_when_name_element_missing() {
        let html = "<html><body></body></html>";
        let result = TypeAlias::from_str(html);
        assert!(result.is_err());
    }

    #[test]
    fn error_from_str_returns_error_when_declaration_element_missing() {
        let html =
            "<html><body><h1>Type Alias <span class=\"type\">Result</span></h1></body></html>";
        let result = TypeAlias::from_str(html);
        assert!(result.is_err());
    }
}
