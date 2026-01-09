---
type: normal
title: "Refactor Error Messages to be LLM-Friendly"
seq: 014
slug: "refactor-error-messages-for-llm"
created: "2026-01-08T13:06:37Z"
status: not_started
---

# Refactor Error Messages to be LLM-Friendly

Refactor error reporting in cargo-txt to optimize for LLMs and coding agents as
the primary users. Error messages should be direct, actionable, and structured
for automated parsing. Combine HTML-related errors into a single `HtmlError`
enum following the existing pattern.

## Current Problems

**Problem 1: Error messages designed for humans, not LLMs**

Current error messages include conversational elements and formatting that
humans find friendly but LLMs don't need:

```rust
// src/error.rs - Current implementation
BuildError::InvalidCrateName {
    requested,
    available,
} => {
    write!(
        f,
        "Crate '{}' is not an installed dependency.\n\nAvailable crates: {}\n\nOnly installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.",
        requested,
        available.join(", ")
    )
}
```

LLM-friendly format should be:

```
Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Add the crate to Cargo.toml as a dependency first.
```

**Problem 2: HTML errors are not consolidated**

The top-level `Error` enum has two separate HTML error variants:

```rust
// src/error.rs - Current implementation
pub enum Error {
    Build(BuildError),
    Show(ShowError),
    HtmlSelectorParseFailed { selector: String, error: String },
    HtmlElementNotFound { selector: String },
}
```

This breaks the pattern established by `Build(BuildError)` and
`Show(ShowError)`. HTML errors should be grouped under `Html(HtmlError)`.

**Problem 3: Error messages lack clear actionable steps**

Some errors don't tell the LLM what to do next. For example:

```rust
// Current message - doesn't provide clear action
"Failed to execute cargo doc for crate 'crate_name':\n{output}"
```

Better format for LLMs:

```
Failed to execute cargo doc for crate 'crate_name':
{output}

Fix the issue preventing cargo doc from running, then retry the command.
```

## Proposed Solution

1. Create a new `HtmlError` enum with variants for selector parse failures and
   element not found
2. Add `Html(HtmlError)` variant to top-level `Error` enum
3. Remove `HtmlSelectorParseFailed` and `HtmlElementNotFound` variants from
   `Error` enum
4. Redesign all error messages to be LLM-friendly:
    - Be direct and specific
    - Include exact file paths when relevant
    - Provide clear next steps/actions
    - Remove conversational filler
    - Structure for easy parsing
5. Update all error construction sites to use new structure
6. Update tests and documentation

## Error Message Redesign

### BuildError Messages

**CargoDocExecFailed**

```
The cargo doc command failed for crate '{crate_name}':
{output}

This error is from cargo doc, not cargo-txt. Inform the user that cargo doc failed to generate documentation.
```

**CargoMetadataExecFailed**

```
The cargo metadata command failed:
{output}

Inform the user they may not be in a valid Rust project directory or their Cargo.toml is invalid.
```

**InvalidCrateName**

```
Crate '{requested}' is not installed in this project.

Available crates: {available_list}

Inform the user that only installed dependencies can be documented. They can add the crate to Cargo.toml if needed.
```

**OutputDirCreationFailed**

```
Cannot create output directory '{path}': {source}

Inform the user about this permission or directory issue. They need to fix their system permissions.
```

**FileReadFailed**

```
Cannot read file '{path}': {source}

Inform the user that the file does not exist or is not readable. Suggest re-running 'cargo txt build {crate}' if the file should exist.
```

**CargoDocOutputParseFailed**

```
Cannot parse cargo doc output - 'Generated' line not found.

Output preview:
{output_preview}

This is unexpected output from cargo doc. Inform the user that cargo doc produced unexpected output.
```

**DocIndexNotFound** (BuildError variant)

```
Documentation index file '{path}' not found: {source}

The documentation was not generated or was deleted. Suggest the user run 'cargo txt build {crate}' again.
```

