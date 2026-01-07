# CLI Reference

This document provides detailed reference information for all cargo-docmd
command-line interface commands and options.

## Overview

cargo docmd converts rustdoc HTML output into markdown documentation optimized
for coding agents. The tool provides two primary modes of operation: build
markdown files or browse documentation.

The build command automatically generates rustdoc HTML using stable cargo doc
and creates markdown files in `$CARGO_TARGET_DIR/docmd`.

## Commands

### build

Generate rustdoc HTML and create markdown documentation from it.

```shell
cargo docmd build --crate <CRATE>
```

#### Options

- `--crate <CRATE>` (or `-c`)
    - **Required**: Crate name to build documentation for
    - Example: `--crate serde`

#### Output Location

Markdown files are placed in `$CARGO_TARGET_DIR/docmd`. If `CARGO_TARGET_DIR` is
not set, the default is `./target/docmd`.

#### Examples

Build documentation for serde crate:

```shell
cargo docmd build --crate serde
```

Build with verbose output:

```shell
cargo docmd -v build --crate serde
```

#### What It Does

1. Runs `cargo doc --package <crate> --no-deps` to generate HTML
2. Parses type alias HTML files from the generated documentation
3. Creates the output directory if needed
4. Generates markdown files for type aliases only
5. Logs a summary of generated files

#### Limitations

The build command currently generates markdown for type aliases only. Other item
types (structs, enums, unions) will be added in future phases.

### browse

Browse crate documentation in the terminal.

```shell
cargo docmd browse --crate <CRATE>
```

#### Options

- `--crate <CRATE>` (or `-c`)
    - **Required**: Crate name to browse
    - Example: `--crate serde`

- `--item <ITEM>` (or `-i`)
    - **Optional**: Display documentation for a specific item only
    - Example: `--item Serialize`

#### Examples

Browse entire crate documentation:

```shell
cargo docmd browse --crate serde
```

Display specific item documentation:

```shell
cargo docmd browse --crate serde --item Serialize
```

#### Limitations

The browse command currently prints the received parameters but does not display
documentation. Interactive browsing functionality will be implemented in future
iterations.

## Global Options

These options apply to all subcommands.

### `--verbose` / `-v`

Increase verbosity of output. Use multiple times for more verbose output (e.g.,
`-vv`, `-vvv`).

Examples:

```shell
cargo docmd -v build --crate serde
cargo docmd -vv browse --crate serde
```

### `--config` / `-c`

Path to configuration file. This option is currently reserved for future
implementation.

Example:

```shell
cargo docmd --config ./config.toml build --crate serde
```

#### Limitations

Configuration file parsing is not yet implemented. The `--config` option is
accepted but has no effect.

### `--help` / `-h`

Print help information for the command or subcommand.

Examples:

```shell
cargo docmd --help
cargo docmd build --help
cargo docmd browse --help
```

### `--version` / `-V`

Print version information.

```shell
cargo docmd --version
```

## Markdown Output Format

The markdown framework generates documentation files optimized for coding
agents. All files follow a consistent structure and naming convention.

### File Naming Convention

Markdown files use a deterministic naming scheme:

- Replace `::` with `-` throughout the item path
- Remove generic parameters (e.g., `HashMap<K, V>` becomes `HashMap`)
- Add `.md` extension

Examples:

- `std::vec::Vec` → `std-vec-Vec.md`
- `std::collections::HashMap<K, V>` → `std-collections-HashMap.md`
- `serde::Serialize::serialize` → `serde-Serialize-serialize.md`

### Index Page

The `index.md` file serves as a navigation hub and contains:

- Crate name and documentation
- Item counts grouped by type
- Links to all public items
- "Next Actions" section for common operations

### Standard Markdown Structure

All generated markdown files follow this structure:

```markdown
# Item Name

Item documentation text from rustdoc.

## Type-Specific Sections

Fields, variants, or type details depending on the item type.

## Next Actions

- View source code: `cargo docmd browse --item <id>`
- Find related items: `cargo docmd browse --type <type>`
```

### Generated Item Types

#### Structs

Structs generate documentation with a **Fields** section listing all struct
fields with their types and visibility markers.

Example generated struct:

```markdown
# Point

A 2D point in Cartesian coordinates.

## Fields

- `x: f64` (pub) - X coordinate
- `y: f64` (pub) - Y coordinate

## Next Actions

- View source: `cargo docmd browse --item 0:3:4`
- Find related structs: `cargo docmd browse --type struct`
```

#### Enums

Enums generate documentation with a **Variants** section listing all enum
variants, their associated data types, and explicit discriminants.

Example generated enum:

```markdown
# Option

A type representing a value that may or may not exist.

## Variants

- `Some(T)` - Some value of type `T`
- `None` - No value

## Next Actions

- View source: `cargo docmd browse --item 0:3:5`
- Find related enums: `cargo docmd browse --type enum`
```

#### Unions

Unions generate documentation with a **Safety** warning section and a **Fields**
section. The safety note reminds users about unsafe access requirements.

Example generated union:

```markdown
# Any

A dynamically-typed value.

## Safety

**Important**: Accessing union fields requires unsafe code. Only access the
field that was most recently written to. Reading from a different field results
in undefined behavior.

## Fields

- `integer: i64` (pub)
- `float: f64` (pub)
- `text: *const u8` (pub)

## Next Actions

- View source: `cargo docmd browse --item 0:3:6`
- Find related unions: `cargo docmd browse --type union`
```

#### Type Aliases

Type aliases generate documentation with a **Type** section showing the target
type in a code block for clarity.

