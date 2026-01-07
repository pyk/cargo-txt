//! HTML to Markdown conversion using scraper.
//!
//! This module provides functions to convert HTML strings to markdown
//! by extracting the <main> element content and converting it to markdown.

use crate::error;
use scraper::{Html, Selector};

/// Convert HTML string to markdown by extracting main element content.
///
/// This function parses the HTML, extracts the content within the <main>
/// element, and converts it to markdown format.
pub fn convert(html: &str) -> error::Result<String> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("main").map_err(|e| error::HtmlExtractError::SelectorParseFailed {
            selector: "main".to_string(),
            error: e.to_string(),
        })?;

    let main_element = document.select(&selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "main".to_string(),
        }
    })?;

    let mut markdown = String::new();
    convert_node(main_element, &mut markdown);
    Ok(markdown)
}

/// Recursively convert HTML nodes to markdown.
///
/// This function walks through the HTML node tree and converts each element
/// to its markdown equivalent, handling nested elements appropriately.
fn convert_node(node: scraper::element_ref::ElementRef, output: &mut String) {
    let name = node.value().name();

    match name {
        "h1" => {
            output.push_str("# ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "h2" => {
            output.push_str("## ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "h3" => {
            output.push_str("### ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "h4" => {
            output.push_str("#### ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "h5" => {
            output.push_str("##### ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "h6" => {
            output.push_str("###### ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "p" => {
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "code" => {
            // Check if this code is inside a pre element (code block)
            let is_code_block = node
                .parent()
                .and_then(|p| p.value().as_element())
                .map(|e| e.name() == "pre")
                .unwrap_or(false);

            if !is_code_block {
                if !output.ends_with(' ') && !output.is_empty() {
                    output.push(' ');
                }
                output.push('`');
                convert_children(node, output);
                output.push('`');
            } else {
                // Inside pre, just output the text content
                convert_children(node, output);
            }
        }
        "pre" => {
            output.push_str("```");
            convert_children(node, output);
            output.push_str("\n```\n\n");
        }
        "a" => {
            if !output.ends_with(' ') && !output.is_empty() {
                output.push(' ');
            }
            if let Some(href) = node.value().attr("href") {
                output.push('[');
                convert_children(node, output);
                output.push_str("](");
                output.push_str(href);
                output.push(')');
            } else {
                convert_children(node, output);
            }
        }
        "ul" | "ol" => {
            convert_list(node, output, name == "ol");
            output.push('\n');
        }
        "li" => {
            // Handled by convert_list
            convert_list_item(node, output);
            output.push('\n');
        }
        "strong" | "b" => {
            if !output.ends_with(' ') && !output.is_empty() {
                output.push(' ');
            }
            output.push_str("**");
            convert_children(node, output);
            output.push_str("**");
        }
        "em" | "i" => {
            if !output.ends_with(' ') && !output.is_empty() {
                output.push(' ');
            }
            output.push('_');
            convert_children(node, output);
            output.push('_');
        }
        "blockquote" => {
            output.push_str("> ");
            convert_children(node, output);
            output.push_str("\n\n");
        }
        "br" => {
            output.push_str("\n\n");
        }
        "div" | "span" | "section" | "article" | "header" | "footer" | "nav" | "aside" => {
            // Structural elements - just process children
            convert_children(node, output);
        }
        _ => {
            // Unknown element - just process children
            convert_children(node, output);
        }
    }
}

/// Convert children of a node to markdown.
fn convert_children(node: scraper::element_ref::ElementRef, output: &mut String) {
    for child in node.children() {
        match child.value() {
            scraper::Node::Text(text) => {
                let text_str = text.text.trim();
                if !text_str.is_empty() {
                    output.push_str(text_str);
                }
            }
            scraper::Node::Element(_elem) => {
                convert_node(
                    scraper::element_ref::ElementRef::wrap(child).unwrap(),
                    output,
                );
            }
            _ => {}
        }
    }
}

/// Convert a list node to markdown.
fn convert_list(node: scraper::element_ref::ElementRef, output: &mut String, is_ordered: bool) {
    let mut index = 1;
    for child in node.children() {
        if let Some(elem) = child.value().as_element() {
            if elem.name() == "li" {
                if is_ordered {
                    output.push_str(&format!("{}. ", index));
                    index += 1;
                } else {
                    output.push_str("- ");
                }
                let li_node = scraper::element_ref::ElementRef::wrap(child).unwrap();
                convert_list_item(li_node, output);
                output.push('\n');
            }
        }
    }
}

/// Convert a list item to markdown.
fn convert_list_item(node: scraper::element_ref::ElementRef, output: &mut String) {
    let mut first_text = true;
    for child in node.children() {
        match child.value() {
            scraper::Node::Text(text) => {
                let trimmed = text.text.trim();
                if !trimmed.is_empty() {
                    if !first_text {
                        output.push(' ');
                    }
                    output.push_str(trimmed);
                    first_text = false;
                }
            }
            scraper::Node::Element(_elem) => {
                if !first_text {
                    output.push(' ');
                }
                convert_node(
                    scraper::element_ref::ElementRef::wrap(child).unwrap(),
                    output,
                );
                first_text = false;
            }
            _ => {}
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_heading() {
        let html = "<main><h1>Test Heading</h1></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "# Test Heading\n\n");
    }

    #[test]
    fn test_convert_paragraph() {
        let html = "<main><p>Test paragraph</p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test paragraph\n\n");
    }

    #[test]
    fn test_convert_inline_code() {
        let html = "<main><p>Test <code>code</code></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test `code`\n\n");
    }

    #[test]
    fn test_convert_link() {
        let html = "<main><p>Test <a href=\"http://example.com\">link</a></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test [link](http://example.com)\n\n");
    }

    #[test]
    fn test_convert_bold() {
        let html = "<main><p>Test <strong>bold</strong></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test **bold**\n\n");
    }

    #[test]
    fn test_convert_italic() {
        let html = "<main><p>Test <em>italic</em></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test _italic_\n\n");
    }

    #[test]
    fn test_convert_blockquote() {
        let html = "<main><blockquote>Quote text</blockquote></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "> Quote text\n\n");
    }

    #[test]
    fn test_convert_unordered_list() {
        let html = "<main><ul><li>Item 1</li><li>Item 2</li></ul></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "- Item 1\n- Item 2\n\n");
    }

    #[test]
    fn test_convert_ordered_list() {
        let html = "<main><ol><li>First</li><li>Second</li></ol></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "1. First\n2. Second\n\n");
    }

    #[test]
    fn test_convert_code_block() {
        let html = "<main><pre><code>fn test() {}</code></pre></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "```fn test() {}\n```\n\n");
    }

    #[test]
    fn test_missing_main_element() {
        let html = "<div><h1>No main</h1></div>";
        let result = convert(html);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_nested_elements() {
        let html = "<main><p><strong>Bold</strong> and <em>italic</em></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "**Bold**and _italic_\n\n");
    }
}
