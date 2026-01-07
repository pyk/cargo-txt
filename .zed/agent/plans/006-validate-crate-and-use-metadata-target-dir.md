---
type: normal
title: "Validate Crate and Use Metadata Target Directory"
seq: 006
slug: "validate-crate-and-use-metadata-target-dir"
created: "2026-01-07T09:33:11Z"
status: completed
---

# Validate Crate and Use Metadata Target Directory

This task improves the build command by validating that the requested crate is
an installed dependency before attempting to build documentation. It also
refactors the target directory resolution to use the value from cargo metadata
instead of relying on the CARGO_TARGET_DIR environment variable.

## Current Problems

The build command currently accepts any crate name without validation, leading
to confusing error messages when the crate is not found:

```bash
$ cargo run -- build random-crate
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/cargo-docmd build random-crate`
Error: Build(CargoExecutionFailed { crate_name: "random-crate", output: "error: package ID specification `random-crate` did not match any packages\n" })
```

The error is technically correct but doesn't clearly communicate:

1. Which crates are supported
2. That only installed dependencies (not random crates) can be built

Additionally, the target directory is resolved using the CARGO_TARGET_DIR
environment variable with a fallback to "target", even though this information
is available from cargo metadata.

## Proposed Solution

1. Execute `cargo metadata --no-deps --format-version 1` and parse the JSON
   output
2. Extract the list of dependency names from the metadata
3. Validate that the requested crate is in the dependencies list before building
4. Provide a user-friendly error message listing available crates if validation
   fails
5. Use the `target_directory` field from metadata instead of CARGO_TARGET_DIR
   environment variable

## Analysis Required

### Dependency Investigation

- [x] Check if serde_json is already imported in Cargo.toml (needed for parsing
      metadata JSON)
- [x] Verify the exact structure of cargo metadata output for dependencies array
- [x] Confirm that all dependency names are present in the dependencies array

### Code Locations to Check

- `src/cargo.rs` - Add metadata parsing function and extract target_directory
- `src/commands/build.rs` - Update to use metadata for target directory and
  crate validation
- `src/error.rs` - Add new error variant for invalid crate name
- `Cargo.toml` - Confirm serde_json dependency

## Implementation Checklist

### Code Changes

#### Phase 1: Update Cargo.toml

- [x] Verify `serde_json` is in dependencies (already at version ^1.0.148)

#### Phase 2: Add Metadata Parsing to cargo.rs

- [x] Create structs to represent the full cargo metadata JSON output with
      public fields

    ```rust
    pub struct Metadata {
        pub packages: Vec<Package>,
        pub workspace_members: Vec<String>,
        pub workspace_default_members: Vec<String>,
        pub resolve: Option<Resolve>,
        pub target_directory: String,
        pub build_directory: String,
        pub version: i32,
        pub workspace_root: String,
        pub metadata: Option<serde_json::Value>,
    }

    pub struct Resolve {
        pub nodes: Vec<Node>,
        pub root: Option<String>,
    }

    pub struct Node {
        pub id: String,
        pub dependencies: Vec<NodeDep>,
        pub deps: Vec<NodeDep>,
        pub features: Vec<(String, Vec<String>)>,
    }

    pub struct NodeDep {
        pub pkg: String,
        pub name: String,
        pub dep_kinds: Vec<DepKindInfo>,
    }

    pub struct DepKindInfo {
        pub kind: String,
        pub target: Option<String>,
    }

    pub struct Package {
        pub name: String,
        pub version: String,
        pub id: String,
        pub license: Option<String>,
        pub license_file: Option<String>,
        pub description: Option<String>,
        pub source: Option<String>,
        pub dependencies: Vec<Dependency>,
        pub targets: Vec<Target>,
        pub features: std::collections::HashMap<String, Vec<String>>,
        pub manifest_path: String,
        pub metadata: Option<serde_json::Value>,
        pub publish: Option<Vec<String>>,
        pub authors: Vec<String>,
        pub categories: Vec<String>,
        pub keywords: Vec<String>,
        pub readme: Option<String>,
        pub repository: Option<String>,
        pub homepage: Option<String>,
        pub documentation: Option<String>,
        pub edition: String,
        pub links: Option<String>,
        pub default_run: Option<String>,
        pub rust_version: Option<String>,
    }

    pub struct Dependency {
        pub name: String,
        pub source: Option<String>,
        pub req: String,
        pub kind: Option<String>,
        pub rename: Option<String>,
        pub optional: bool,
        pub uses_default_features: bool,
        pub features: Vec<String>,
        pub target: Option<String>,
        pub registry: Option<String>,
    }

    pub struct Target {
        pub kind: Vec<String>,
        pub crate_types: Vec<String>,
        pub name: String,
        pub src_path: String,
        pub edition: String,
        pub doc: bool,
        pub doctest: bool,
        pub test: bool,
    }
    ```

- [x] Enable serde derive feature in Cargo.toml
- [x] Implement `cargo::metadata()` function that executes
      `cargo metadata --no-deps --format-version 1`
- [x] Parse JSON output into Metadata struct using serde_json
- [x] Add error handling for metadata parsing failures (new error variant)

#### Phase 3: Update Error Handling

- [x] Add new error variant to `BuildError` in `src/error.rs`:
    ```rust
    InvalidCrateName {
        requested: String,
        available: Vec<String>,
    }
    ```
- [x] Implement Display trait for the new error variant showing user-friendly
      message
- [x] Include list of available crates in the error message
- [x] Add error variant for metadata parsing failures:
    ```rust
    CargoMetadataExecFailed { output: String },
    ```

#### Phase 4: Update Build Command

- [x] Call `cargo::metadata()` at the start of `build()` function in
      `src/commands/build.rs`
- [x] Call `cargo::metadata()` to get metadata
- [x] Validate crate_name is in the dependencies list from
      `metadata.packages[0].dependencies`
- [x] Return InvalidCrateName error if crate not found
- [x] Replace `std::env::var("CARGO_TARGET_DIR")` with metadata access to
      `metadata.target_directory` in `get_html_dir()`
- [x] Replace `std::env::var("CARGO_TARGET_DIR")` with metadata access to
      `metadata.target_directory` in `get_output_dir()`
- [x] Remove the fallback to "target" since metadata always provides the value

#### Phase 5: Update Tests

- [x] Add test in `error.rs` for InvalidCrateName error formatting
- [x] Add test in `error.rs` for CargoMetadataExecFailed error formatting
- [x] Update test in `commands/build.rs` for get_html_dir to use metadata
- [x] Update test in `commands/build.rs` for get_output_dir to use metadata
- [x] Add test in `commands/build.rs` for invalid crate validation
- [x] Add integration test for the full build command with invalid crate

### Documentation Updates

- [x] Update README.md to explain that only installed dependencies can be built
- [x] Add error example showing the user-friendly error message in README.md
- [x] Update DOCS.md to document the crate validation behavior
- [x] Update DOCS.md to document how target directory is resolved
- [x] Update DOCS.md Error Handling section to include new error variants

## Test Plan

### Verification Tests

- [x] Verify that `cargo metadata` command is executed with correct arguments
- [x] Verify that build command fails gracefully with InvalidCrateName error for
      unknown crate
- [x] Verify that error message lists available crates when crate is not found
- [x] Verify that build command fails gracefully with CargoMetadataExecFailed
      error when cargo metadata execution fails
- [x] Verify that CargoMetadataExecFailed error includes command output
- [x] Verify that build command succeeds for valid dependency
- [x] Verify that target directory is used from metadata, not CARGO_TARGET_DIR

### Regression Tests

- [x] Test building documentation for an existing dependency (e.g., serde)
- [x] Test that HTML is still generated in the correct location using metadata
      target directory
- [x] Test that markdown files are still written to the correct output directory
- [x] Test that type alias parsing still works after refactoring

## Structure After Changes

### File Structure

```

