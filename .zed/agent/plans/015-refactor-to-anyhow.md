---
type: normal
title: "Refactor error handling to use anyhow"
seq: 015
slug: "refactor-to-anyhow"
created: "2026-01-09T12:43:33Z"
status: in_progress
---

# Refactor error handling to use anyhow

This plan refactors the cargo-txt binary to use anyhow for error handling,
replacing the custom error types defined in `src/error.rs`. anyhow provides
uniform error handling with automatic source chaining, context addition, and
LLM-friendly error messages.

## Current Problems

The current error handling uses custom error enums that are verbose to maintain
and don't provide the flexibility of anyhow:

```rust
// src/error.rs - Custom error types (to be removed)
pub enum Error {
    Build(BuildError),
    Show(ShowError),
    HtmlSelectorParseFailed { selector: String, error: String },
    HtmlElementNotFound { selector: String },
}

// Verbose error creation in commands
std::fs::read_to_string(&html_path).map_err(|error| error::BuildError::FileReadFailed {
    path: html_path.clone(),
    source: Box::new(error),
})?;
```

## Proposed Solution

1. Remove `src/error.rs` and all custom error types
2. Replace `error::Result` with `anyhow::Result` throughout the codebase
3. Use `.context()` to add descriptive context to errors
4. Use `bail!` for early returns with descriptive messages
5. Use `ensure!` for validation conditions
6. Update tests to match anyhow error handling

## Analysis Required

### Dependency Investigation

- [x] Confirm anyhow v1.0.100 is properly configured in Cargo.toml
- [x] Review rust-error-handling-bin.md guidelines for proper anyhow patterns

### Code Locations to Update

- `src/error.rs` - Entire file to be removed
- `src/main.rs` - Remove `mod error;`, add `use anyhow::Result;`
- `src/commands/build.rs` - Replace custom error creation with `.context()` and
  `bail!`
- `src/commands/show.rs` - Replace custom error creation with `.context()` and
  `ensure!`
- `src/cargo.rs` - Replace custom error creation with `.context()`
- `src/html2md.rs` - Replace custom error creation with `.context()` and `bail!`

## Implementation Checklist

### Code Changes

#### Remove Custom Error Types

- [x] Delete `src/error.rs` file entirely
- [x] Update `src/main.rs`:
    - Remove `mod error;` declaration
    - Add `use anyhow::Result;` at top
    - Replace `error::Result<()>` with `Result<()>` in main function signature

#### Update src/commands/build.rs

- [x] Add imports: `use anyhow::{Context, Result};`
- [x] Replace `error::Result` with `Result` in function signatures
- [x] Replace `InvalidCrateName` with `bail!`:
    ```rust
    bail!(
        r#"Crate '{}' is not an installed dependency.
    ```

Available crates: {}

Only installed dependencies can be built. Add the crate to Cargo.toml as a
dependency first."#, crate_name, available.join(", ") ); ```

- [x] Replace `FileReadFailed` with `.context()`:
    ```rust
    std::fs::read_to_string(&html_path)
        .context(format!("failed to read file '{}'", html_path.display()))?;
    ```
- [x] Replace `OutputDirCreationFailed` with `.context()`:
    ```rust
    std::fs::create_dir_all(&output_dir)
        .context(format!("failed to create output directory '{}'", output_dir.display()))?;
    ```
- [x] Replace `MarkdownWriteFailed` with `.context()`:
    ```rust
    std::fs::write(&index_path, markdown_content)
        .context(format!("failed to write markdown file '{}'", index_path.display()))?;
    ```
- [x] Replace `DocIndexNotFound` with `.context()`:
    ```rust
    std::fs::read_to_string(&all_html_path)
        .context(format!("failed to read documentation index file '{}'", all_html_path.display()))?;
    ```
- [x] Replace `HtmlSelectorParseFailed` with `.context()`:
    ```rust
    let selector = Selector::parse("ul.all-items li a")
        .context("failed to parse HTML selector for item mappings")?;
    ```