**MarkdownWriteFailed**

```
Cannot write markdown file '{path}': {source}

Inform the user about disk space or write permission issues. This is a system problem they need to resolve.
```

### ShowError Messages

**DocIndexNotFound** (ShowError variant)

```
Documentation index file '{path}' not found: {source}

The documentation has not been built. Suggest the user run 'cargo txt build {crate}' first.
```

**InvalidItemPath**

```
The item path '{item_path}' is invalid. Expected format: <crate> or <crate>::<item> (e.g., 'serde' or 'serde::Error').

Inform the user of the correct format and ask for a valid item path.
```

**MarkdownNotFound**

```
Markdown file '{path}' not found: {source}

The documentation for this item was not generated. Suggest the user run 'cargo txt build {crate}' to generate all documentation.
```

**ItemPathResolutionFailed**

```
Item path '{item_path}' was not found in the documentation.

Inform the user that this item does not exist in the crate's public API. Suggest running 'cargo txt show {crate}' to see all available items.
```

**Note:** The `attempted_paths` field has been removed from this error variant
because it is always empty in the current implementation. The item path is
resolved using a direct lookup in the item mappings, so there are no actual
"attempted paths" to report.

### HtmlError Messages (New)

**SelectorParseFailed**

```
Internal error: CSS selector '{selector}' is invalid: {error}

Processing crate: {crate_name}
HTML file: {html_path}

This is a bug in cargo-txt. Inform the user that cargo-txt encountered an internal error parsing CSS selectors. They should report this issue with:
- Cargo-txt version
- Rust version: {rust_version}
- Crate being processed: {crate_name}
- HTML file path: {html_path}
```

**ElementNotFound**

```
Internal error: HTML element not found with selector '{selector}'.

Processing crate: {crate_name}
HTML file: {html_path}

This is a bug in cargo-txt. The expected HTML structure from rustdoc is missing. This may indicate:
- The rustdoc HTML format has changed in a new Rust version
- The generated documentation is incomplete or corrupted
- A cargo-txt bug with selector matching

Inform the user that cargo-txt encountered an internal error and they should report this issue with:
- Cargo-txt version
- Rust version: {rust_version}
- Crate being processed: {crate_name}
- HTML file path: {html_path}
```

## Analysis Required

### Dependency Investigation

- [ ] Review all sites where errors are constructed to ensure consistent usage
- [ ] Verify that all error variants are covered in the redesign
- [ ] Check for any error messages in comments or documentation that need
      updating
- [ ] Determine how to obtain `rust_version` for HtmlError context:
    - [ ] Check if cargo metadata provides Rust version
    - [ ] Check if rustdoc HTML contains version information
    - [ ] Consider running `rustc --version` command
- [ ] Determine how to obtain `html_path` for HtmlError context:
    - [ ] Review html2md.rs to see if path is available
    - [ ] Review build.rs to see if path is available in error construction
          sites

### Code Locations to Check

- `src/error.rs` - Update Error enum, create HtmlError enum, update Display
  impls
- `src/commands/build.rs` - Update error construction for HTML errors with
  context
- `src/commands/show.rs` - Verify error messages are consumed correctly
- `src/html2md.rs` - Update error construction to use HtmlError with context
- `src/cargo.rs` - Check for Rust version availability
- `src/main.rs` - Verify error display works correctly

## Implementation Checklist

### Code Changes

- [ ] Implement helper function to get Rust version for HtmlError context:
    - [ ] Add function `get_rust_version() -> error::Result<String>` in
          appropriate module
    - [ ] Consider using `std::process::Command` to run `rustc --version`
    - [ ] Parse version string to extract version number
- [ ] Create `HtmlError` enum in `src/error.rs` with variants:
    - `SelectorParseFailed { selector: String, error: String, crate_name: String, html_path: String, rust_version: String }`
    - `ElementNotFound { selector: String, crate_name: String, html_path: String, rust_version: String }`
