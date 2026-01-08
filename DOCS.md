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

## Commands

### show

Show and view crate documentation:

```shell
cargo txt show <ITEM_PATH>
```

**Arguments:**

- `<ITEM_PATH>` - Item path to show (required). Can be:
    - Crate name only (e.g., `serde`): displays master index of all items
    - Full item path (e.g., `serde::Error`, `serde::ser::StdError`): displays
      specific item documentation

**Examples:**

View all items in a crate:

```shell
cargo txt show serde
```

View specific item documentation:

```shell
cargo txt show serde::Error
cargo txt show serde::ser::StdError
```

**How It Works:**

1. Parses the item path to extract crate name and optional item
2. Checks if markdown documentation exists (auto-builds if needed)
3. If item path is just a crate name: reads and displays `all.md` (master index)
4. If item path includes modules/items:
    - Reads `all.html` to extract item mappings
    - Looks up the exact HTML file for the requested item
    - Converts HTML path to markdown path and displays contents

**Auto-Build:**

The show command automatically builds documentation if it doesn't exist. You
don't need to run `cargo txt build` separately. It checks for the existence of
`target/docmd/<crate>/all.md` and triggers a build if the file is missing.

**Error Handling:**

If you request a crate that is not installed, you will see an error message:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

If you request an item that doesn't exist, you will see:

```
Error: Could not resolve item path 'serde::NonExistent'.

Attempted paths:
  - /path/to/target/docmd/serde/struct.NonExistent.md
```

### build

Generate markdown documentation for a crate:

```shell
cargo txt build <CRATE>
```

**Arguments:**

- `<CRATE>` - Crate name to build documentation for (required)

**Examples:**

Build documentation for the serde crate:

```shell
cargo txt build serde
```

Output:

```
Generated markdown: /path/to/project/target/docmd/serde/index.md
Generated markdown: /path/to/project/target/docmd/serde/all.md
Generated markdown for 42 items
```

**Output Location:**

Markdown files are placed in `target/docmd/<crate>/`. The target directory is
determined by cargo metadata. The following files are generated:

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

**Item Mapping:**

During build, the command parses `all.html` to extract mappings between full
Rust paths and HTML file paths. For example:

- `serde::Error` -> `struct.Error.html`
- `serde::ser::StdError` -> `ser/trait.StdError.html`

These mappings are used by the show command to quickly resolve item paths to
markdown files.

## Markdown Output Format

The build command generates comprehensive markdown documentation including a
master index and individual item files.

### Output Structure

```
target/
└── docmd/
    └── <crate>/
        ├── all.md                   # Master index of all items
        ├── index.md                 # Crate overview
        ├── struct.Error.md          # Individual item files
        ├── trait.Serialize.md
        └── ser/
            └── trait.StdError.md   # Nested modules preserved
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

### Common Errors

#### Failed Cargo Execution

If `cargo doc` fails (crate not found), you will see:

```
Error: Failed to execute cargo doc for crate 'crate_name':

error: package ID specification `crate_name` did not match any packages
```

#### Failed Cargo Metadata Execution

If `cargo metadata` fails (invalid `Cargo.toml`), you will see:

```
Error: Failed to execute cargo metadata command:

error: failed to parse manifest at `/path/to/Cargo.toml`
```

#### Invalid Crate Name

If you request a crate that is not an installed dependency, you will see:

```
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

#### HTML Parsing Error

If HTML parsing fails, you will see the error with its full chain:

```
Error: Build(HtmlParseFailed { path: "path/to/index.html", source: ElementNotFound { selector: "main" } })
Caused by:
  Element not found with selector 'main'
```

### Exit Codes

- `0`: Command executed successfully
- `1`: Command failed

## Current Limitations

- **Build command**: Generates comprehensive markdown documentation including
  `all.md`, `index.md`, and individual item files. The conversion handles common
  HTML elements but may not cover all edge cases in rustdoc HTML.
- **Show command**: Fully implemented. Displays crate documentation to stdout
  with auto-build functionality.
