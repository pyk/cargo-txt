# Implement cargo docmd build Command - Core Infrastructure

Implement the foundational build infrastructure including CLI refactoring, cargo
command execution, JSON parsing, and output directory management. This work
enables the markdown generation that follows.

## Current Problems

The current `generate` command is a placeholder with incorrect naming and no
actual functionality.

```rust
// Current implementation in src/main.rs
#[derive(Subcommand)]
enum Command {
    Generate {
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,

        // Problem: Output flag is unnecessary and creates coupling
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<std::path::PathBuf>,
    },
}

// Current placeholder implementation in src/commands/generate.rs
pub fn generate(crate_name: String, output: PathBuf) {
    println!("Generate command: crate={}, output={:?}", crate_name, output);
    println!("Not yet implemented");
}
```

Key problems:

1. Command name `generate` is verbose - `build` is more conventional
2. `--output` flag creates unnecessary coupling between build and browse
3. No actual cargo rustdoc execution
4. No JSON parsing from rustdoc output
5. Missing error handling for cargo command failures
6. No verification that nightly toolchain is installed
7. No standardized output directory

## Proposed Solution

1. Rename `generate` command to `build` with simplified interface
2. Implement cargo rustdoc execution with nightly toolchain verification
3. Parse rustdoc JSON using rustdoc_types crate
4. Create standardized output directory at `$CARGO_TARGET_DIR/docmd`
5. Implement comprehensive error handling with user-friendly messages

## Implementation Checklist

### CLI Changes

- [x] Rename `src/commands/generate.rs` to `src/commands/build.rs`
- [x] Update `Command` enum in `src/main.rs`:
    - Change `Generate` variant to `Build`
    - Remove `output` field from `Build`
    - Update command name in clap derive macros
    - Add doc comment explaining the command
- [x] Update command execution in `main()` function to call `build()` instead of
      `generate()`
- [x] Update `mod commands` import to include `build` instead of `generate`
- [x] Update `src/commands/mod.rs` to export `build` instead of `generate`

### Error Module

- [x] Create `src/error.rs` module
- [x] Define centralized `Error` enum:
    - `Build(BuildError)` - Build-related errors
    - `Markdown(MarkdownError)` - Markdown generation errors
- [x] Define `BuildError` enum:
    - `NightlyNotInstalled` - with installation suggestion
    - `CargoExecutionFailed(String)` - with crate name and command output
    - `JsonNotFound(PathBuf)` - with expected path
    - `JsonParseError(String)` - with serde error details
    - `OutputDirCreationFailed(PathBuf, String)` - with path and io error
- [x] Define `MarkdownError` enum:
    - `FileWriteFailed(PathBuf, String)` - with path and io error
    - `DirectoryCreationFailed(PathBuf, String)` - with path and io error
    - `ItemNotFound(Id)` - with item ID
- [x] Implement `std::error::Error` trait for `Error` enum
- [x] Implement `Display` trait for all error types with user-friendly messages
- [x] Define `type Result<T> = std::result::Result<T, Error>` alias
- [x] Implement `From<std::io::Error> for Error` conversion
- [x] Add module-level documentation to `src/error.rs`

### Cargo Execution Module

- [x] Create `src/cargo.rs` module
- [x] Implement `check_nightly_installed()` function:
    - Execute `cargo +nightly --version` to check for nightly
    - Return `Result<()>` with descriptive message if missing
    - Include installation suggestion in error message
- [x] Implement `generate_rustdoc_json(crate_name: &str) -> Result<PathBuf>`
      function:
    - Execute
      `cargo +nightly rustdoc -p <crate> -- --output-format json -Z unstable-options`
    - Return the path to the generated JSON file
    - Include crate name in error message if command fails
- [x] Add module-level documentation to `src/cargo.rs`

### Build Command Implementation

- [x] Implement `build(crate_name: String) -> Result<()>` function in
      `src/commands/build.rs`:
    - Check nightly toolchain availability
    - Generate rustdoc JSON via cargo command
    - Determine output directory path
    - Create output directory if it doesn't exist
    - Parse JSON file into `rustdoc_types::Crate`
    - Log summary of parsed items (count by type)
    - Return success (actual markdown generation to be added later)