- [ ] Implement `fmt::Display` for `HtmlError` with LLM-friendly messages
- [ ] Implement `std::error::Error` for `HtmlError`
- [ ] Implement `fmt::Debug` for `HtmlError` (delegates to Display)
- [ ] Add `Html(HtmlError)` variant to top-level `Error` enum
- [ ] Remove `HtmlSelectorParseFailed` and `HtmlElementNotFound` variants from
      `Error` enum
- [ ] Update `Error::fmt` Display implementation to handle `Html` variant
- [ ] Add `From<HtmlError>` implementation for `Error`
- [ ] Update all `HtmlSelectorParseFailed` constructions in `src/html2md.rs` to
      use `HtmlError::SelectorParseFailed` with context (crate_name, html_path,
      rust_version)
- [ ] Update all `HtmlElementNotFound` constructions in `src/html2md.rs` to use
      `HtmlError::ElementNotFound` with context (crate_name, html_path,
      rust_version)
- [ ] Update all `HtmlSelectorParseFailed` constructions in
      `src/commands/build.rs` to use `HtmlError::SelectorParseFailed` with
      context (crate_name, html_path, rust_version)
- [ ] Update all `HtmlElementNotFound` constructions in `src/commands/build.rs`
      to use `HtmlError::ElementNotFound` with context (crate_name, html_path,
      rust_version)
- [ ] Ensure html_path is available at all error construction sites:
    - [ ] Pass html_path as parameter to functions that may construct HtmlError
    - [ ] Update function signatures in html2md.rs if needed
- [ ] Update `BuildError::Display` implementation with new LLM-friendly messages
- [ ] Update `ShowError::Display` implementation with new LLM-friendly messages
- [ ] Remove `attempted_paths` field from `ShowError::ItemPathResolutionFailed`
      (always empty)
- [ ] Ensure all error messages include actionable next steps where appropriate

### Documentation Updates

- [ ] Update `DOCS.md` section on error handling to reflect new error structure
- [ ] Document the `HtmlError` enum in `DOCS.md`
- [ ] Update error message examples in `DOCS.md` with new LLM-friendly format
- [ ] Update `README.md` error examples if present
- [ ] Document the design principles for LLM-friendly error messages

### Test Updates

- [ ] Update tests in `src/commands/build.rs` that check for
      `HtmlSelectorParseFailed` errors
- [ ] Update tests in `src/commands/build.rs` that check for
      `HtmlElementNotFound` errors
- [ ] Update tests in `src/html2md.rs` that check for HTML parsing errors
- [ ] Update tests in `src/html2md.rs` to include crate name, html_path, and
      rust_version in test cases
- [ ] Add tests for new `HtmlError` variants with all required fields
- [ ] Verify all error messages match the new format in integration tests
- [ ] Run `cargo test` to ensure all tests pass

## Test Plan

### Verification Tests

- [ ] Test `BuildError::InvalidCrateName` displays available crates correctly
- [ ] Test `HtmlError::SelectorParseFailed` displays selector, error message,
      crate name, html path, and rust version
- [ ] Test `HtmlError::ElementNotFound` displays selector, crate name, html
      path, rust version, and actionable steps
- [ ] Test `ShowError::ItemPathResolutionFailed` displays item path and
      actionable steps
- [ ] Verify error messages are machine-parseable (no ambiguous phrasing)
- [ ] Test error output with `cargo txt build` using invalid crate
- [ ] Test error output with `cargo txt show` using invalid item path
- [ ] Test HTML parsing error messages in build command

### Regression Tests

- [ ] Ensure all existing functionality still works correctly
- [ ] Ensure error messages still provide helpful information
- [ ] Verify error chain display still works via `std::error::Error::source()`
- [ ] Test that `cargo txt build` still works for valid crates
- [ ] Test that `cargo txt show` still works for valid item paths
- [ ] Verify no performance degradation from new error formatting

## Structure After Changes

### File Structure

