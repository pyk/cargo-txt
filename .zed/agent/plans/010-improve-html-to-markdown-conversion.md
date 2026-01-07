---
type: normal
title: "Improve HTML to Markdown conversion"
seq: 010
slug: improve-html-to-markdown-conversion
created: "2026-01-07T16:33:48Z"
status: completed
---

# Improve HTML to Markdown conversion

Enhance the HTML-to-Markdown converter to handle rustdoc-specific HTML patterns
correctly, fixing spacing, link rendering, and table/list conversion issues.

## Current Problems

The `html2md.rs` module has several issues when converting rustdoc HTML to
markdown:

### 1. H1 Heading Issues

**Current output:** `# Craterustdoc_typesCopy item path`

**Expected output:** `# Crate rustdoc_types`

**Root cause:**

```html
<h1>
    Crate <span>rustdoc_<wbr />types</span>&nbsp;<button id="copy-path">
        Copy item path
    </button>
</h1>
```

- No space added between text nodes
- `<wbr />` elements not handled
- Button with `id="copy-path"` is rendered as text
- Non-breaking space (`&nbsp;`) not properly handled

### 2. Docblock Toolbar Issues

**Current output:**
`[Source](../src/rustdoc_types/lib.rs.html#1-1465)Expand descriptionRustdoc's JSON output interface`

**Expected output:** `Rustdoc's JSON output interface`

**Root cause:**

```html
<rustdoc-toolbar></rustdoc-toolbar>
<span class="sub-heading">
    <a class="src" href="../src/rustdoc_types/lib.rs.html#1-1465">Source</a>
</span>
<summary class="hideme">
    <span>Expand description</span>
</summary>
```

- `<a class="src">` links are converted to markdown links
- `<summary class="hideme">` elements are rendered

### 3. Inline Code Spacing Issues

**Current output:** `...through the `--output-format
json`flag. The [ `Crate`](struct.Crate.html)struct...`

**Expected output:** `...through the `--output-format
json`flag. The`Crate` struct...`

**Root cause:**

```html
<p>
    These types are the public API exposed through the
    <code>--output-format json</code> flag. The
    <a href="struct.Crate.html" title="struct rustdoc_types::Crate">
        <code>Crate</code>
    </a>
    struct is the root...
</p>
```

- No space added after inline code closing backtick
- `<a>` tags containing `<code>` are rendered as markdown links instead of just
  the code

### 4. H2 Anchor Links

**Current output:** `## Structs [§](#structs)`

**Expected output:** `## Structs`

**Root cause:**

```html
<h2 id="structs" class="section-header">
    Structs<a href="#structs" class="anchor">§</a>
</h2>
```

- `<a class="anchor">` is rendered as markdown link

### 5. Definition List Not Supported

**Current output:** Giant single line concatenating all items

**Expected output:** Readable markdown list

**Root cause:**

```html
<dl class="item-table">
    <dt>
        <a class="struct" href="struct.AssocItemConstraint.html">
            Assoc<wbr />Item<wbr />Constraint
        </a>
    </dt>
    <dd>Describes a bound applied to an associated type/constant.</dd>
    ...
</dl>
```

- `<dl>`, `<dt>`, `<dd>` elements not handled
- Results in concatenated text without proper formatting

### 6. Link Rendering Issue

**Current behavior:** All `<a>` tags converted to markdown links `[text](url)`

**Expected behavior:** Render inner content only (no markdown link format)

**Examples:**

```html
<!-- Should render as `Crate` -->
<a href="struct.Crate.html"><code>Crate</code></a>

<!-- Should render as Something -->
<a href="struct.Crate.html">Something</a>
```

## Proposed Solution

1. Add a helper function to check if a node should be skipped based on
   attributes
2. Update `convert_node` to skip nodes with specific IDs or classes
3. Fix spacing around inline code elements
4. Change `<a>` tag handling to render inner content only
5. Add support for definition lists (`<dl>`, `<dt>`, `<dd>`)
6. Handle `<wbr />` elements (ignore them)
7. Handle `<rustdoc-toolbar>` elements (ignore them)
8. Add comprehensive tests with real rustdoc HTML snippets

## Analysis Required

### Dependency Investigation

- [ ] Verify scraper crate API for accessing element attributes (id, class)
- [ ] Check how to properly handle text node concatenation with spacing
- [ ] Verify if any other rustdoc-specific elements need special handling

### Code Locations to Check