- [x] Replace `HtmlElementNotFound` with `bail!`:
    ```rust
    if mappings.is_empty() {
        bail!("failed to find item mappings in documentation - no items found");
    }
    ```

#### Update src/commands/show.rs

- [x] Add imports: `use anyhow::{Context, ensure, Result};`
- [x] Replace `error::Result` with `Result` in function signatures
- [x] Replace `InvalidItemPath` with `ensure!`:
    ```rust
    let crate_name = parts.next().filter(|s| !s.is_empty());
    ensure!(crate_name.is_some(),
        "invalid item path '{}'. Expected format: <crate> or <crate>::<item> (e.g., 'serde' or 'serde::Error').",
        item_path
    );
    let crate_name = crate_name.unwrap();
    ```
- [x] Replace `DocIndexNotFound` with `.context()`:
    ```rust
    std::fs::read_to_string(&all_html_path)
        .context(format!("failed to read documentation index file '{}'", all_html_path.display()))?;
    ```
- [x] Replace `MarkdownNotFound` with `.context()`:
    ```rust
    std::fs::read_to_string(&markdown_path)
        .context(format!("failed to read markdown file '{}'", markdown_path.display()))?;
    ```
- [x] Replace `ItemPathResolutionFailed` with `bail!`:
    ```rust
    bail!(
        r#"could not resolve item path '{}'. Please ensure the item exists in the crate and try: `cargo txt build {}`"#,
        item_path,
        parsed.crate_name
    );
    ```

#### Update src/cargo.rs

- [x] Add imports: `use anyhow::{Context, Result};`
- [x] Replace `error::Result` with `Result` in function signatures
- [x] Replace `CargoMetadataExecFailed` with `.context()`:
    ```rust
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to execute cargo metadata command")?;
    ```
- [x] Replace `CargoDocExecFailed` with `.context()`:
    ```rust
    let output = cmd.output()
        .context(format!("failed to execute cargo doc for crate '{}'", crate_name))?;
    ```
- [x] Replace `CargoDocOutputParseFailed` with `bail!`:
    ```rust
    bail!(
        r#"failed to parse cargo doc output - could not find 'Generated' line. Output preview: {}"#,
        output_preview
    );
    ```

#### Update src/html2md.rs

- [x] Add imports: `use anyhow::{Context, bail, Result};`
- [x] Replace `error::Result` with `Result` in function signatures
- [x] Replace `HtmlSelectorParseFailed` with `.context()`:
    ```rust
    let selector = Selector::parse("main")
        .context("failed to parse HTML selector for main element")?;
    ```
- [x] Replace `HtmlElementNotFound` with `bail!`:
    ```rust
    bail!(
        "HTML document does not contain a <main> element. This may indicate invalid rustdoc HTML output."
    );
    ```

### Test Updates

- [x] Update `src/commands/show.rs` tests:
    - Change error matching from specific variants to string matching
    - Example: `assert!(result.is_err())` with message checks
- [x] Update `src/commands/build.rs` tests:
    - Change error matching from specific variants to string matching
    - Example: `assert!(result.is_err())` with message checks
- [x] Update `src/cargo.rs` tests:
    - Change error matching from specific variants to string matching
    - Example: `assert!(result.is_err())` with message checks

### Documentation Updates

- [x] Update `DOCS.md` Error Handling section:
    - Remove references to specific error types (BuildError, ShowError, etc.)
    - Update examples to show anyhow-style error handling with context chains
    - Document that errors are automatically chained with context
    - Update error message examples to match anyhow output format

## Test Plan

### Verification Tests

- [x] Run `cargo test` - All tests should pass
- [x] Run `cargo clippy` - No warnings should be present
- [x] Run `cargo build` - Binary should compile successfully

### Manual Testing

- [x] Install binary: `cargo install --path .`
- [x] Test build command with valid crate: `cargo txt build serde`
- [x] Test build command with invalid crate: `cargo txt build random-crate`
      (should show user-friendly error)