```
cargo-txt/
└── src/
    ├── error.rs           # Updated with HtmlError enum and new Display impls
    ├── commands/
    │   ├── build.rs      # Updated error construction
    │   └── show.rs       # Updated error consumption
    ├── html2md.rs        # Updated error construction
    └── main.rs           # Error display verification
```

### Error Enum Structure

```rust
// src/error.rs - New structure

pub enum Error {
    /// Errors that occur during the build process
    Build(BuildError),
    /// Errors that occur during the show process
    Show(ShowError),
    /// HTML parsing errors
    Html(HtmlError),
}

pub enum HtmlError {
    /// CSS selector failed to parse
    SelectorParseFailed {
        selector: String,
        error: String,
        crate_name: String,
        html_path: String,
        rust_version: String,
    },
    /// Required HTML element not found
    ElementNotFound {
        selector: String,
        crate_name: String,
        html_path: String,
        rust_version: String,
    },
}
```

**Note:**

1. The `ShowError::ItemPathResolutionFailed` variant will have the
   `attempted_paths` field removed since it is always empty in the current
   implementation.
2. The `HtmlError` variants now include `crate_name`, `html_path`, and
   `rust_version` fields to provide complete context for debugging internal
   errors. The `rust_version` can be obtained by running `rustc --version`
   command and parsing the output.

### Error Message Format

```rust
// Example: InvalidCrateName error message
BuildError::InvalidCrateName {
    requested: "random-crate".to_string(),
    available: vec!["clap".to_string(), "serde".to_string()],
}

// Display output:
// Crate 'random-crate' is not an installed dependency.
//
// Available crates: clap, serde
//
// Add the crate to Cargo.toml as a dependency first.
```

## Design Considerations

1. **HTML Error Consolidation**:
    - **Alternative**: Keep HTML errors as separate top-level variants
    - **Resolution**: Create `HtmlError` enum and wrap in `Html(HtmlError)`
      variant
    - **Rationale**: Follows the established pattern of `Build(BuildError)` and
      `Show(ShowError)`, making the error hierarchy consistent and easier to
      maintain

2. **Error Message Actionability**:
    - **Alternative**: Keep some errors without explicit next steps
    - **Resolution**: Every error should tell the LLM what to communicate to the
      user
    - **Rationale**: LLMs are using cargo-txt as a tool, not modifying it. Error
      messages should guide LLMs on how to help the user, not how to fix
      cargo-txt

3. **Error Message Conciseness**:
    - **Alternative**: Keep conversational elements for readability
    - **Resolution**: Remove all conversational filler and unnecessary
      formatting
    - **Rationale**: LLMs parse structured, direct text more efficiently; humans
      can still understand direct messages

4. **Error Message Structure**:
    - **Alternative**: Use JSON or other structured format
    - **Resolution**: Keep text-based format but use consistent structure
    - **Rationale**: Text is more readable for humans while still being
      parseable for LLMs; JSON would require breaking changes to command-line
      output handling

5. **Backward Compatibility**:
    - **Alternative**: Maintain dual error message formats
    - **Resolution**: Replace all error messages with LLM-friendly versions
    - **Rationale**: Primary users are LLMs; human users can adapt to clearer,
      more direct messages. The improvements benefit all users.

## Success Criteria

- All error messages follow LLM-friendly design principles (direct, actionable,
  structured)
- `HtmlError` enum is created and all HTML errors use it
- Top-level `Error` enum uses `Html(HtmlError)` variant instead of separate
  variants
- Every error message includes at least one actionable next step where
  applicable
- Error messages include exact file paths when relevant
- All tests pass with `cargo test`
- No compiler warnings with `cargo clippy`
- Integration tests verify error output format
- Documentation is updated to reflect new error structure and message format
- Error messages remain helpful for humans while being optimized for LLMs

## Implementation Status: 🟡 NOT STARTED

## Implementation Notes

Record specific technical details, challenges, or decisions made during
implementation.
