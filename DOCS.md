# cargo-txt Documentation

cargo-txt generates markdown documentation from rustdoc HTML files. The tool
provides generation and browsing capabilities optimized for coding agents.

## Installation

Install the cargo-txt binary:

```shell
cargo install txt
```

Use it as a cargo subcommand:

```shell
cargo txt --help
```

## Crate Name vs Library Name

cargo-txt distinguishes between two types of names that often differ due to
Rust's identifier rules:

### Crate Name

The dependency name from `Cargo.toml`. For example: `rustdoc-types`

- Available from `cargo metadata`
- Used for: `cargo txt build <crate>`
- Often uses hyphens to separate words (crates.io convention)
- Examples: `rustdoc-types`, `serde-json`, `actix-web`

### Library Name

The root namespace name from `cargo doc` output. For example: `rustdoc_types`

- Extracted from cargo doc output directory name
- Used for: `cargo txt show <lib_name>::<item>` and `cargo txt list <name>`
- Always uses underscores (Rust identifier syntax)
- Matches the name you use in Rust code paths
- Examples: `rustdoc_types`, `serde_json`, `actix_web`

### Why This Distinction Matters

When you add a dependency to `Cargo.toml`:

```toml
[dependencies]
rustdoc-types = "0.57"
```

Cargo generates documentation using the library name `rustdoc_types`
(underscores), not the crate name `rustdoc-types` (hyphens). This is because:

1. **Rust identifier rules**: Rust identifiers cannot contain hyphens
2. **Cargo's conversion**: Cargo automatically converts hyphens to underscores
   when generating the library name
3. **Code references**: Your Rust code uses underscores:
   `use rustdoc_types::Item;`

### Command Usage

| Command                             | Name Type    | Example                              |
| ----------------------------------- | ------------ | ------------------------------------ |
| `cargo txt build <crate>`           | Crate name   | `cargo txt build rustdoc-types`      |
| `cargo txt list <name>`             | Library name | `cargo txt list rustdoc_types`       |
| `cargo txt show <lib_name>::<item>` | Library name | `cargo txt show rustdoc_types::Item` |

### Practical Examples

```shell
# Build using crate name (from Cargo.toml)
cargo txt build rustdoc-types

# List using library name
cargo txt list rustdoc_types

# Show using library name (required)
cargo txt show rustdoc_types::Item

# INCORRECT: Using crate name in list or show command will fail
cargo txt list rustdoc-types
# Error: Documentation for 'rustdoc-types' is not built yet.

cargo txt show rustdoc-types::Item
# Error: failed to read metadata file 'target/docmd/rustdoc-types/metadata.json'
```

### Output Directory Structure

The output directory uses the **library name**, not the crate name:

```
target/docmd/rustdoc_types/    # Library name (underscores)
├── metadata.json               # Contains crate_name and lib_name
├── index.md                    # Crate overview
├── all.md                      # Master index
└── struct.Item.md              # Item documentation
```

The `metadata.json` file contains both names:

```json
{
    "crate_name": "rustdoc-types",
    "lib_name": "rustdoc_types",
    "item_map": {
        "rustdoc_types::Item": "struct.Item.html"
    }
}
```

## Commands

### build

Generate markdown documentation for a crate:

```shell
cargo txt build <CRATE>
```

**Arguments:**

- `<CRATE>` - Crate name to build documentation for (required). This is the
  dependency name from `Cargo.toml` (e.g., `rustdoc-types`).

**Examples:**

Build documentation for the rustdoc-types crate:

```shell
cargo txt build rustdoc-types
```

Output:

```
✓ Built documentation for rustdoc_types (55 items)
  Run `cargo txt list rustdoc_types` to see all items
```

**Output Directory Structure:**

The output directory uses the **library name** (from `cargo doc` output), not
the crate name. For example, building `rustdoc-types` creates:

```
target/docmd/rustdoc_types/    # Library name directory (underscores)
├── metadata.json               # Contains crate_name, lib_name, and item_map
├── index.md                    # Crate overview
├── all.md                      # Master index of all items
└── struct.Item.md              # Individual item markdown files
```

The following files are generated:

- `all.md` - Master index of all items (from `all.html`)
- `index.md` - Crate overview (from `index.html`)
- Individual item markdown files (e.g., `struct.Error.md`, `trait.Serialize.md`)
  with preserved directory structure

**Crate Validation:**

You can only build documentation for installed dependencies listed in your
`Cargo.toml`. You cannot build documentation for arbitrary crates from
crates.io.

If you request a crate that is not installed, you will see an error message:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

**What It Does:**

1. Runs `cargo metadata --no-deps --format-version 1` to get project information
2. Validates that the requested crate is an installed dependency
3. Runs `cargo doc --package <crate> --no-deps` to generate HTML
4. Parses the cargo doc output to extract the generated directory path
5. Reads the `index.html` file from that directory
6. Extracts the `<main>` element from the HTML
7. Converts HTML to markdown using the `scraper` crate
8. Creates the output directory `target/docmd/<crate>/` if needed
9. Writes the markdown content to `target/docmd/<crate>/index.md`
10. Reads the `all.html` file and converts it to `all.md`
11. Extracts item mappings from `all.html` (full Rust paths to HTML file paths)
12. Generates markdown for each individual item, preserving directory structure

