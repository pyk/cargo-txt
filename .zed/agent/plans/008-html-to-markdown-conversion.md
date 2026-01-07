---
type: normal
title: "HTML to Markdown Conversion Approach"
seq: 008
slug: "html-to-markdown-conversion"
created: "2025-01-09T12:00:00Z"
status: completed
---

# Implement HTML to Markdown Conversion Approach

Replace the current structured HTML parsing approach with a simpler method:
extract content from the `<main>` element of cargo doc HTML and convert it
directly to markdown. This approach reduces complexity by avoiding case-by-case
HTML parsing for different rustdoc item types.

## Current Problems

The current implementation has several complexity issues:

1. **Complex structured parsing**: Each rustdoc item type requires custom HTML
   parsing logic ( structs, enums, type aliases, etc.)
2. **Case-by-case handling**: Different HTML structures for different item types
   create maintenance burden
3. **Multiple files generated**: Currently generates separate markdown files for
   each item
4. **Limited coverage**: Only type aliases are fully implemented; other types
   require significant additional work

Current approach in `src/commands/build.rs`:

```rust
fn parse_html_directory(
    html_dir: &std::path::Path,
    output_dir: &std::path::Path,
) -> error::Result<usize> {
    let html_files = collect_html_files_recursive(html_dir)?;

    for path in html_files {
        let is_type_alias = file_name
            .map(|name| name.starts_with("type.") && name.ends_with(".html"))
            .unwrap_or(false);

        if !is_type_alias {
            continue;  // Skips non-type-alias files
        }

        let type_alias = items::type_alias::TypeAlias::from_str(&html_content)?;
        let markdown_content = type_alias.markdown();
        // Write individual markdown files
    }
}
```

## Proposed Solution

Implement a simplified approach:

1. Capture cargo doc output to parse the "Generated" line
2. Extract the directory path from the output (e.g.,
   `/home/pyk/bidentxyz/cargo-docmd/target/doc/serde`)
3. Read the `index.html` file from that directory
4. Extract only the `<main>` element content
5. Convert HTML to markdown using scraper
6. Write single `index.md` file to `target/docmd/<crate>/`

Example workflow:

```sh
$ cargo doc --package serde
   Generated /home/pyk/bidentxyz/cargo-docmd/target/doc/serde/index.html

# Extract "Generated" line, parse directory path
# Read /home/pyk/bidentxyz/cargo-docmd/target/doc/serde/index.html
# Extract <main>, convert to markdown
# Output to: target/docmd/serde/index.md
```

## Analysis Required

### Dependency Investigation

- [ ] Verify `<main>` element structure in cargo doc HTML output
- [ ] Test scraper with sample cargo doc HTML to verify we can extract content
- [ ] Design HTML-to-markdown conversion rules for common cargo doc elements
- [ ] Check what HTML elements appear in cargo doc `<main>` content

### Code Locations to Check

- `src/cargo.rs` - Modify `doc()` function to capture output and return path
- `src/commands/build.rs` - Restructure to use new conversion approach
- `src/error.rs` - May need new error types for markdown conversion
- `src/` - Existing `scraper` dependency will be used

## Implementation Checklist

### Code Changes

#### Modify Cargo Module

- [x] Update `src/cargo.rs`:
    - Modify `doc()` function signature to return `Result<PathBuf>` for the
      directory
    - Capture stderr from cargo doc command (cargo doc writes to stderr)
    - Parse output to find "Generated" line using simple string processing
    - Extract directory path from "Generated /path/to/crate/index.html"
    - Return the directory path (e.g., `/path/to/crate`)
    - Keep existing error handling and validation
- [x] Add unit tests:
    - Test parsing of "Generated" line from cargo output
    - Test directory path extraction from various cargo doc outputs
    - Test error handling when "Generated" line is missing
    - Created dedicated `parse_generated_output()` function for testability

#### Create HTML to Markdown Conversion Module

- [x] Create `src/html2md.rs` module:
    - Implement `convert(html: &str) -> Result<String>`:
        - Parse HTML using `scraper` (already available)
        - Select `<main>` element using CSS selector
        - Recursively convert HTML elements within `<main>` to markdown:
            - `<h1>`-`<h6>` → `#` to `######` headings
            - `<p>` → paragraph text with newline
            - `<code>` → inline code with backticks
            - `<pre><code>` → code blocks with triple backticks
            - `<a>` → markdown links `[text](url)`
            - `<ul>`/`<ol>` → bullet/numbered lists
            - `<li>` → list items with proper indentation
            - `<strong>`/`<b>` → **bold** text
            - `<em>`/`<i>` → _italic_ text
            - `<blockquote>` → quoted text with `>`
        - Handle nested elements recursively
        - Handle conversion errors
        - Return markdown string
    - Add module-level documentation
