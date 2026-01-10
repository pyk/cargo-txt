//! HTML to Markdown conversion using scraper.
//!
//! This module provides functions to convert HTML strings to markdown
//! by extracting the <main> element content and converting it to markdown.

use anyhow::{Result, bail};
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

/// Convert HTML string to markdown by extracting main element content.
///
/// This function parses the HTML, extracts the content within the <main>
/// element, and converts it to markdown format.
pub fn convert(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse("main") {
        Ok(s) => s,
        Err(e) => bail!("failed to parse HTML selector for main element: {}", e),
    };

    let main_element = match document.select(&selector).next() {
        Some(e) => e,
        None => bail!(
            "HTML document does not contain a <main> element. This may indicate invalid rustdoc HTML output."
        ),
    };

    let mut markdown = String::new();
    convert_node(main_element, &mut markdown);
    Ok(markdown)
}

/// Check if a node should be skipped based on its attributes.
///
/// Returns true for rustdoc-specific elements that should not be rendered
/// in the markdown output, such as UI controls, toolbars, and anchors.
fn should_skip_node(node: ElementRef) -> bool {
    let elem = node.value();

    match elem.name() {
        "wbr" | "rustdoc-toolbar" | "script" => return true,
        _ => {}
    }

    match elem.attr("id") {
        Some("copy-path") | Some("implementors") | Some("implementors-list") => return true,
        _ => {}
    }

    let should_skip_class = match elem.attr("class") {
        Some(class) => {
            class.contains("src")
                || class.contains("hideme")
                || class.contains("anchor")
                || class.contains("rustdoc-breadcrumbs")
                || class.contains("tooltip")
        }
        None => false,
    };
    if should_skip_class {
        return true;
    }

    false
}