- [x] Update `src/main.rs` to import and use the centralized error types
- [x] Add module-level documentation to `src/commands/build.rs`

### Output Directory Management

- [x] Implement `get_output_dir() -> Result<PathBuf>` function:
    - Read `CARGO_TARGET_DIR` environment variable
    - Fall back to `./target` if not set
    - Append `docmd` subdirectory
    - Ensure the path exists, create if necessary
- [ ] ~~Implement `get_json_path()`~~ - REMOVED: unnecessary, path constructed
      directly in generate_rustdoc_json()

### Main Entry Point Updates

- [x] Update `main()` function error handling:
    - Match on `Command` enum and call appropriate functions
    - Call `build()` for `Build` variant
    - Convert `BuildError` to appropriate exit code (1)
    - Display error message to stderr

### Documentation Updates

- [x] Update `DOCS.md` to reflect new `build` command:
    - Replace all references to `generate` with `build`
    - Remove `--output` option documentation
    - Add information about automatic output location
    - Update examples to use `build` command
    - Add section about error messages and troubleshooting
- [x] Update `README.md` if it mentions the generate command
- [x] Add module-level documentation to all new modules
- [x] Remove "interactive" terminology from documentation

### Test Updates

- [x] ~~Create unit tests for `check_nightly_installed()`~~ - REMOVED: tests
      don't verify meaningful behavior
- [x] Create unit tests for `generate_rustdoc_json()`:
    - Test with invalid crate name
- [x] ~~Create unit tests for `get_output_dir()`~~ - REMOVED: test doesn't
      verify meaningful behavior
- [x] ~~Create unit tests for `get_json_path()`~~ - REMOVED: function was
      removed
- [x] ~~Create unit tests for `BuildError` display formatting~~ - Display
      behavior verified by error tests
- [x] Update existing command parsing tests if any

## Test Plan

### Verification Tests

- [x] Verify `cargo docmd build --crate serde` executes successfully
- [x] Verify JSON file is generated in `target/doc/serde.json`
- [x] Verify error message when nightly is not installed includes installation
      instructions
- [x] Verify error when crate is not found includes the crate name
- [x] Verify build function logs count of parsed items
- [x] Verify `--help` shows correct command structure
- [x] Verify error messages include full paths when applicable

### Regression Tests

- [x] Verify global flags still work (`--verbose`)
- [x] Verify `--version` still works
- [x] Verify `browse` command is still available

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── main.rs                 # Updated Command enum
│   ├── error.rs                # NEW: Centralized error definitions
│   ├── cargo.rs                # NEW: Cargo command execution
│   └── commands/
│       ├── mod.rs              # Updated exports
│       ├── browse.rs           # Unchanged
│       └── build.rs            # RENAMED from generate.rs
```

### Module Exports

```rust
// src/main.rs
mod error;  // NEW
mod cargo;  // NEW
use error::Result;  // Use centralized Result alias
use commands::{browse, build};
```

```rust
// src/commands/mod.rs
pub mod browse;
pub mod build;  // CHANGED from generate
```

```rust
// src/main.rs Command enum
#[derive(Subcommand)]
enum Command {
    /// Build markdown documentation from rustdoc JSON
    Build {
        /// Crate name to build documentation for
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,
    },
    Browse { /* ... */ },
}
```

### Key Functions

```rust
// src/error.rs
pub enum Error { /* ... */ }
pub enum BuildError { /* ... */ }
pub enum MarkdownError { /* ... */ }
pub type Result<T> = std::result::Result<T, Error>;

// src/cargo.rs
use error::{Result, BuildError};
pub fn check_nightly_installed() -> Result<()>;
pub fn generate_rustdoc_json(crate_name: &str) -> Result<PathBuf>;