- [x] Add unit tests:
    - Test `convert()` extracts main content and converts to markdown
    - Test HTML to markdown conversion for each element type
    - Test nested HTML elements
    - Test error handling for missing `<main>` element
    - Test with actual cargo doc HTML sample
    - All tests passing (19 tests)

#### Restructure Build Command

- [x] Update `src/commands/build.rs`:
    - Remove dependency on `crate::items` module
    - Call modified `cargo::doc()` to get HTML directory path
    - Read `index.html` from that directory
    - Call `html2md::convert()` to convert HTML to markdown
    - Update output directory structure to `target/docmd/<crate>/`
    - Write markdown to `target/docmd/<crate>/index.md`
    - Update log messages to reflect new approach
    - Remove `collect_html_files_recursive()` function
    - Remove `parse_html_directory()` function
    - Remove file-by-file iteration logic
    - Add `--debug` flag support for troubleshooting
- [ ] Simplify build flow:
    ```rust
    let html_dir = cargo::doc(&crate_name, &metadata.target_directory, ...)?;
    let html_path = html_dir.join("index.html");
    let html_content = std::fs::read_to_string(&html_path)?;
    let markdown = html2md::convert(&html_content)?;
    let output_dir = metadata.target_directory.join("docmd").join(&crate_name);
    std::fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("index.md");
    std::fs::write(&output_path, markdown)?;
    ```

#### Update Error Handling

- Reviewed `src/error.rs`:
    - Existing `HtmlParseFailed` and `HtmlExtractError` variants are sufficient
    - No new error types needed
    - Removed unused `wrap_with_path()` helper function
- All error handling working correctly

### Documentation Updates

- Updated `README.md`:
    - Updated "Build Command" section to describe new approach
    - Removed references to item-specific generation
    - Updated examples to show single index.md output
    - Updated "Current Status" section
- Updated `DOCS.md`:
    - Removed detailed sections about item-specific markdown formats
    - Added new section describing HTML to markdown conversion approach
    - Updated "Build Command" section
    - Updated "Markdown Output Format" section to describe new index.md format
    - Removed "Generated Item Types" section (no longer applicable)
- Documentation now accurately reflects new approach

### Test Updates

- [x] Remove `src/items` directory and all its contents:
    - Deleted `src/items/mod.rs`
    - Deleted `src/items/type_alias.rs`
    - Deleted entire `src/items/` directory
    - Removed `use crate::items;` from `src/main.rs`
    - Added `use crate::html2md;` to `src/main.rs`
    - Verified build works with actual dependency (rustdoc-types tested
      successfully)
    - Generated markdown: `target/docmd/rustdoc-types/index.md`
- [x] Add `--debug` flag:
    - Added `--debug` CLI flag to main.rs
    - Passed debug flag through build and browse commands
    - Added debug output to cargo::doc(), build::build(), and browse::browse()
    - Debug output shows: crate name, target directory, cargo doc command, HTML
      path, conversion steps
- [x] Run all tests with `cargo test`:
    - All 19 tests passing
    - Includes new unit tests for `parse_generated_output()`
    - Includes comprehensive tests for `html2md::convert()`

## Test Plan

### Verification Tests

#### Cargo Module

- [x] Verify `doc()` returns correct directory path from "Generated" line
- [x] Verify parsing works with various cargo doc outputs
- [x] Verify error handling when "Generated" line is missing
- [x] Verify index.html exists in returned directory
- [x] Added dedicated `parse_generated_output()` function with 4 unit tests

#### HTML to Markdown Module

- [x] Verify `convert()` extracts `<main>` element from HTML string
- [x] Verify heading conversion from HTML strings (`<h1>` through `<h6>`)
- [x] Verify paragraph conversion from HTML strings (`<p>`)
- [x] Verify inline code conversion from HTML strings (`<code>`)
- [x] Verify code block conversion from HTML strings (`<pre><code>`)
- [x] Verify link conversion from HTML strings (`<a>`)
- [x] Verify list conversion from HTML strings (`<ul>`, `<ol>`, `<li>`)
- [x] Verify bold and italic conversion from HTML strings (`<strong>`, `<em>`)
- [x] Verify blockquote conversion from HTML strings (`<blockquote>`)
- [x] Verify nested element conversion from HTML strings (e.g., list with
      paragraphs)