- [x] Test show command with crate only: `cargo txt show serde`
- [x] Test show command with item path: `cargo txt show serde::Error`
- [x] Test show command with invalid item: `cargo txt show serde::NonExistent`
      (should show user-friendly error)
- [x] Test error output with verbosity: `cargo txt build invalid -v` (should
      show error chain)

## Structure After Changes

### File Structure

```
src/
├── main.rs           # Updated to use anyhow::Result
├── cargo.rs          # Updated to use anyhow::Result
├── html2md.rs        # Updated to use anyhow::Result
└── commands/
    ├── mod.rs        # No changes needed
    ├── build.rs      # Updated to use anyhow::Result
    └── show.rs       # Updated to use anyhow::Result

# REMOVED: src/error.rs
```

### Module Exports

```rust
// BEFORE
mod error;
mod cargo;
mod html2md;

// AFTER
mod cargo;
mod html2md;

// Add to main.rs
use anyhow::Result;
```

## Design Considerations

1. **Error Context Strategy**: Add context at each layer to build clear error
   chains. This helps both humans and LLMs understand where errors occurred.

2. **User vs Internal Errors**: Distinguish between:
    - User errors (invalid paths, missing crates) - provide helpful guidance
    - Internal errors (parsing failures) - suggest reporting bugs with details

3. **Error Message Format**: anyhow automatically formats error chains with
   "Caused by:" sections. Leverage this by adding appropriate context at each
   layer.

4. **Test Strategy**: Since we lose specific error type matching, tests will
   verify errors through message content and `is_err()` checks rather than
   variant matching.

5. **Raw String Literals**: Use `r#"..."#` for multiline error messages to
   improve readability and avoid escaped newline characters.

## Success Criteria

- All custom error types removed from `src/error.rs` (file deleted)
- All functions use `anyhow::Result` instead of custom `error::Result`
- Error messages are preserved or improved with context
- All tests pass: `cargo test`
- No clippy warnings: `cargo clippy`
- Binary compiles and runs successfully: `cargo install --path .` followed by
  `cargo txt build <crate>`
- Documentation updated to reflect anyhow-based error handling

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

### Technical Details

1. **Selector::parse Compatibility**: The `scraper` crate's `Selector::parse`
   returns a `Result<Selector, SelectorErrorKind<'_>>` where the error type
   doesn't implement `Send + Sync + StdError`, so `.context()` cannot be used.
   Instead, we use `.map_err(|e| anyhow::anyhow!(...))` to convert these errors.

2. **Error Message Format**: anyhow automatically formats error chains with
   "Caused by:" sections. We added appropriate context at each layer to build
   clear error chains for both humans and LLMs.

3. **Test Updates**: Since we lost specific error type matching, tests now
   verify errors through message content and `is_err()` checks rather than
   variant matching.

4. **User vs Internal Errors**: The refactored code distinguishes between:
    - User errors (invalid paths, missing crates) - provide helpful guidance
      with `bail!`
    - Internal errors (parsing failures, file I/O) - suggest reporting bugs with
      `.context()`

5. **Manual Testing Results**:
    - Valid crate build: ✅ Works correctly
    - Invalid crate: ✅ Shows helpful error with available crates list
    - Show crate index: ✅ Displays all.md correctly
    - Show invalid item: ✅ Shows helpful error message
    - Valid item path: ✅ Displays documentation correctly
    - Error with verbosity: ✅ Shows error chain appropriately

All success criteria have been met:

- All custom error types removed from `src/error.rs` (file deleted)
- All functions use `anyhow::Result` instead of custom `error::Result`
- Error messages are preserved or improved with context
- All tests pass: `cargo test` (43/43 passed)
- No clippy warnings: `cargo clippy`
- Binary compiles and runs successfully: `cargo install --path .` followed by
  successful manual tests
- Documentation updated to reflect anyhow-based error handling