**Item Mapping and Metadata:**

During build, the command parses `all.html` to extract mappings between full
Rust paths and HTML file paths, then saves them in `metadata.json`. For example:

```json
{
    "crate_name": "rustdoc-types",
    "lib_name": "rustdoc_types",
    "item_map": {
        "rustdoc_types::Item": "struct.Item.html",
        "rustdoc_types::Abi": "enum.Abi.html"
    }
}
```

These mappings are used by the show command to quickly resolve item paths to
markdown files without parsing HTML at runtime.

### list

List all items in a crate:

```shell
cargo txt list <NAME>
```

**Arguments:**

- `<NAME>` - Library name to list items for (required). This is the library name
  from `cargo doc` output (e.g., `rustdoc_types`).

**Examples:**

List all items:

```shell
cargo txt list rustdoc_types
```

**How It Works:**

1. Attempts to read `metadata.json` from `docmd/<lib_name>/metadata.json`
2. If not found, shows error with available crate names from `cargo metadata`
3. Reads and displays `all.md` from the library name directory

**Output Format:**

The list command displays formatted output with:

- Library name as H1 heading at the top
- All list items prefixed with the library name (e.g., `rustdoc_types::Item`)
- Usage instructions at the bottom with `cargo txt show` examples

Example output for `cargo txt list rustdoc_types`:

````markdown
# rustdoc_types

List of all items

### Structs

- rustdoc_types::Item
- rustdoc_types::Function
- rustdoc_types::Struct

### Enums

- rustdoc_types::Type
- rustdoc_types::ItemKind
- rustdoc_types::Visibility

## Usage

To view documentation for a specific item, use the `show` command:

```shell
cargo txt show <ITEM_PATH>
```
````

Examples:

- Show struct: `cargo txt show rustdoc_types::Item`
- Show enum: `cargo txt show rustdoc_types::Type`
- Show trait: `cargo txt show rustdoc_types::Visibility`

**Auto-Build:**

The list command no longer auto-builds documentation. You must run
`cargo txt build <crate>` first. This provides clearer error messages and better
user control.

**Error Handling:**

If you provide a path with `::`, you will see an error message:

```
Error: Documentation for 'rustdoc-types::Item' is not built yet. Run 'cargo txt build <crate>` for one of the following crates: rustdoc-types, serde, serde_json
```

If the documentation doesn't exist, you will see:

```
Error: Documentation for 'unknown_crate' is not built yet. Run 'cargo txt build <crate>` for one of the following crates: rustdoc-types, serde, serde_json
```

### show

Show and view crate documentation:

```shell
cargo txt show <ITEM_PATH>
```

**Arguments:**

- `<ITEM_PATH>` - Item path to show (required). Can be:
    - Library name only (e.g., `rustdoc_types`): displays crate overview
      (index.md)
    - Full item path with library name (e.g., `rustdoc_types::Item`,
      `rustdoc_types::Abi`): displays specific item documentation

**Important:** Use the library name (with underscores), not the crate name (with
hyphens). For example:

- Use `rustdoc_types::Item` (library name)
- NOT `rustdoc-types::Item` (crate name - this will fail)

**Examples:**

View crate overview:

```shell
cargo txt show rustdoc_types
```

View specific item documentation:

```shell
cargo txt show rustdoc_types::Item
cargo txt show rustdoc_types::Abi
```

**How It Works:**

1. Parses the item path to extract library name and optional item
2. Checks if markdown documentation exists by reading `metadata.json`
3. If item path is just a library name: reads and displays `index.md` (crate
   overview)
4. If item path includes modules/items:
    - Reads `metadata.json` to get the item map
    - Looks up the exact markdown file for the requested item
    - Displays the contents

**Auto-Build:**

The show command no longer auto-builds documentation. You must run
`cargo txt build <crate>` first. This provides clearer error messages and better
user control.

**Error Handling:**

If `metadata.json` doesn't exist, shows error with available crate names from
`cargo metadata`:

```
Error: failed to read metadata file 'target/docmd/rustdoc_types/metadata.json'

Documentation for library 'rustdoc_types' is not built yet. Run 'cargo txt build <crate>` for one of the following crates: rustdoc-types, serde, serde_json
```

If item not found in metadata, suggests list command with library name:

```
Error: could not resolve item path 'rustdoc_types::NonExistent'. The item may not exist. Try: `cargo txt list rustdoc_types` to see all available items.
```

## Markdown Output Format

The build command generates comprehensive markdown documentation including a
master index and individual item files.

### Output Structure

```
target/
└── docmd/
    └── rustdoc_types/              # Library name directory (underscores)
        ├── metadata.json           # Crate name and item mappings
        ├── all.md                  # Master index of all items
        ├── index.md                # Crate overview
        ├── struct.Item.md          # Individual item files
        ├── enum.Abi.md
        └── ...
