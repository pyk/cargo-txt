cargo docmd build <CRATE>

````

#### Arguments

- `<CRATE>`
    - **Required**: Crate name to build documentation for
    - Example: `serde`

#### Output Location

Markdown files are placed in the target directory's `docmd` subdirectory. The
target directory is determined by cargo metadata and typically resolves to
`./target/docmd/<crate>/index.md`.

#### Examples

Build documentation for serde crate:

```shell
cargo docmd build serde
````

Output:

```shell
Generated markdown: /path/to/project/target/docmd/serde/index.md
```

#### What It Does

1. Runs `cargo metadata --no-deps --format-version 1` to get project information
2. Validates that the requested crate is an installed dependency
3. Runs `cargo doc --package <crate> --no-deps` with the dependency's feature
   configuration to generate HTML
4. Parses the cargo doc output to extract the generated directory path from the
   "Generated /path/to/crate/index.html" line
5. Reads the `index.html` file from that directory
6. Extracts the `<main>` element from the HTML
7. Converts HTML to markdown using the `scraper` crate
8. Creates the output directory `target/docmd/<crate>/` if needed
9. Writes the markdown content to `target/docmd/<crate>/index.md`

#### Feature Detection

The build command automatically detects which features are enabled for a crate
from your Cargo.toml and passes them to cargo doc when generating documentation.
This ensures the generated documentation matches your project's actual feature
configuration.

- Features listed in `features = [...]` are passed as `--features feat1,feat2`
- `default-features = false` is passed as `--no-default-features`
- No feature flags are added when the dependency uses default features and has
  no specific features enabled

#### Crate Validation

The build command validates that the requested crate is an installed dependency
in your project. Only crates listed in your `Cargo.toml` (including regular,
dev, and build dependencies) can be built. You cannot build documentation for
arbitrary crates from crates.io.

If you request a crate that is not installed, you will see an error message
listing all available crates:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

#### HTML to Markdown Conversion

The build command uses a simplified approach to convert rustdoc HTML to
markdown:

1. **HTML Extraction**: Uses the `scraper` crate to parse HTML and extract the
   `<main>` element using CSS selectors
2. **Element Conversion**: Converts HTML elements to markdown equivalents:
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
3. **Recursive Processing**: Handles nested HTML elements appropriately

This approach avoids the complexity of item-specific parsing and provides
readable markdown for all rustdoc content.

### browse

Browse crate documentation in the terminal.

```shell
cargo docmd browse <CRATE>
```

#### Arguments

- `<CRATE>`
    - **Required**: Crate name to browse
    - Example: `serde`

#### Options

- `--item <ITEM>` (or `-i`)
    - **Optional**: Display documentation for a specific item only
    - Example: `--item Serialize`

#### Examples

Browse entire crate documentation:

```shell
cargo docmd browse serde
```

Display specific item documentation:

```shell
cargo docmd browse serde --item Serialize
```

#### Limitations

The browse command currently prints the received parameters but does not display
documentation. Interactive browsing functionality will be implemented in future
iterations.

## Markdown Output Format

The markdown framework generates a single index.md file containing the entire
crate documentation extracted from the `<main>` element of rustdoc HTML.

### Output Structure

The generated markdown file follows this structure:

```
target/
└── docmd/
    └── <crate>/
        └── index.md
```

For example, building documentation for `serde` produces:

```
target/
└── docmd/
    └── serde/
        └── index.md
```

### Content Format

The markdown file contains all content from the `<main>` element of the rustdoc
HTML, converted to markdown format. This includes:

- Crate-level documentation
- Module documentation
- Item documentation (structs, enums, traits, functions, etc.)
- Code examples
- Type signatures
- Links to related items

The content is rendered in a format optimized for reading by coding agents,
preserving the structure and hierarchy of the original documentation.

## Current Limitations

This section documents current limitations of cargo docmd as of version 0.1.0.

- **Build command**: Generates a single markdown file containing all
  documentation from the `<main>` element. The conversion handles common HTML
  elements but may not cover all edge cases in rustdoc HTML.
- **Browse command**: Accepts crate name and optional item parameter but does
  not display documentation yet.

## Error Handling

The CLI uses centralized error handling with the `std::error::Error` trait for
automatic error display.

### Error Architecture

All error functionality is centralized in the `src/error.rs` module, which
defines a clear error hierarchy:

- `Error` - Top-level error enum wrapping all specific error types
    - `BuildError` - Errors during the build process (cargo execution, HTML
      parsing, markdown generation, etc.)
- `HtmlExtractError` - Low-level HTML parsing errors (selector failures, missing
  elements)

All error types implement the `std::error::Error` trait with proper `source()`
methods for error chain propagation.

### Error Hierarchy

The error structure follows a clear separation of concerns:

```
Error (top-level application error)
└── BuildError (build process errors)
    ├── CargoDocExecFailed
    ├── CargoMetadataExecFailed
    ├── InvalidCrateName
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
// In src/html2md.rs
pub fn convert(html: &str) -> error::Result<String> {
    let selector = Selector::parse("main")?;  // HtmlExtractError converted to Error
    // ...
}

// In src/commands/build.rs
let markdown_content = html2md::convert(&html_content)?;
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
Error: Failed to execute cargo doc for crate 'crate_name':

error: package ID specification `crate_name` did not match any packages
```

#### Failed Cargo Metadata Execution

If the `cargo metadata` command fails (e.g., invalid Cargo.toml), you will see:

```
Error: Failed to execute cargo metadata command:

error: failed to parse manifest at `/path/to/Cargo.toml`

This may indicate an issue with your cargo installation or Cargo.toml file.
```

#### Invalid Crate Name

If you request a crate that is not an installed dependency, you will see:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

This error lists all available crates to help you identify the correct crate
name.

#### Documentation Not Generated

If HTML documentation was not generated for the crate, you will see:

```
Error: Build(DocNotGenerated { crate_name: "crate_name", expected_path: "path/to/doc" })
```

#### HTML Parsing Error

If HTML parsing fails, you will see the error with its full chain, demonstrating
the error hierarchy:

```
Error: Build(HtmlParseFailed { path: "path/to/index.html", source: ElementNotFound { selector: "main" } })
Caused by:
  Element not found with selector 'main'
```

For markdown write errors:

```
Error: Build(MarkdownWriteFailed { path: "path/to/docmd/serde/index.md", error: "Permission denied" })
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

- Interactive terminal-based documentation browser
- Configuration file support
- Custom output formatting options
- Support for multiple crates simultaneously
- Search and filter capabilities in browse mode
- Detailed signature rendering with type information
- Cross-reference links between related items
