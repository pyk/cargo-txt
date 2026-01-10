---
type: normal
title: "Fix crate item markdown rendering bugs"
seq: 019
slug: "fix-crate-item-markdown-rendering"
created: "2026-01-10T23:10:57Z"
status: completed
---

# Fix crate item markdown rendering bugs

Fix several bugs in the HTML to Markdown conversion for crate items where
specific rustdoc elements are incorrectly rendered in the markdown output.

## Current Problems

The `should_skip_node` function in `src/html2md.rs` does not skip several
rustdoc elements that should not appear in the markdown output.

### Problem 1: Breadcrumbs are rendered in item header

The first line of crate items incorrectly includes the crate name followed by
the item type, resulting in:

```markdown
serde# Trait Deserializer
```

This comes from the `div.rustdoc-breadcrumbs` element:

```html
<div class="main-heading">
    <div class="rustdoc-breadcrumbs">
        <a href="index.html">serde</a>
    </div>
    <h1>
        Trait
        <span class="trait">Deserializer</span>&nbsp;<button
            id="copy-path"
            title="Copy item path to clipboard"
        >
            Copy item path
        </button>
    </h1>
</div>
```

### Problem 2: Tooltip icons are rendered as ASCII characters

The tooltip icon appears in the markdown output:

````markdown
ⓘ```
````

This comes from the `a.tooltip` element:

```html
<a href="#" class="tooltip" title="This example runs with edition 2021">ⓘ</a>
```

### Problem 3: Implementors sections are rendered

The implementors section should not appear in the markdown but is currently
rendered:

```html
<h2 id="implementors" class="section-header">
    Implementors<a href="#implementors" class="anchor">§</a>
</h2>
<div id="implementors-list">... many text here</div>
```

## Proposed Solution

Update the `should_skip_node` function to skip elements with specific classes
and IDs, and add test cases to prevent regression.

## Analysis Required

### Code Locations to Check

- `src/html2md.rs` - `should_skip_node` function and test module

## Implementation Checklist

### Code Changes

- [x] Update `should_skip_node` function to skip elements with class
      "rustdoc-breadcrumbs"
- [x] Update `should_skip_node` function to skip elements with class "tooltip"
- [x] Update `should_skip_node` function to skip elements with id "implementors"
- [x] Update `should_skip_node` function to skip elements with id
      "implementor-list"

### Test Updates

- [x] Add test `convert_rustdoc_breadcrumbs_skipped` using the raw breadcrumbs
      HTML
- [x] Add test `convert_tooltip_skipped` using the raw tooltip HTML
- [x] Add test `convert_implementors_section_skipped` using the raw implementors
      HTML
- [x] Add test `convert_implementors_list_skipped` using the raw
      implementors-list HTML
- [x] Add test `convert_combined_rustdoc_elements_skipped` to verify all
      elements are skipped together

### Verification

- [x] Run `cargo test` to ensure all tests pass
- [x] Run `rust-lint` to check coding conventions
- [x] Run `cargo clippy -- -D warnings` to ensure no warnings
- [x] Run `cargo build` to ensure the project compiles

## Test Plan

### Verification Tests

- [x] Verify breadcrumbs are not rendered in markdown output
- [x] Verify tooltip icons are not rendered in markdown output
- [x] Verify implementors section is not rendered in markdown output
- [x] Verify implementors list is not rendered in markdown output

### Regression Tests

- [x] Ensure all existing tests still pass
- [ ] Test with real crate documentation (e.g., serde crate)

## Structure After Changes

The `should_skip_node` function will be updated to include additional class and
ID checks.

### Before

```rust
fn should_skip_node(node: ElementRef) -> bool {
    let elem = node.value();

    match elem.name() {
        "wbr" | "rustdoc-toolbar" | "script" => return true,
        _ => {}
    }

    match elem.attr("id") {
        Some("copy-path") => return true,
        _ => {}
    }

    let should_skip_class = match elem.attr("class") {
        Some(class) => {
            class.contains("src") || class.contains("hideme") || class.contains("anchor")
        }
        None => false,
    };
    if should_skip_class {
        return true;
    }

    false
}
```

### After

```rust
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
            class.contains("src") || class.contains("hideme") || class.contains("anchor")
                || class.contains("rustdoc-breadcrumbs") || class.contains("tooltip")
        }
        None => false,
    };
    if should_skip_class {
        return true;
    }

    false
}
```

## Design Considerations

1. **Selector efficiency**: Using class string checks is efficient and works
   well for our use case.
2. **Future-proofing**: Adding these checks now prevents future regressions as
   rustdoc evolves.
3. **Test coverage**: Using the actual raw HTML provided ensures we test
   real-world scenarios.

## Success Criteria

- Breadcrumbs no longer appear in markdown output
- Tooltip icons no longer appear in markdown output
- Implementors section and list no longer appear in markdown output
- All existing tests pass (approximately 20+ existing tests)
- New tests pass (5 new tests)
- `cargo test` passes without errors
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds

## Implementation Status: 🟢 COMPLETE

## Implementation Notes

### Changes Made

1. **Updated `should_skip_node` function** (lines 37-61):
    - Added `Some("implementors")` and `Some("implementors-list")` to the ID
      match statement
    - Added `class.contains("rustdoc-breadcrumbs")` and
      `class.contains("tooltip")` to the class checks

2. **Added 5 new test cases** (lines 607-687):
    - `convert_rustdoc_breadcrumbs_skipped`: Verifies breadcrumb elements are
      skipped
    - `convert_tooltip_skipped`: Verifies tooltip icons are skipped
    - `convert_implementors_section_skipped`: Verifies implementors section
      header is skipped
    - `convert_implementors_list_skipped`: Verifies implementors list div is
      skipped
    - `convert_combined_rustdoc_elements_skipped`: Verifies all elements are
      skipped together

### Test Results

All 54 tests passed, including the 5 new tests:

- `convert_rustdoc_breadcrumbs_skipped` ✓
- `convert_tooltip_skipped` ✓
- `convert_implementors_section_skipped` ✓
- `convert_implementors_list_skipped` ✓
- `convert_combined_rustdoc_elements_skipped` ✓

### Verification

- ✓ `cargo test` passed (54 tests, 0 failed)
- ✓ `rust-lint` passed (coding conventions verified)
- ✓ `cargo clippy -- -D warnings` passed (no warnings)
- ✓ `cargo build` succeeded

### Implementation Details

The implementation followed the existing pattern in the codebase:

- Used match statements for ID checks
- Used class string contains checks for classes
- Used raw string literals with `r##"..."##` for HTML containing `#` characters
- Followed the existing test pattern for similar skip tests (e.g.,
  `convert_script_tag_skipped`)
