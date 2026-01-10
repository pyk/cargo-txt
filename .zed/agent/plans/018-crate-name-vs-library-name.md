---
type: normal
title: "Implement Crate Name vs Library Name Concept"
seq: 018
slug: "crate-name-vs-library-name"
created: "2025-01-10T12:00:00Z"
status: completed
---

# Implement Crate Name vs Library Name Concept

Introduce clear separation between crate names and library names to improve
documentation handling and user experience. This change addresses the common
case where crate names use hyphens (e.g., `rustdoc-types`) but library names use
underscores (e.g., `rustdoc_types`).

## Current Problems

The current implementation conflates crate names and library names, causing
confusion when users try to access documentation. For example, a crate named
`rustdoc-types` in `Cargo.toml` produces a library named `rustdoc_types`, but
the system expects users to know and use the crate name everywhere.

Current issues:

1. Output directory uses crate name: `target/docmd/rustdoc-types/`
2. Show command expects crate name input: `cargo txt show rustdoc-types::Item`
3. No metadata tracking the relationship between crate name and library name
4. Users must know the Cargo.toml dependency name instead of the Rust path name

## Proposed Solution

Separate crate name and library name throughout the system:

1. **Crate Name**: Dependency name from `Cargo.toml` (e.g., `rustdoc-types`)
    - Available from `cargo metadata`
    - Used for `cargo txt build` and `cargo txt list` commands

2. **Library Name**: Root namespace name from `cargo doc` (e.g.,
   `rustdoc_types`)
    - Extracted from cargo doc output directory
    - Used for `cargo txt show` command and output directory structure

3. **Metadata JSON**: Track the relationship in
   `target/docmd/<lib_name>/metadata.json`
    - Contains crate name for error messages
    - Contains library name for validation and debugging
    - Contains item map for fast lookups

## Analysis Required

### Dependency Investigation

- [x] Examine `extract_item_mappings` to understand current item-to-file mapping
- [x] Check error handling for missing documentation directories
- [x] Review how library name is extracted from cargo doc output

### Code Locations to Check

- `src/commands/build.rs` - Build command implementation
- `src/commands/show.rs` - Show command implementation
- `src/commands/list.rs` - List command implementation
- `src/cargo.rs` - Cargo metadata and doc execution
- `src/main.rs` - CLI entry point and argument parsing

## Implementation Checklist

### Code Changes

#### Define Metadata Structure