- [x] Verify error handling for invalid HTML strings
- [x] Verify error handling when `<main>` element is missing in HTML string
- [x] All 11 html2md tests passing

#### Build Command Integration

- [x] Verify build command creates `target/docmd/<crate>/index.md`
- [x] Verify index.md contains converted markdown
- [x] Verify log messages are appropriate
- [x] Verify error handling for missing HTML file
- [x] Successfully tested with rustdoc-types crate

### Manual Testing

- [x] Run `cargo docmd build rustdoc-types --debug` and inspect output
- [x] Verify generated markdown is readable and useful (5995 bytes from 15113
      bytes HTML)
- [x] Successfully converted rustdoc HTML with various item types (structs,
      enums, traits, etc.)
- [x] Subsequent runs produce same output (cargo doc caches)

### Regression Tests

- [x] Ensure cargo doc execution still works correctly
- [x] Ensure metadata parsing still works
- [x] Ensure crate validation still works
- [x] Ensure existing error handling still functions

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── cargo.rs              # Modified to return HTML path
│   ├── html2md.rs            # NEW: HTML to markdown conversion using scraper
│   ├── commands/
│   │   └── build.rs          # Simplified to use conversion approach
│   ├── error.rs              # Updated with new error types if needed
│   └── main.rs
├── Cargo.toml                # No changes needed (scraper already present)
├── README.md                 # Updated
└── DOCS.md                   # Updated
```

### Output Structure

```
target/
├── doc/                      # Cargo doc HTML (unchanged)
│   └── serde/
│       └── index.html
└── docmd/                    # NEW STRUCTURE
    └── serde/
        └── index.md          # Single markdown file
```

### Module Exports

```rust
// src/html2md.rs (NEW)
//! HTML to Markdown conversion using scraper.
//!
//! This module provides functions to convert HTML strings to markdown
//! by extracting the <main> element content and converting it to markdown.

use scraper::{Html, Selector};
use crate::error::Result;

/// Convert HTML string to markdown by extracting main element content.
///
/// This function parses the HTML, extracts the content within the <main>
/// element, and converts it to markdown format.
pub fn convert(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("main").map_err(|e| {
        // Convert selector parse error to appropriate error type
        crate::error::HtmlExtractError::SelectorParseFailed {
            selector: "main".to_string(),
            error: e.to_string(),
        }
    })?;

    let main_element = document
        .select(&selector)
        .next()
        .ok_or_else(|| {
            crate::error::HtmlExtractError::ElementNotFound {
                selector: "main".to_string(),
            }
        })?;

    let mut markdown = String::new();
    convert_node(main_element, &mut markdown, 0);
    Ok(markdown)
}