```

### HTML to Markdown Conversion

The build command uses the `scraper` crate to convert rustdoc HTML to markdown:

1. **HTML Extraction**: Parses HTML and extracts the `<main>` element using CSS
   selectors
2. **Element Conversion**: Converts HTML elements to markdown equivalents:
    - `<h1>`-`<h6>` → `#` to `######` headings
    - `<p>` → paragraph text with newline
    - `<code>` → inline code with backticks
    - `<pre><code>` → code blocks with triple backticks
    - `<a>` → inner content only
    - `<ul>`/`<ol>` → bullet/numbered lists
    - `<li>` → list items with proper indentation
    - `<dl>` → definition lists as nested bullet lists
    - `<dt>` → definition terms as bold items
    - `<dd>` → definition descriptions indented
    - `<strong>`/`<b>` → **bold** text
    - `<em>`/`<i>` → _italic_ text
    - `<blockquote>` → quoted text with `>`
3. **UI Element Filtering**: Skips elements that should not appear in
   documentation:
    - Elements with `id="copy-path"` (copy buttons)
    - Elements with `class="src"` (source links)
    - Elements with `class="hideme"` (expandable sections)
    - Elements with `class="anchor"` (anchor links)
    - `<wbr>` elements (word break opportunities)
    - `<rustdoc-toolbar>` elements (toolbar UI)
4. **Text Processing**: Handles special text entities:
    - Non-breaking spaces converted to regular spaces

## Verbosity

cargo-txt uses `env_logger` and `log` for flexible logging. Control output
verbosity with command-line flags or environment variables.

### Command-line Flags

- `-v` - Show warnings
- `-vv` - Show info messages
- `-vvv` - Show debug messages
- `-vvvv` - Show trace messages
- `-q, --quiet` - Suppress output (errors only)

**Examples:**

```shell
cargo txt build serde -v
cargo txt build serde -vv
cargo txt build serde -q
```

### Environment Variables

Control verbosity using the `RUST_LOG` environment variable:

```shell
RUST_LOG=debug cargo txt build serde
RUST_LOG=warn cargo txt build serde
RUST_LOG=info cargo txt build serde
RUST_LOG=trace cargo txt build serde
```

Customize log levels for specific modules:

```shell
RUST_LOG=txt=debug,cargo=warn cargo txt build serde
```

By default, cargo-txt shows only error messages.

## Error Handling

cargo-txt uses the `anyhow` crate for error handling, providing uniform error
handling with automatic source chaining and context addition. Errors are
displayed with clear context and full error chains when available.

### Error Chain Format

anyhow automatically formats error chains with "Caused by:" sections. Each layer
of the application adds appropriate context to help you understand where and why
errors occurred:

```
Error: failed to execute cargo doc for crate 'serde'

Caused by:
    failed to execute cargo doc command: error: package ID specification `serde` did not match any packages
```

### Common Errors

#### Failed Cargo Execution

If `cargo doc` fails (crate not found), you will see:

```
Error: failed to execute cargo doc for crate 'crate_name':

error: package ID specification `crate_name` did not match any packages
```

#### Failed Cargo Metadata Execution

If `cargo metadata` fails (invalid `Cargo.toml`), you will see:

```
Error: failed to execute cargo metadata command:

error: failed to parse manifest at `/path/to/Cargo.toml`
```

#### Invalid Crate Name

If you request a crate that is not an installed dependency, you will see:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: anyhow, clap, clap-verbosity-flag, env_logger, log, rustdoc-types, scraper, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

#### HTML Parsing Error

If HTML parsing fails, you will see the error with its full chain:

```
Error: failed to read file 'path/to/index.html'

Caused by:
    No such file or directory (os error 2)
```

or for selector errors:

```
Error: failed to parse HTML selector for item mappings: expected '>'
```

#### Documentation Index Not Found

If the documentation index file cannot be found, you will see:

```
Error: failed to read documentation index file 'path/to/target/doc/serde/all.html'

Caused by:
    No such file or directory (os error 2)
```

#### Item Path Resolution Failed

If you request an item that doesn't exist, you will see:

```
Error: could not resolve item path 'serde::NonExistent'. Please ensure the item exists in the crate and try: `cargo txt build serde`
```

#### Invalid Item Path Format

If you provide an invalid item path format, you will see:

```
Error: invalid item path '::invalid'. Expected format: <crate> or <crate>::<item> (e.g., 'serde' or 'serde::Error').
```

### Exit Codes

- `0`: Command executed successfully
- `1`: Command failed

## Current Limitations

- **Build command**: Generates comprehensive markdown documentation including
  `metadata.json`, `all.md`, `index.md`, and individual item files. The
  conversion handles common HTML elements but may not cover all edge cases in
  rustdoc HTML. Output directory uses library name (e.g., `rustdoc_types`)
  instead of crate name (e.g., `rustdoc-types`).
- **Show command**: Fully implemented. Displays crate documentation to stdout.
  Uses `metadata.json` for fast lookups. Shows crate overview (`index.md`) for
  library name requests or specific item documentation for full item paths.
  Requires `cargo txt build <crate>` to be run first.
- **List command**: Fully implemented. Lists all items in a crate by displaying
  the master index (`all.md`). Accepts library names. Requires
  `cargo txt build <crate>` to be run first.