- [x] Add `CrateDocMetadata` struct to `src/commands/build.rs`:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct CrateDocMetadata {
    crate_name: String,
    lib_name: String,
    item_map: HashMap<String, String>,
}
```

- [x] Add `serde` and `serde_json` dependencies if not present

#### Update Build Command

- [x] Modify `build` function to (after `cargo::doc` finishes and returns
      `html_dir`):
    - Extract library name from `html_dir.file_name()` (already doing this)
    - Create output directory using library name: `target/docmd/<lib_name>/`
    - Generate `CrateDocMetadata` with crate_name, lib_name, and item_map
    - Save `metadata.json` to `target/docmd/<lib_name>/metadata.json`
- [x] Remove `save_crate_path_name` function (no longer needed)
- [x] Remove any other unused functions from build.rs
- [x] Update `extract_item_mappings` to:
    - Accept library name instead of crate name for prefixing
    - Generate full paths like `rustdoc_types::AssocItemConstraint` (not
      `rustdoc-types::AssocItemConstraint`)
- [x] Remove `if_needed` function (no longer needed, show and list commands do
      not auto-build)

#### Update Show Command

- [x] Modify `parse_item_path` to:
    - Parse input as `lib_name::item_path` (rename from crate_name)
    - Validate library name format
- [x] Replace `resolve_markdown_path` with new logic:
    - Attempt to read and parse `target/docmd/<lib_name>/metadata.json`
    - If file not found, return error with available crates list
    - Look up `lib_name::item_path` in item_map
    - If not found, error suggesting `cargo txt list <crate_name>`
    - Read and display the found file
- [x] Update error messages:
    - When item not found in item_map, suggest `cargo txt list <crate_name>`
      where crate_name comes from metadata
    - When metadata not found, list available crate names from cargo metadata
- [x] Remove `read_crate_path_name` function (no longer needed)
- [x] Remove any other unused functions from show.rs

#### Update List Command

- [x] Modify list command to use library name input (similar to show command)
- [x] Update error handling:
    - Attempt to read `target/docmd/<lib_name>/metadata.json`
    - If file not found, return error with available crates list
    - Read and display `target/docmd/<lib_name>/all.md` directly
- [x] Remove `read_crate_path_name` function (no longer needed)
- [x] Remove `validate_crate_name` function (no longer needed, library name
      validation in parse)
- [x] Remove any other unused functions from list.rs

### Documentation Updates

- [x] Update README.md:
    - Add section explaining crate name vs library name concept
    - Update build command examples to clarify crate name input
    - Update show command examples to use library name
    - Update list command examples to clarify behavior
    - Remove all auto-build references from show and list command sections
- [x] Update DOCS.md:
    - Add crate name vs library name definition section
    - Update build command documentation to explain output directory naming
    - Update show command documentation to explain library name input
    - Update error message examples to show library name usage
    - Document metadata.json structure and purpose
    - Remove all auto-build references from show and list command sections

### Test Updates

#### Add New Tests

- [x] Add tests for metadata generation in build command:
    - Assert metadata.json exists at correct path
    - Assert crate_name field matches input crate name
    - Assert lib_name field matches extracted library name
    - Assert item_map contains expected entries with library name prefix
- [x] Add tests for show command library name parsing:
    - Assert valid library name is extracted from input
    - Assert item path is correctly parsed
    - Assert error on invalid library name format
- [x] Add tests for show command metadata lookup:
    - Assert correct file path is found for known item
    - Assert item_map keys use library name prefix (e.g., `rustdoc_types::Item`)
- [x] Add tests for error when metadata file doesn't exist:
    - Assert error message contains library name
    - Assert error message lists available crates from cargo metadata
- [x] Add tests for error when item not found in metadata:
    - Assert error message suggests list command with crate_name from metadata
    - Assert error shows crate name (not library name)
- [x] Add tests for crate names with hyphens vs library names with underscores:
    - Assert build with `rustdoc-types` creates `rustdoc_types` directory
    - Assert metadata contains crate_name="rustdoc-types" and
      lib_name="rustdoc_types"
    - Assert show command works with `rustdoc_types::Item` (underscored)
    - Assert show command fails with `rustdoc-types::Item` (hyphenated)

#### Update Existing Tests

- [x] Update `extract_item_mappings` tests to use library name prefix
- [x] Update show command tests to use library names
- [x] Update build command tests to verify output directory naming
- [x] Remove tests for `save_crate_path_name` and `read_crate_path_name`
      functions

## Test Plan

### Verification Tests

- [x] Build crate with hyphenated name (e.g., `rustdoc-types`)
- [x] Verify output directory uses underscored name:
      `target/docmd/rustdoc_types/`
- [x] Verify `metadata.json` exists and contains correct crate name and item map
- [x] Run `cargo txt show rustdoc_types::Item` (library name with underscore)
- [x] Verify show command correctly resolves and displays item
- [x] Run `cargo txt show rustdoc-types::Item` (crate name with hyphen)
- [x] Verify this returns error about library name not built
- [x] Run `cargo txt list rustdoc-types` (crate name)
- [x] Verify list works with crate name (backward compatibility)
- [x] Run `cargo txt list rustdoc_types` (library name)
- [x] Verify list works with library name

### Regression Tests

- [x] Test build command with simple crate name (no hyphens/underscores)
- [x] Test show command with nested paths: `crate::module::Type`
- [x] Test error when item doesn't exist
- [x] Verify all existing tests pass

## Structure After Changes

### Directory Structure

```
target/
└── docmd/
    └── rustdoc_types/              # Library name directory (not rustdoc-types)
        ├── metadata.json            # Contains crate name + item map
        ├── index.md                # Crate overview
        ├── all.md                  # Master index
        ├── struct.AssocItemConstraint.md
        └── ...
```

### Metadata JSON Format

```json
{
    "crate_name": "rustdoc-types",
    "lib_name": "rustdoc_types",
    "item_map": {
        "rustdoc_types::AssocItemConstraint": "struct.AssocItemConstraint.md",
        "rustdoc_types::Abi": "enum.Abi.md",
        "rustdoc_types::Function": "struct.Function.md"
    }
}
```

### Show Command Flow

```
Input: rustdoc_types::AssocItemConstraint

1. Parse: lib_name = "rustdoc_types", item_path = "AssocItemConstraint"
2. Load: target/docmd/rustdoc_types/metadata.json
   - If not found: Error "Documentation for library 'rustdoc_types' is not built yet.
          Run 'cargo txt build <crate>' for one of the following crate: rustdoc-types"
3. Lookup: item_map["rustdoc_types::AssocItemConstraint"]
   - If not found: Error suggesting "cargo txt list rustdoc-types" (using crate_name from metadata)