/// Recursively convert HTML nodes to markdown.
///
/// This function walks through the HTML node tree and converts each element
/// to its markdown equivalent, handling nested elements appropriately.
fn convert_node(node: ElementRef, output: &mut String) {
    if should_skip_node(node) {
        return;
    }

    let name = node.value().name();

    match name {
        "h1" => {
            output.push_str("# ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "h2" => {
            output.push_str("## ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "h3" => {
            output.push_str("### ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "h4" => {
            output.push_str("#### ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "h5" => {
            output.push_str("##### ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "h6" => {
            output.push_str("###### ");
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "p" => {
            convert_children_normalized(node, output);
            output.push_str("\n\n");
        }
        "code" => {
            let is_code_block = node
                .parent()
                .and_then(|p| p.value().as_element())
                .map(|e| e.name() == "pre")
                .unwrap_or(false);

            if !is_code_block {
                output.push('`');
                convert_children(node, output);
                output.push('`');
            } else {
                convert_children(node, output);
            }
        }
        "pre" => {
            output.push_str("```\n");
            convert_children(node, output);
            output.push_str("\n```\n\n");
        }
        "div" | "section" | "article" | "header" | "footer" | "nav" | "aside" => {
            convert_children(node, output);
        }
        "span" => {
            convert_children(node, output);
        }
        "a" => {
            convert_children(node, output);
        }
        "ul" | "ol" => {
            convert_list(node, output, name == "ol");
            output.push('\n');
        }
        "li" => {
            convert_list_item(node, output);
        }
        "dl" => {
            convert_definition_list(node, output);
            output.push('\n');
        }
        "dt" => {
            output.push_str("- **");
            convert_children(node, output);
            output.push_str("**");
        }
        "dd" => {
            output.push_str(": ");
            convert_children(node, output);
            output.push('\n');
        }
        "strong" | "b" => {
            output.push_str("**");
            convert_children(node, output);
            output.push_str("**");
        }
        "em" | "i" => {
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
        _ => {
            convert_children(node, output);
        }
    }
}

/// Convert children of a node to markdown with normalized whitespace.
///
/// This is used for block-level elements where whitespace should be collapsed
/// into single spaces (paragraphs, headings, list items, definition terms/descriptions).
fn convert_children_normalized(node: ElementRef, output: &mut String) {
    let mut buffer = String::new();
    convert_children(node, &mut buffer);
    let normalized: Vec<&str> = buffer.split_whitespace().collect();
    let normalized = normalized.join(" ");
    output.push_str(&normalized);
}

/// Convert children of a node to markdown.
fn convert_children(node: ElementRef, output: &mut String) {
    for child in node.children() {
        match child.value() {
            scraper::Node::Text(text) => {
                let mut text_str = text.text.to_string();
                text_str = text_str.replace('\u{a0}', " ");
                text_str = text_str.replace("&nbsp;", " ");
                if text_str.trim().is_empty() {
                    continue;
                }
                let processed = process_text_links(&text_str);
                output.push_str(&processed);
            }
            scraper::Node::Element(_elem) => {
                let Some(elem_ref) = ElementRef::wrap(child) else {
                    continue;
                };
                convert_node(elem_ref, output);
            }
            _ => {}
        }
    }
}

/// Convert a list node to markdown.
fn convert_list(node: ElementRef, output: &mut String, is_ordered: bool) {
    let mut index = 1;
    for child in node.children() {
        let Some(elem) = child.value().as_element() else {
            continue;
        };
        if elem.name() == "li" {
            if is_ordered {
                output.push_str(&format!("{}. ", index));
                index += 1;
            } else {
                output.push_str("- ");
            }
            let Some(li_node) = ElementRef::wrap(child) else {
                continue;
            };
            convert_list_item(li_node, output);
            output.push('\n');
        }
    }
}

/// Convert a list item to markdown with normalized whitespace.
fn convert_list_item(node: ElementRef, output: &mut String) {
    convert_children_normalized(node, output);
}

/// Convert a definition list (<dl>) to markdown.
///
/// Renders definition terms as bold list items and descriptions on the same line.
/// Format: "- **Term**: Description"
fn convert_definition_list(node: ElementRef, output: &mut String) {
    let mut current_term: Option<String> = None;
    let mut has_description = false;

    for child in node.children() {
        let Some(elem) = child.value().as_element() else {
            continue;
        };

        match elem.name() {
            "dt" => {
                if current_term.is_some() && has_description {
                    output.push('\n');
                }

                let Some(dt_node) = ElementRef::wrap(child) else {
                    continue;
                };

                output.push_str("- **");
                let mut term_text = String::new();
                convert_children_normalized(dt_node, &mut term_text);
                output.push_str(&term_text);
                output.push_str("**");
                current_term = Some(term_text);
                has_description = false;
            }
            "dd" => {
                if current_term.is_some() {
                    output.push_str(": ");
                    let Some(dd_node) = ElementRef::wrap(child) else {
                        continue;
                    };
                    convert_children_normalized(dd_node, output);
                    has_description = true;
                }
            }
            _ => {}
        }
    }

    if current_term.is_some() && has_description {
        output.push('\n');
    }
}

/// Process text to convert markdown reference-style links to plain text.
///
/// Converts patterns like `[text][reference]` to just `text`.
/// This handles rustdoc's internal reference link format.
fn process_text_links(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            let mut link_text = String::new();
            let mut bracket_count = 1;

            while let Some(next) = chars.peek() {
                let next_char = *next;
                if next_char == '[' {
                    bracket_count += 1;
                    chars.next();
                    link_text.push(next_char);
                } else if next_char == ']' {
                    bracket_count -= 1;
                    chars.next();
                    if bracket_count == 0 {
                        break;
                    }
                    link_text.push(next_char);
                } else {
                    chars.next();
                    link_text.push(next_char);
                }
            }

            while let Some(next) = chars.peek() {
                let next_char = *next;
                if next_char.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            }

            match chars.peek() {
                Some(&'[') => {
                    chars.next();
                    let mut ref_bracket_count = 1;
                    for ref_next in chars.by_ref() {
                        if ref_next == '[' {
                            ref_bracket_count += 1;
                        } else if ref_next == ']' {
                            ref_bracket_count -= 1;
                            if ref_bracket_count == 0 {
                                break;
                            }
                        }
                    }
                    result.push_str(&link_text);
                }
                _ => {
                    result.push('[');
                    result.push_str(&link_text);
                    result.push(']');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_heading() {
        let html = "<main><h1>Test Heading</h1></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "# Test Heading\n\n");
    }

    #[test]
    fn convert_paragraph() {
        let html = "<main><p>Test paragraph</p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test paragraph\n\n");
    }

    #[test]
    fn convert_inline_code() {
        let html = "<main><p>Test <code>code</code></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test `code`\n\n");
    }

    #[test]
    fn convert_link() {
        let html = "<main><p>Test <a href=\"http://example.com\">link</a></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test link\n\n");
    }

    #[test]
    fn convert_bold() {
        let html = "<main><p>Test <strong>bold</strong></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test **bold**\n\n");
    }

    #[test]
    fn convert_italic() {
        let html = "<main><p>Test <em>italic</em></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test _italic_\n\n");
    }

    #[test]
    fn convert_blockquote() {
        let html = "<main><blockquote>Quote text</blockquote></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "> Quote text\n\n");
    }

    #[test]
    fn convert_unordered_list() {
        let html = "<main><ul><li>Item 1</li><li>Item 2</li></ul></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "- Item 1\n- Item 2\n\n");
    }

    #[test]
    fn convert_ordered_list() {
        let html = "<main><ol><li>First</li><li>Second</li></ol></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "1. First\n2. Second\n\n");
    }

    #[test]
    fn convert_code_block() {
        let html = "<main><pre><code>fn test() {}</code></pre></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "```\nfn test() {}\n```\n\n");
    }

    #[test]
    fn convert_missing_main_element() {
        let html = "<div><h1>No main</h1></div>";
        let result = convert(html);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("does not contain a <main> element"));
    }

    #[test]
    fn convert_nested_elements() {
        let html = "<main><p><strong>Bold</strong> and <em>italic</em></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "**Bold** and _italic_\n\n");
    }

    #[test]
    fn convert_h1_with_copy_path_button() {
        let html = r#"<main>
            <h1>
                Crate <span>rustdoc_<wbr />types</span> <button id="copy-path">
                    Copy item path
                </button>
            </h1>
        </main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "# Crate rustdoc_types\n\n");
    }

    #[test]
    fn convert_docblock_with_toolbar() {
        let html = r#"<main>
            <rustdoc-toolbar></rustdoc-toolbar>
            <span class="sub-heading">
                <a class="src" href="../src/rustdoc_types/lib.rs.html#1-1465">Source</a>
            </span>
            <summary class="hideme">
                <span>Expand description</span>
            </summary>
            <p>Rustdoc's JSON output interface</p>
        </main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "Rustdoc's JSON output interface\n\n");
    }

    #[test]
    fn convert_inline_code_spacing_and_links() {
        let html = "<main><p>Through the <code>--output-format json</code> flag. The <a href=\"struct.Crate.html\"><code>Crate</code></a> struct.</p></main>";
        let result = convert(html).unwrap();
        assert_eq!(
            result,
            "Through the `--output-format json` flag. The `Crate` struct.\n\n"
        );
    }

    #[test]
    fn convert_h2_with_anchor() {
        let html = "<main><h2 id=\"structs\" class=\"section-header\">Structs<a href=\"#structs\" class=\"anchor\">§</a></h2></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "## Structs\n\n");
    }

    #[test]
    fn convert_definition_list_simple() {
        let html = "<main><dl class=\"item-table\"><dt><a href=\"struct.Crate.html\">Crate</a></dt><dd>The root.</dd><dt><a href=\"struct.Enum.html\">Enum</a></dt><dd>An enum.</dd></dl></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "- **Crate**: The root.\n- **Enum**: An enum.\n\n");
    }

    #[test]
    fn convert_definition_list_with_wbr() {
        let html = "<main><dl class=\"item-table\"><dt><a class=\"struct\" href=\"struct.AssocItemConstraint.html\">Assoc<wbr />Item<wbr />Constraint</a></dt><dd>Describes a bound applied to an associated type/constant.</dd></dl></main>";
        let result = convert(html).unwrap();
        assert_eq!(
            result,
            "- **AssocItemConstraint**: Describes a bound applied to an associated type/constant.\n\n"
        );
    }

    #[test]
    fn convert_non_breaking_space_handling() {
        let html = "<main><p>Test&nbsp;text</p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "Test text\n\n");
    }

    #[test]
    fn convert_markdown_reference_links() {
        let html = r#"<main><ul><li>Derive [tutorial][_derive::_tutorial] and [reference][_derive]</li></ul></main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "- Derive tutorial and reference\n\n");
    }

    #[test]
    fn convert_paragraph_whitespace_normalization() {
        let html = r#"<main><p>
	These types are the public API exposed through
	the <code>--output-format json</code> flag. The
	<a
		href="struct.Crate.html"
		title="struct rustdoc_types::Crate"
		><code>Crate</code></a
	>
	struct is the root of the JSON blob and all
	other items are contained within.
</p></main>"#;
        let result = convert(html).unwrap();
        assert_eq!(
            result,
            "These types are the public API exposed through the `--output-format json` flag. The `Crate` struct is the root of the JSON blob and all other items are contained within.\n\n"
        );
    }

    #[test]
    fn convert_definition_list_whitespace_normalization() {
        let html = r#"<main><dl>
	<dt>
		<a
			class="struct"
			href="struct.AssocItemConstraint.html"
			title="struct rustdoc_types::AssocItemConstraint"
			>Assoc<wbr />Item<wbr />Constraint</a
		>
	</dt>
	<dd>
		Describes a bound applied to an associated
		type/constant.
	</dd>
</dl></main>"#;
        let result = convert(html).unwrap();
        assert_eq!(
            result,
            "- **AssocItemConstraint**: Describes a bound applied to an associated type/constant.\n\n"
        );
    }

    #[test]
    fn convert_code_block_with_newline() {
        let html = r#"<main><div class="example-wrap"><pre class="language-console"><code>$ cargo add clap --features derive</code></pre></div></main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "```\n$ cargo add clap --features derive\n```\n\n");
    }

    #[test]
    fn convert_script_tag_skipped() {
        let html = r#"<main>
            <p>Some content</p>
            <script type="text/json" id="notable-traits-data">
                {
                    "&<Vec<T, A> as Index<I>>::Output": "<h3>Notable traits</h3>"
                }
            </script>
            <p>More content</p>
        </main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "Some content\n\nMore content\n\n");
    }

    #[test]
    fn convert_link_renders_inner_content_only() {
        let html = "<main><p><a href=\"struct.Crate.html\"><code>Crate</code></a> and <a href=\"struct.Enum.html\">Something</a></p></main>";
        let result = convert(html).unwrap();
        assert_eq!(result, "`Crate` and Something\n\n");
    }

    #[test]
    fn convert_rustdoc_breadcrumbs_skipped() {
        let html = r#"<main>
            <div class="main-heading">
                <div class="rustdoc-breadcrumbs">
                    <a href="index.html">serde</a>
                </div>
                <h1>
                    Trait
                    <span class="trait">Deserializer</span>
                </h1>
            </div>
        </main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "# Trait Deserializer\n\n");
    }

    #[test]
    fn convert_tooltip_skipped() {
        let html = r##"<main>
            <p>This example runs with edition 2021 <a href="#" class="tooltip" title="This example runs with edition 2021">ⓘ</a></p>
        </main>"##;
        let result = convert(html).unwrap();
        assert_eq!(result, "This example runs with edition 2021\n\n");
    }

    #[test]
    fn convert_implementors_section_skipped() {
        let html = r##"<main>
            <p>Some content</p>
            <h2 id="implementors" class="section-header">
                Implementors<a href="#implementors" class="anchor">§</a>
            </h2>
            <p>More content</p>
        </main>"##;
        let result = convert(html).unwrap();
        assert_eq!(result, "Some content\n\nMore content\n\n");
    }

    #[test]
    fn convert_implementors_list_skipped() {
        let html = r#"<main>
            <p>Some content</p>
            <div id="implementors-list">
                <p>This should not appear</p>
            </div>
            <p>More content</p>
        </main>"#;
        let result = convert(html).unwrap();
        assert_eq!(result, "Some content\n\nMore content\n\n");
    }

    #[test]
    fn convert_combined_rustdoc_elements_skipped() {
        let html = r##"<main>
            <div class="main-heading">
                <div class="rustdoc-breadcrumbs">
                    <a href="index.html">serde</a>
                </div>
                <h1>
                    Trait
                    <span class="trait">Serializer</span>
                </h1>
            </div>
            <p>Description text</p>
            <h2 id="implementors" class="section-header">
                Implementors<a href="#implementors" class="anchor">§</a>
            </h2>
            <div id="implementors-list">
                <p>Implementor details</p>
            </div>
            <a href="#" class="tooltip" title="Tooltip">ⓘ</a>
            <p>End content</p>
        </main>"##;
        let result = convert(html).unwrap();
        assert_eq!(
            result,
            "# Trait Serializer\n\nDescription text\n\nEnd content\n\n"
        );
    }
}
