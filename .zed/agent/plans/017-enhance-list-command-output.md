---
type: normal
title: "Enhance list command output"
seq: 017
slug: "enhance-list-command-output"
created: "2025-01-09T12:00:00Z"

status: completed
---

# Enhance list command output

Enhance the `cargo txt list` command output to include crate name as the
top-level heading, prefix all items with the crate name, add usage instructions
for showing specific items, generate dynamic usage examples using actual items
from the crate, and fix error message formatting bugs.

## Current Problems

When running `cargo txt list serde`, the output lacks three important pieces of
information:

1. **Missing crate name header**: The output starts with "# List of all items"
   instead of showing the crate name as the primary heading.

2. **Missing crate name prefix on items**: Items are listed without the crate
   name prefix (e.g., `de::IgnoredAny` instead of `serde::de::IgnoredAny`).

3. **Missing usage instructions**: Users don't know how to view documentation
   for specific items from the list.

### Additional Improvements Needed

4. **Generic usage examples**: The current usage examples use placeholder names
   like `SomeStruct`, `SomeTrait`, which are not helpful to users.

5. **Error message formatting bug**: When users run
   `cargo txt show rustdoc-types::Abi` (with hyphen), the error message
   incorrectly shows `rustdoc_types::Abi` (with underscore), losing the original
   input format.

Example current output:

```markdown
# List of all items

### Structs

- de::IgnoredAny
- de::value::BoolDeserializer
```

## Proposed Solution

Modify the build command to post-process `all.md` after converting from
`all.html` to:

1. Add crate name as H1 heading at the top
2. Prefix all list items with crate name
3. Add usage instructions section at the bottom with dynamic examples
4. Fix error message formatting in `show` command to preserve input format

## Analysis Required

### Code Locations to Check

- `src/commands/build.rs` - Identify where `all.md` is written and add
  post-processing logic

### Existing HTML Structure to Understand

- Examine `all.html` structure to confirm item format and ensure prefixing logic
  handles all cases correctly

### Error Message Bug Investigation

- Locate error handling in `src/commands/show.rs`
- Identify where item path normalization occurs
- Ensure error messages preserve the original user input format

## Implementation Checklist

### Code Changes

- [x] Add `format_all_md` function in `src/commands/build.rs` that:
    - Takes crate name and raw markdown content
    - Converts existing H1 "# List of all items" to a paragraph (removes `# `
      prefix)
    - Inserts crate name as H1 at the start
    - Prefixes all list items (`- text`) with crate name
    - Extracts first item from each section for usage examples
    - Appends usage instructions with dynamic examples at the end
- [x] Update `build` function in `src/commands/build.rs` to call `format_all_md`
      before writing `all.md`
- [x] Add unit tests for `format_all_md` covering:
    - Crate name H1 insertion
    - Conversion of existing H1 "# List of all items" to paragraph
    - Simple item prefixing (e.g., "Error" -> "serde::Error")
    - Nested item prefixing (e.g., "de::IgnoredAny" -> "serde::de::IgnoredAny")
    - Multiple sections (structs, traits, enums, macros)
    - Dynamic usage example generation with actual items
    - Usage instructions appended at end
- [x] Locate and fix error message formatting bug in `src/commands/show.rs`
- [x] Ensure error messages preserve the original user input format (hyphens
      remain hyphens, underscores remain underscores)

### Documentation Updates

- [x] Update `DOCS.md` List Command section to describe the new output format
      with H1 crate name and prefixed items
- [x] Update `README.md` List Command section to show the enhanced output format
      with usage instructions
- [x] Update examples in both files to reflect the crate name prefix in item
      paths

### Test Updates

- [x] Verify existing tests still pass after changes
- [x] Test manually by running `cargo txt list serde` and confirming output
      format
- [x] Test dynamic usage examples with `cargo txt list rustdoc-types`
- [x] Test error message preservation with `cargo txt show rustdoc-types::Abi`

## Test Plan

### Verification Tests

- [x] Run `cargo txt list serde` and verify output starts with `# serde`
- [x] Verify all items have `serde::` prefix (e.g., `serde::de::IgnoredAny`)
- [x] Verify usage instructions appear at the bottom with actual crate items
      (not generic placeholders)
- [x] Run `cargo txt list rustdoc-types` and verify examples show actual items
      like `rustdoc_types::AssocItemConstraint`
- [x] Run `cargo txt list` on multiple crates to ensure formatting works
      consistently
- [x] Run `cargo test` to ensure all unit tests pass
- [x] Run `rust-lint` to ensure coding guidelines are followed

### Regression Tests

- [x] Ensure `cargo txt show` still works correctly with prefixed item paths
- [x] Ensure build command generates all other files correctly (index.md,
      individual item files)