4. Found: "struct.AssocItemConstraint.md"
5. Read: target/docmd/rustdoc_types/struct.AssocItemConstraint.md
6. Display content
```

## Design Considerations

1. **Output Directory Naming**: Why use library name instead of crate name?
    - Aligns with Rust path syntax users are familiar with
    - Matches cargo doc output directory naming
    - More intuitive for users accessing documentation

2. **Metadata Format**: Why store crate name, library name, and item map?
    - Crate name needed for helpful error messages telling users which crate to
      build
    - Library name provides self-contained metadata for validation and debugging
    - Item map eliminates need to parse all.html at runtime in show command
    - Faster lookups and cleaner code separation

3. **Explicit Name Usage**: No implicit conversion between crate name and
   library name
    - Build command accepts crate name (from Cargo.toml)
    - Show command accepts library name (from cargo doc output)
    - List command accepts library name (from cargo doc output)
    - Users must use the correct name type for each command

4. **Error Messages**: How to help users when documentation not built?
    - Check for metadata.json file existence
    - If missing, show available crate names from cargo metadata
    - Users then know which `cargo txt build <crate>` to run

## Success Criteria

- Build command creates output directory named by library name
- Build command generates metadata.json with crate name, lib_name, and item map
- Show command accepts library name input (e.g., `rustdoc_types::Item`)
- Show command uses metadata.json for item lookups
- Show command provides helpful errors when library not built
- List command accepts library name input
- All tests pass (new and existing)
- Documentation clearly explains crate name vs library name concepts
- `rust-lint` passes without warnings
- `cargo clippy -- -D warnings` passes without warnings

## Implementation Status: ✅ COMPLETED

## Feedback Implementation Summary

All feedback items have been successfully implemented:

### Build Module (src/commands/build.rs)

- ✅ Kept `CrateDocMetadata` as public struct
- ✅ Created dedicated `validate_crate_name()` function
- ✅ Renamed variable `available_list` to `available_crates`
- ✅ Renamed variable `html_dir` to `cargo_doc_output_dir`
- ✅ Defined `CargoDocOutput` struct with path, files (HashMap), and metadata
- ✅ Defined `DocOutput` struct with path and files (HashMap)
- ✅ Created `read_cargo_doc_output()` function that:
    - Takes path to directory as input
    - Returns CargoDocOutput
    - Checks index.html and all.html exist, returns error if not
    - Reads all files into HashMap
    - Builds metadata by traversing all.html
- ✅ Created `process_cargo_doc_output()` function that:
    - Takes CargoDocOutput as input
    - Returns DocOutput
    - Applies transformation logic (HTML to markdown)
- ✅ Created `save_doc()` function that:
    - Takes DocOutput as input
    - Iterates and writes files to disk
- ✅ Refactored `build()` to use new functions, separating I/O from logic
- ✅ Fixed `format_all_md` tests:
    - Uses exact assertions instead of `.contains()`
    - Merged into comprehensive test

### List Module (src/commands/list.rs)

- ✅ Imported `CrateDocMetadata` from build module
- ✅ Removed `find_metadata_by_crate_name` function
- ✅ Simplified `list()` function to just check if metadata.json exists
- ✅ Ran `rust-lint` and fixed all errors

### Show Module (src/commands/show.rs)

- ✅ Imported `CrateDocMetadata` from build module
- ✅ Changed error message to recommend `cargo txt list <lib_name>` instead of
  crate_name

## Implementation Details

### Code Organization Improvements

The build module has been significantly refactored to improve maintainability:

1. **Separation of Concerns**: I/O operations are now clearly separated from
   business logic
    - `read_cargo_doc_output()`: Pure I/O - reads files and builds initial data
      structure
    - `process_cargo_doc_output()`: Pure logic - transforms data without I/O
    - `save_doc()`: Pure I/O - writes data to disk
    - `build()`: Orchestrator - coordinates the three phases

2. **Clear Data Flow**:

    ```
    cargo doc → read_cargo_doc_output() → CargoDocOutput
                                     → process_cargo_doc_output() → DocOutput
                                                                 → save_doc() → Files on disk
    ```

3. **Better Error Handling**:
    - Early validation of required files (index.html, all.html)
    - Clear error messages at each phase
    - Proper use of `bail!` and context for better debugging

4. **Test Improvements**:
    - Merged multiple `format_all_md` tests into one comprehensive test
    - Changed from `.contains()` assertions to exact string assertions
    - Better test coverage with edge cases

### Code Quality

- All `rust-lint` warnings addressed
- No `unwrap()` calls without proper error handling
- Clear variable names (`available_crates`, `cargo_doc_output_dir`)
- Proper module structure with shared `CrateDocMetadata` struct

### Testing

All tests pass successfully (49 tests):

- Build module tests: format_all_md, extract_item_mappings_from_html
- Show module tests: parse_item_path, error message format preservation
- List module tests: metadata structure validation
- HTML2MD module tests: all conversion tests pass

## Implementation Notes

### Key Functions to Modify

1. `build::build` - Main build function
2. `build::extract_item_mappings` - Change prefix from crate name to library
   name
3. `show::parse_item_path` - Update field names
4. `show::resolve_markdown_path` - Complete rewrite to use metadata
5. `list::resolve_all_md_path` - Update to use library name directory

### Dependencies

Add to `Cargo.toml` if not present:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Test Coverage Priority

High priority:

- Metadata generation and persistence
- Show command library name parsing
- Show command metadata lookup
- Error handling for missing library directory
- Edge cases (empty names, special characters)

Medium priority:

- Build command output directory naming
- List command with library names
- Backward compatibility

Low priority:

- Performance benchmarks (should be fast with metadata)