- `src/html2md.rs` - Main conversion logic
- `src/html2md.rs` - `convert_node` function (needs refactoring)
- `src/html2md.rs` - `convert_children` function (needs spacing logic)
- `src/error.rs` - No changes needed (existing error types sufficient)

## Implementation Checklist

### Code Changes

- [x] Add `should_skip_node` helper function to check node attributes
    - Skip if `id="copy-path"`
    - Skip if `class="src"`
    - Skip if `class="hideme"`
    - Skip if `class="anchor"`
    - Skip if tag is `wbr`
    - Skip if tag is `rustdoc-toolbar`

- [x] Update `convert_node` to call `should_skip_node` and return early if true

- [x] Fix inline code spacing in `convert_node` for `<code>` elements
    - Add space after closing backtick when not followed by whitespace or
      punctuation

- [x] Change `<a>` tag handling in `convert_node`
    - Remove markdown link format `[text](url)`
    - Render only inner content (recursively process children)

- [x] Add support for definition lists in `convert_node`
    - Handle `<dl>` - container for definition list
    - Handle `<dt>` - definition term (render as bold or list item)
    - Handle `<dd>` - definition description (render indented)

- [x] Add `convert_definition_list` function to handle `<dl>` elements

- [x] Update `convert_children` to handle non-breaking spaces (`&nbsp;`)
    - Convert to regular space

### Test Updates

- [x] Add test for H1 with copy-path button

    ```html
    <main>
        <h1>
            Crate <span>rustdoc_<wbr />types</span>&nbsp;<button id="copy-path">
                Copy item path
            </button>
        </h1>
    </main>
    ```

    Expected: `# Crate rustdoc_types\n\n`

- [x] Add test for docblock with toolbar

    ```html
    <main>
        <rustdoc-toolbar></rustdoc-toolbar
        ><span class="sub-heading"><a class="src" href="...">Source</a></span>
        <summary class="hideme"><span>Expand description</span></summary>
        <p>Rustdoc's JSON output interface</p>
    </main>
    ```

    Expected: `Rustdoc's JSON output interface\n\n`

- [x] Add test for inline code spacing and links

    ```html
    <main>
        <p>
            Through the <code>--output-format json</code> flag. The
            <a href="struct.Crate.html"><code>Crate</code></a> struct.
        </p>
    </main>
    ```

    Expected:

    ```
    Through the `--output-format json` flag. The `Crate` struct.\n\n
    ```

- [x] Add test for H2 with anchor

    ```html
    <main>
        <h2 id="structs" class="section-header">
            Structs<a href="#structs" class="anchor">§</a>
        </h2>
    </main>
    ```

    Expected: `## Structs\n\n`

- [x] Add test for definition list (simplified version)

    ```html
    <main>
        <dl class="item-table">
            <dt><a href="struct.Crate.html">Crate</a></dt>
            <dd>The root.</dd>
            <dt><a href="struct.Enum.html">Enum</a></dt>
            <dd>An enum.</dd>
        </dl>
    </main>
    ```

    Expected: Properly formatted definition list

- [x] Add comprehensive test with full real-world HTML snippet for structs table

### Documentation Updates

- [x] Update `DOCS.md` section on "HTML to Markdown Conversion" to reflect new
      capabilities
- [x] Document new definition list support
- [x] Document node skipping behavior (which elements are skipped and why)
- [x] Document link rendering change (inner content only)

## Test Plan

### Verification Tests

- [x] Verify H1 rendering: `# Crate rustdoc_types` with no button text
- [x] Verify docblock rendering: Clean text without Source link or Expand
      description
- [x] Verify inline code spacing: Space after closing backtick
- [x] Verify link rendering: Inner content only, no markdown link syntax
- [x] Verify H2 rendering: No anchor link text
- [x] Verify definition list rendering: Proper list format with terms and
      descriptions
- [x] Verify all existing tests still pass

### Regression Tests

- [x] Test with existing simple HTML snippets (headings, paragraphs, code
      blocks, etc.)
- [x] Test with nested elements to ensure spacing is consistent
- [x] Test with multiple definition lists in one document
- [x] Test with mixed content (definition lists + regular lists)

## Structure After Changes

### Modified File

`src/html2md.rs` - Enhanced HTML to Markdown converter with:

- New `should_skip_node` function
- Updated `convert_node` with skip logic and new element handling
- New `convert_definition_list` function
- Updated test suite with 6+ new test cases

### Key Function Signatures