- [x] Test error message format preservation: Run
      `cargo txt show rustdoc-types::NonExistentItem` and verify error shows
      `rustdoc-types::NonExistentItem` (not `rustdoc_types::NonExistentItem`)

## Design Considerations

1. **Where to add formatting logic**: Should be in build command, not list
   command, so that `all.md` is complete and self-contained for potential future
   use by other commands.
    - **Alternative**: Format in list command when displaying.
    - **Resolution**: Format in build command for consistency and reusability.

2. **Item prefixing strategy**: Items in `all.md` are in markdown list format
   (`- text`). Need to identify lines starting with `-` and prefix them.
    - Edge case: Items may already have module paths (e.g., `de::IgnoredAny`).
      Need to preserve these and only add crate prefix.
    - Edge case: Items may be simple names without modules (e.g., `Serialize`).

3. **Dynamic usage example generation**: Extract first item from each section to
   create concrete examples.
    - Parse markdown sections (### Structs, ### Traits, ### Enums, etc.)
    - Extract first list item from each non-empty section
    - Prioritize sections: structs, traits, enums, constants
    - Handle edge case where some sections may be empty
    - Examples should be simple commands without descriptive text

4. **Error message format preservation**: Ensure user input is preserved in
   error messages.
    - Locate where item path normalization occurs in show command
    - Store original user input separately from normalized version
    - Use original input in error messages
    - Test with various crate names containing hyphens, underscores, etc.

## Success Criteria

- `cargo txt list serde` output starts with `# serde` as the first heading
- Original H1 "# List of all items" is converted to a paragraph
- All list items are prefixed with `serde::`
- Usage instructions appear at the bottom with actual crate items as examples
  (not generic placeholders like `SomeStruct`)
- `cargo txt list rustdoc-types` shows examples like:
  `cargo txt show rustdoc_types::AssocItemConstraint`
- Error messages preserve input format: `cargo txt show rustdoc-types::Abi`
  shows error with `rustdoc-types::Abi` (not `rustdoc_types::Abi`)
- All existing tests pass
- New unit tests for `format_all_md` pass
- New unit tests for error message format preservation pass
- `rust-lint` passes to ensure coding guidelines are followed

## Implementation Status: 🟢 COMPLETED

## Updates Made

- Added dynamic usage example generation requirement
- Added error message format preservation requirement
- Updated implementation checklist with new tasks
- Updated test plan with new verification and regression tests
- Updated design considerations for dynamic examples and error handling
- Updated success criteria to include concrete examples and error format
- Updated implementation notes with dynamic template and error fix details

## Implementation Completed

All implementation tasks have been completed successfully:

1. **Dynamic Usage Examples**: Modified `format_all_md` function to extract the
   first item from each section (Structs, Traits, Enums, etc.) and use them as
   concrete examples in the usage instructions. The function now properly tracks
   sections and collects the first list item from each.

2. **Error Message Format Preservation**: Fixed the error message bug in
   `show.rs` by storing the original user input format separately from the
   normalized path. Error messages now preserve the exact format the user
   provided (hyphens remain hyphens, underscores remain underscores).

3. **Unit Tests**: Added comprehensive tests for both features:
    - Tests for dynamic example generation with multiple sections
    - Tests for fallback when no sections exist
    - Tests for error message format preservation with both hyphen and
      underscore crate names

4. **Manual Testing**: Verified functionality with real crates:
    - `cargo txt list serde` shows dynamic examples from actual crate items
    - `cargo txt list rustdoc-types` shows examples like
      `rustdoc_types::AssocItemConstraint`
    - `cargo txt show rustdoc-types::NonExistentItem` displays error with
      `rustdoc-types::NonExistentItem` (preserving hyphen)

All 66 unit tests pass successfully.

## Implementation Notes

The `format_all_md` function should:

1. Convert existing H1 "# List of all items" to a paragraph by removing the `# `
   prefix (first line only)
2. Insert crate name as new H1 at the start: `# {crate_name}`
3. Use regex or string parsing to find lines starting with `- ` and prepend
   crate name
4. Extract first items from sections (Structs, Traits, Enums, Constants, etc.)
5. Generate usage examples using extracted items
6. Append usage instructions in markdown format with concrete examples
7. Handle edge cases like empty lines, non-list lines, and empty sections

Usage instructions template (with dynamic examples):

````markdown
## Usage

To view documentation for a specific item, use the `show` command:

```shell
cargo txt show <ITEM_PATH>
```

Examples:

```shell
cargo txt show rustdoc_types::AssocItemConstraint
cargo txt show rustdoc_types::Abi
cargo txt show rustdoc_types::FORMAT_VERSION
```
````

Error message fix in `show` command:

1. Locate where item path resolution fails and error is generated
2. Ensure the error message uses the original user input, not normalized version
3. Test with crate names containing hyphens (e.g., `rustdoc-types`)