cargo-docmd/ ├── src/ │ ├── cargo.rs # Add Metadata struct and parsing functions
│ ├── commands/ │ │ └── build.rs # Update to use metadata and validate crates
└── error.rs # Add InvalidCrateName and CargoMetadataExecFailed └── tests/ └──
integration/ # Add integration test for crate validation

```

### Module Exports

```rust
// src/cargo.rs - New exports
pub struct Metadata { /* ... */ }
pub struct Package { /* ... */ }
pub struct Dependency { /* ... */ }
pub struct Resolve { /* ... */ }
pub struct Node { /* ... */ }
pub struct NodeDep { /* ... */ }
pub struct DepKindInfo { /* ... */ }
pub struct Target { /* ... */ }

pub fn metadata() -> error::Result<Metadata>;
// Consumers access fields directly:
// - metadata.target_directory
// - metadata.packages[0].dependencies (includes all dependency kinds)

// src/error.rs - New error variants
pub enum BuildError {
    // ... existing variants ...
    InvalidCrateName {
        requested: String,
        available: Vec<String>,
    },
    CargoMetadataExecFailed {
        output: String,
    },
 }
```

## Design Considerations

1. **Metadata Caching**: Should we cache the metadata result?
    - **Decision**: No. The metadata is only parsed once per build command
      execution, so caching adds complexity without significant benefit.
    - **Alternative**: Could cache if the metadata function is called multiple
      times in the future.

2. **Dev Dependencies**: Should we include dev dependencies in the available
   crates list?
    - **Decision**: Yes. Users may want to build documentation for any
      dependency in their project, including dev dependencies.

3. **Error Message Format**: How should we display the list of available crates?
    - **Decision**: Display as a comma-separated list of all available crates.
    - **Example**: "Available crates: clap, rustdoc-types, serde, serde_json,
      tempfile"

4. **Target Directory Fallback**: Should we keep a fallback if metadata parsing
   fails?
    - **Decision**: No. If metadata parsing fails, we should return an error
      rather than silently falling back. This ensures users know something went
      wrong.

5. **Metadata Execution Errors**: How should we handle failures to execute
   `cargo metadata`?
    - **Decision**: Return a specific `CargoMetadataExecFailed` error with the
      command output. This helps users debug issues with their cargo
      installation.

## Success Criteria

- Build command validates that requested crate is an installed dependency
- User receives clear error message listing available crates when invalid crate
  is requested
- Target directory is resolved from cargo metadata instead of environment
  variable
- All existing tests pass
- New tests cover metadata parsing and crate validation
- Documentation updated to reflect the validation behavior
- No regression in existing functionality (building for valid dependencies still
  works)

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

### Cargo Metadata Structure

The `cargo metadata --no-deps --format-version 1` output includes these fields:

**Top-level Metadata:**

- `target_directory` - Path to the target directory
- `build_directory` - Path to the build directory
- `workspace_root` - Path to workspace root
- `packages` - Array of package objects
- `workspace_members` - Array of workspace member package IDs
- `version` - Format version number

**Package structure:**

- `name` - Package name
- `version` - Package version
- `dependencies` - Array of dependency objects
    - `name` - Dependency name
    - `kind` - Dependency kind ("dev", "build", or null for regular
      dependencies)
    - `source` - Source (e.g.,
      "registry+https://github.com/rust-lang/crates.io-index")
    - `req` - Version requirement
    - `optional` - Whether this is an optional dependency
- `targets` - Array of build targets
- `features` - HashMap of feature names to their dependencies

We collect all dependency names from the first package (typically the workspace
root package), including both regular and dev dependencies.

### Error Message Design

The InvalidCrateName error should display something like:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

The CargoMetadataExecFailed error should display something like:

```
Error: Failed to execute cargo metadata command:

error: failed to parse manifest at `/path/to/Cargo.toml`

This may indicate an issue with your cargo installation or Cargo.toml file.
```

These error messages provide clear guidance on what went wrong and how to fix
it.