```rust
// Helper function to check if node should be skipped
fn should_skip_node(node: ElementRef) -> bool {
    // Check for specific ids, classes, or tag names
}

// Updated convert_node with new cases
fn convert_node(node: ElementRef, output: &mut String) {
    // Add handling for: dl, dt, dd, wbr, rustdoc-toolbar
    // Update a tag handling to render inner content only
}

// New function for definition lists
fn convert_definition_list(node: ElementRef, output: &mut String) {
    // Process dt and dd elements
}
```

## Design Considerations

1. **Node skipping strategy**: Should we skip at the element level or text
   level?
    - **Decision**: Skip at element level in `convert_node` before processing
      children
    - **Alternative**: Skip during text processing in `convert_children`
    - **Resolution**: Element-level skipping is cleaner and prevents unnecessary
      recursion

2. **Link rendering change**: Should we preserve href attributes for future use?
    - **Decision**: No, render inner content only as requested
    - **Alternative**: Store href in a comment or data attribute
    - **Resolution**: Keep it simple - render inner content only

3. **Definition list format**: How should we render `<dl>` in markdown?
    - **Decision**: Render as nested list with terms as bold items and
      descriptions indented
    - **Alternative**: Use definition list syntax if supported, or custom format
    - **Resolution**: Use standard markdown nested lists for compatibility:
        ```
        - **Term**: Description
        ```

4. **Spacing strategy**: When to add spaces around inline elements?
    - **Decision**: Add space after inline code if next character is
      alphanumeric
    - **Alternative**: Always add space or never add space
    - **Resolution**: Conditional spacing based on context for natural text flow

## Success Criteria

- H1 renders as `# Crate rustdoc_types` without button text
- Docblock renders clean without Source link or Expand description
- Inline code has proper spacing after closing backtick
- Links render inner content only (no markdown link syntax)
- H2 renders without anchor link text
- Definition lists render as readable markdown lists
- All existing tests pass
- All new tests pass with real rustdoc HTML snippets
- No regression in existing HTML-to-Markdown conversion

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

The user provided extensive raw HTML snippets. These should be used as test
cases to ensure the fixes work correctly with real rustdoc output. The test
cases should cover:

1. Full H1 with wbr elements and copy-path button
2. Full docblock with toolbar, source link, and expand description
3. Full paragraph with inline code and nested links
4. Full H2 with anchor link
5. Complete definition list from the structs table

### Additional Edge Cases Discovered During Implementation

During implementation and testing, several additional edge cases were discovered
and addressed:

#### Markdown Reference-Style Links

**Raw HTML:**

```html
<li>Derive [tutorial][_derive::_tutorial] and [reference][_derive]</li>
```

**Issue:** Reference-style links like `[text][reference]` were being rendered
as-is, creating non-standard markdown syntax.

**Solution:** Implemented `process_text_links()` function that detects and
converts `[text][reference]` patterns to just `text`, removing the reference
identifier.

**Output:** `- Derive tutorial and reference`

#### Code Block Formatting

**Raw HTML:**

```html
<div class="example-wrap">
    <pre class="language-console">
        <code>$ cargo add clap --features derive</code>
    </pre>
</div>
```

**Issue:** Code blocks weren't properly formatted with a newline after the
opening fence.

**Solution:** Added newline after opening ` ``` ` in `<pre>` element handling to
ensure proper markdown code block format.

**Output:**

```

```

$ cargo add clap --features derive

```

```

#### Script Tags with JSON Data

**Raw HTML:**

```html
<script type="text/json" id="notable-traits-data">
    {
        "&<Vec<T, A> as Index<I>>::Output": "..."
    }
</script>
```

**Issue:** Script tags containing JSON data were being rendered as text,
creating garbage output in the markdown.

**Solution:** Added `"script"` to the `should_skip_node()` function to
completely skip script tags during conversion.

**Output:** Script content is not rendered at all.

#### Whitespace Normalization

**Raw HTML:**

```html
<p>
    These types are the public API exposed through the
    <code>--output-format json</code> flag. The
    <a href="struct.Crate.html">
        <code>Crate</code>
    </a>
    struct is the root...
</p>
```

**Issue:** HTML formatting (indentation, newlines) was being preserved in the
markdown output, creating multi-line paragraphs and definition items.

**Solution:** Implemented `convert_children_normalized()` function that:

- Collects all child content into a buffer
- Normalizes whitespace by splitting on whitespace and joining with single
  spaces
- This effectively collapses multiple spaces, tabs, and newlines into single
  spaces
- Applied to all block-level elements: headings, paragraphs, list items,
  definition terms, and definition descriptions

**Output:** Single-line text with proper spacing

This ensures the converter handles all edge cases present in actual rustdoc HTML
output.