/// Recursively convert HTML nodes to markdown.
///
/// This function walks through the HTML node tree and converts each element
/// to its markdown equivalent, handling nested elements appropriately.
fn convert_node(node: &scraper::element_ref::ElementRef, output: &mut String, depth: usize) {
    // Implementation handles various HTML element types:
    // - <h1>-<h6> → # to ###### headings
    // - <p> → paragraph text with newline
    // - <code> → inline code with backticks
    // - <pre><code> → code blocks with triple backticks
    // - <a> → markdown links [text](url)
    // - <ul>/<ol> → bullet/numbered lists
    // - <li> → list items with proper indentation
    // - <strong>/<b> → **bold** text
    // - <em>/<i> → _italic_ text
    // - <blockquote> → quoted text with >
    // Handles nested elements recursively
}
```

### Build Command Flow (After Changes)

```rust
// src/commands/build.rs
pub fn build(crate_name: String) -> error::Result<()> {
    // Get metadata (unchanged)
    let metadata = cargo::metadata()?;

    // Validate crate (unchanged)
    let dependency = /* ... */;

    // Generate HTML and get directory path (MODIFIED)
    let html_dir = cargo::doc(
        &crate_name,
        &metadata.target_directory,
        &feature_refs,
        dependency.uses_default_features,
    )?;

    // Read HTML file and convert to markdown (NEW)
    let html_path = html_dir.join("index.html");
    let html_content = std::fs::read_to_string(&html_path)?;
    let markdown_content = html2md::convert(&html_content)?;

    // Create output directory (MODIFIED path structure)
    let output_dir = metadata.target_directory
        .join("docmd")
        .join(&crate_name);
    std::fs::create_dir_all(&output_dir)?;

    // Write index.md (NEW)
    let index_path = output_dir.join("index.md");
    std::fs::write(&index_path, markdown_content)?;

    println!("Generated markdown: {}", index_path.display());

    Ok(())
}
```

## Design Considerations

### 1. HTML to Markdown Implementation Approach

**Decision**: Implement custom HTML-to-markdown conversion using `scraper`.

- **Alternative**: Add `html2md` or `html2text` crate
    - Rejected: Adds unnecessary dependency, `scraper` is already sufficient
- **Alternative**: Use complex HTML-to-markdown library
    - Rejected: Overkill for our needs, cargo doc HTML has predictable structure
- **Alternative**: Regex-based conversion
    - Rejected: HTML parsing with regex is error-prone and fragile
- **Resolution**: Implement recursive converter using `scraper` for clean,
  maintainable code without additional dependencies

### 2. Main Element Extraction

**Decision**: Use `scraper` crate (already a dependency) to extract `<main>`.

- **Alternative**: Use regex to extract main content
    - Rejected: HTML parsing with regex is error-prone
- **Alternative**: Parse entire document and ignore non-main elements
    - Rejected: More complex than just extracting main element
- **Resolution**: Use existing `scraper` dependency with CSS selector "main"

### 3. Output Structure

**Decision**: Single `index.md` file in `target/docmd/<crate>/`.

- **Alternative**: Keep multiple files for different items
    - Rejected: Defeats the purpose of simplified approach
- **Alternative**: Put `index.md` directly in `target/docmd/`
    - Rejected: Would clash when building multiple crates
- **Alternative**: Use different directory name than `docmd`
    - Rejected: `docmd` is already established in documentation
- **Resolution**: Follow pattern: `target/docmd/<crate>/index.md`

### 4. Cargo Doc Output Parsing

**Decision**: Parse stdout to find "Generated" line using regex and extract
directory.

- **Alternative**: Assume standard path and construct it
    - Rejected: Paths can vary based on cargo configuration
- **Alternative**: Use cargo metadata to determine path
    - Rejected: More complex, parsing output is simpler
- **Alternative**: Search filesystem for most recent HTML
    - Rejected: Unreliable and race-prone
- **Resolution**: Parse "Generated /path/to/crate/index.html" line with regex,
  extract directory path

### 5. Handling of Existing `items` Module

**Decision**: Delete `items` module entirely.

- **Alternative**: Keep `items` module but mark as deprecated or unused
    - Rejected: Increases codebase complexity without providing value
- **Alternative**: Keep and maintain both approaches
    - Rejected: Increases complexity and maintenance burden
- **Alternative**: Move to separate branch or tag
    - Rejected: Git history preserves old approach if needed for reference
- **Resolution**: Delete entire `src/items` directory since new approach
  replaces it

### 6. Markdown Post-Processing

**Decision**: Start with direct conversion, add cleanup if needed.

- **Alternative**: Implement extensive cleanup immediately
    - Rejected: Hard to anticipate all cleanup needs without testing
- **Alternative**: Pre-define cleanup rules
    - Rejected: May not match actual cargo doc output structure
- **Resolution**: Generate initial output, evaluate quality, add targeted
  cleanup later

## Success Criteria

- [ ] Build command successfully generates `target/docmd/<crate>/index.md`
- [ ] Generated markdown contains content from `<main>` element of cargo doc
      HTML
- [ ] Markdown is readable and preserves structure (headings, code blocks,
      lists)
- [ ] Error messages are clear and helpful
- [ ] All unit tests pass
- [ ] Integration test passes with real crate
- [ ] Manual testing with serde produces useful markdown
- [ ] Documentation (README.md and DOCS.md) is updated accurately
- [ ] No compiler warnings
- [ ] Code follows existing Rust guidelines in AGENTS.md
- [ ] No additional dependencies added (uses existing `scraper` crate)
- [ ] `src/items` directory and all its contents are removed from the codebase
- [ ] All references to `crate::items` are removed from the code

## Implementation Status: ✅ COMPLETED

## Implementation Notes

Space for recording specific technical details or roadblocks encountered during
work.