// src/commands/build.rs
use error::Result;
pub fn build(crate_name: String) -> Result<()>;
fn get_output_dir() -> Result<PathBuf>;
fn parse_rustdoc_json(json_path: &Path) -> Result<rustdoc_types::Crate>;
fn log_item_summary(krate: &rustdoc_types::Crate);
```

## Design Considerations

### 1. Command Name Selection

**Decision**: Use `build` instead of `generate`.

- **Alternative**: Keep `generate`.
    - Rejected: `build` is the conventional cargo command name for generating
      artifacts
- **Resolution**: `build` is concise, familiar to Rust developers, and indicates
  we're building documentation artifacts

### 2. Output Directory Location

**Decision**: Use `$CARGO_TARGET_DIR/docmd` as the default.

- **Alternative 1**: Use `./docs` as default.
    - Rejected: Would pollute workspace root with docs for multiple crates
- **Alternative 2**: Use `./target/docmd` hardcoded.
    - Rejected: Not compatible with custom `CARGO_TARGET_DIR` settings
- **Resolution**: Respecting `$CARGO_TARGET_DIR` ensures compatibility with all
  cargo configurations

### 3. Nightly Toolchain Check

**Decision**: Check for nightly before executing rustdoc.

- **Alternative 1**: Just execute and let cargo error.
    - Rejected: Error messages from cargo can be confusing
- **Alternative 2**: Install nightly automatically.
    - Rejected: Too intrusive, may not be desired
- **Resolution**: Check explicitly and provide clear error message with
  installation instructions

### 4. Error Handling Strategy

**Decision**: Use centralized `src/error.rs` module with custom enums.

- **Alternative**: Use `anyhow` crate.
    - Rejected: Adds dependency, custom enums provide better control over error
      messages
- **Alternative**: Use `Box<dyn Error>`.
    - Rejected: Less ergonomic for specific error variants
- **Alternative**: Define error types in each module.
    - Rejected: Duplicates error definitions, harder to maintain
- **Resolution**: Centralized error module provides consistency, type-safe
  errors, and user-friendly messages. The `Result<T>` alias simplifies error
  propagation throughout the codebase.

### 5. JSON Parsing Location

**Decision**: Parse JSON directly in `build.rs`.

- **Alternative**: Create separate `json_parser.rs` module.
    - Rejected: Parsing is a single line with serde, unnecessary abstraction
- **Alternative**: Defer parsing until markdown generation.
    - Rejected: We want to validate JSON early and provide item count feedback
- **Resolution**: Keep parsing in build.rs for early validation and feedback

## Success Criteria

- [x] Command `cargo docmd build --crate serde` executes successfully on a
      workspace with serde as a dependency
- [x] JSON file is generated at `target/doc/serde.json`
- [x] Build function logs count of parsed items to stdout
- [x] Error when nightly is not installed includes clear installation
      instructions
- [x] Error when crate is not found includes the crate name that was searched
- [x] Error when JSON file is missing includes the full path that was expected
- [x] `--help` output shows `build` command with correct signature
- [x] All unit tests pass (2 tests)
- [x] Documentation in `DOCS.md` accurately reflects the new command interface
- [x] No compiler warnings after all changes (only 1 expected warning for unused
      MarkdownError variants)

## Implementation Status: ✅ COMPLETED

## Implementation Notes

### Completed Work

All checklist items completed successfully. Key accomplishments:

1. **Simplified generate_rustdoc_json()**: Removed complex file searching logic.
   Function now simply constructs and returns the expected path to
   target/doc/crate_name.json. Error handling happens appropriately when parsing
   JSON in build.rs.

2. **Removed unnecessary tests**: Removed tests that didn't verify meaningful
   behavior:
    - Nightly check tests (just called function and ignored result)
    - Output directory test (couldn't assert exact path)
    - JSON path construction tests (function was removed)

3. **Followed Rust coding guidelines**:
    - All doc comments avoid "Arguments" and "Returns" sections
    - Use descriptive variable names (no abbreviations)
    - Favor linear control flow over nesting
    - Removed "interactive" terminology from documentation

4. **Documentation updates**:
    - All references to "generate" updated to "build"
    - Removed "--output" flag documentation
    - Removed misleading "interactive" language
    - Updated both README.md and DOCS.md

5. **Test suite**: Now has 2 meaningful tests that verify actual error handling
   behavior:
    - parse_rustdoc_json_returns_error_for_missing_file
    - generate_rustdoc_json_returns_error_for_invalid_crate

### Known Warnings

- One compiler warning for unused MarkdownError variants
  (DirectoryCreationFailed, ItemNotFound). These are intentionally kept for
  future markdown generation functionality.

### Next Steps

Ready to proceed to plan: `2026-01-07-markdown-framework.md` to create the
markdown generation infrastructure.