Example generated type alias:

````markdown
# Result

Result type alias for convenience.

## Type

```rust
type Result<T> = std::result::Result<T, Error>;
```

## Next Actions

- View source: `cargo docmd browse --item 0:3:7`
- Find related aliases: `cargo docmd browse --type type-alias`
````

## Current Limitations

This section documents current limitations of cargo docmd as of version 0.1.0.

- **Build command**: Generates markdown for type aliases only. Other item types
  (structs, enums, unions) will be added in future phases.
- **Browse command**: Accepts crate name and optional item parameter but does
  not display documentation yet.
- **Configuration**: The `--config` option is available but configuration file
  parsing is not implemented.
- **Verbosity**: The `--verbose` flag is accepted but does not affect output
  behavior yet.

## Error Handling

The CLI uses centralized error handling with the `std::error::Error` trait for
automatic error display.

### Error Architecture

All error functionality is centralized in the `src/error.rs` module, which
defines a clear error hierarchy:

- `Error` - Top-level error enum wrapping all specific error types
    - `BuildError` - Errors during the build process (cargo execution, HTML
      parsing, markdown generation, etc.)
        - May have `HtmlExtractError` as its source via
          `BuildError::HtmlParseFailed`
- `HtmlExtractError` - Low-level HTML parsing errors (selector failures, missing
  elements)

All error types implement the `std::error::Error` trait with proper `source()`
methods for error chain propagation.

### Error Hierarchy

The error structure follows a clear separation of concerns:

```
Error (top-level application error)
└── BuildError (build process errors)
    ├── CargoExecutionFailed
    ├── OutputDirCreationFailed
    ├── HtmlParseFailed (may wrap HtmlExtractError)
    │   └── HtmlExtractError (low-level HTML parsing)
    ├── DocNotGenerated
    └── MarkdownWriteFailed
```

This hierarchy ensures:

1. **Clear separation**: BuildError wraps low-level errors (HtmlExtractError) at
   appropriate boundaries
2. **Proper context**: File paths are added at the BuildError level when
   wrapping HtmlExtractError, not at the HTML extraction level
3. **Consistent propagation**: The `?` operator automatically converts errors
   through the `From` trait implementations
4. **Full error chains**: Rust's standard library displays the complete error
   hierarchy via the `Error::source()` method
5. **Simplified structure**: All build process errors (including markdown
   generation) are consolidated in BuildError

### Error Propagation

All fallible functions use `error::Result<T>` (a type alias for
`std::result::Result<T, Error>`) and propagate errors using the `?` operator,
including the main function. When the main function returns an error:

1. Rust's standard library automatically displays the error using the `Display`
   trait
2. The error chain is automatically displayed via the `Error::source()` method
3. The program exits with status code 1

#### Error Propagation Example

Here's how errors propagate through the hierarchy in a typical HTML parsing
operation:

```rust
// In src/items/type_alias.rs
pub fn from_str(html_str: &str) -> error::Result<Self> {
    let name = extract_name(&document)?;  // Returns error::Result<String>
    // ...
}

fn extract_name(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("h1 .type")?;  // HtmlExtractError converted to Error
    // ...
}

// In src/commands/build.rs
let type_alias = error::wrap_with_path(
    items::type_alias::TypeAlias::from_str(&html_content),
    path.clone(),  // Path context added here
)?;

// Error chain displayed to user:
// Error: Build(HtmlParseFailed { path: "path/to/type.html", source: ... })
// Caused by:
//   Element not found with selector 'h1 .type'
```

#### Error Conversion Chain

The `From` trait implementations enable automatic error conversion:

1. `HtmlExtractError` → `Error` via `From<HtmlExtractError>`
    - Converts to `Error::Build(HtmlExtractError.into())`
2. `BuildError` → `Error` via `From<BuildError>`
    - Wraps in `Error::Build(build_error)`

This allows the `?` operator to automatically propagate errors through the
correct conversion path.

### Common Error Messages

#### Failed Cargo Execution

If cargo doc fails (e.g., crate not found), you will see:

```
Error: Build(CargoExecutionFailed { crate_name: "crate_name", output: "<command output>" })
```

#### Documentation Not Generated

If HTML documentation was not generated for the crate, you will see:

```
Error: Build(DocNotGenerated { crate_name: "crate_name", expected_path: "path/to/doc" })
```

#### HTML Parsing Error

If HTML parsing fails, you will see the error with its full chain, demonstrating
the error hierarchy:

```
Error: Build(HtmlParseFailed { path: "path/to/type.html", source: ElementNotFound { selector: "h1 .type" } })
Caused by:
  Element not found with selector 'h1 .type'
```

For markdown write errors:

```
Error: Build(MarkdownWriteFailed { path: "path/to/Type.md", error: "Permission denied" })
```

The error shows:

1. The top-level `Error` wrapping a `BuildError`
2. The `BuildError::HtmlParseFailed` variant with the file path context
3. The underlying `HtmlExtractError::ElementNotFound` as the source via
   `source()`

This full chain provides context at each level (path at BuildError level,
selector at HtmlExtractError level) while maintaining a clean separation of
concerns.

## Exit Codes

- `0`: Command executed successfully
- `1`: Command failed (cargo execution error, HTML parsing error, etc.)

## Future Enhancements

Planned features for future versions:

- Individual item markdown generation for all 21 rustdoc item types
- Interactive terminal-based documentation browser
- Configuration file support
- Custom output formatting options
- Support for multiple crates simultaneously
- Search and filter capabilities in browse mode
- Detailed signature rendering with type information
- Cross-reference links between related items
