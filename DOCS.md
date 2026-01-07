# cargo-docmd Documentation

cargo-docmd generates markdown documentation from rustdoc HTML files. The tool
provides generation and browsing capabilities optimized for coding agents.

## Installation

Install the cargo-docmd binary:

```shell
cargo install docmd
```

Use it as a cargo subcommand:

```shell
cargo docmd --help
```

## Commands

### build

Generate markdown documentation for a crate:

```shell
cargo docmd build <CRATE>
```

**Arguments:**

- `<CRATE>` - Crate name to build documentation for (required)

**Examples:**

Build documentation for the serde crate:

```shell
cargo docmd build serde
```

Output:

```
Generated markdown: /path/to/project/target/docmd/serde/index.md
```

**Output Location:**

Markdown files are placed in `target/docmd/<crate>/index.md`. The target
directory is determined by cargo metadata.

**Feature Detection:**

The build command automatically detects enabled features from your `Cargo.toml`.
When you run `cargo docmd build <crate>`, the tool passes the crate's features
to `cargo doc`.

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

### browse

Browse crate documentation:

```shell
cargo docmd browse <CRATE>
```

**Arguments:**

- `<CRATE>` - Crate name to browse (required)

**Options:**

- `--item <ITEM>` (or `-i`) - Display documentation for a specific item
  (optional)

**Examples:**

Browse entire crate documentation:

```shell
cargo docmd browse serde
```

Display specific item documentation:

```shell
cargo docmd browse serde --item Serialize
```

**Current Status:**

The browse command accepts parameters but does not display documentation yet.
Interactive browsing will be implemented in a future release.

## Markdown Output Format

The build command generates a single `index.md` file containing the entire crate
documentation.

### Output Structure

```
target/
└── docmd/
    └── <crate>/
        └── index.md
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

cargo-docmd uses `env_logger` and `log` for flexible logging. Control output
verbosity with command-line flags or environment variables.

### Command-line Flags

- `-v` - Show warnings
- `-vv` - Show info messages
- `-vvv` - Show debug messages
- `-vvvv` - Show trace messages
- `-q, --quiet` - Suppress output (errors only)

**Examples:**

```shell
cargo docmd build serde -v
cargo docmd build serde -vv
cargo docmd build serde -q
```

### Environment Variables

Control verbosity using the `RUST_LOG` environment variable:

```shell
RUST_LOG=debug cargo docmd build serde
RUST_LOG=warn cargo docmd build serde
RUST_LOG=info cargo docmd build serde
RUST_LOG=trace cargo docmd build serde
```

Customize log levels for specific modules:

```shell
RUST_LOG=docmd=debug,cargo=warn cargo docmd build serde
```

By default, cargo-docmd shows only error messages.

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

- **Build command**: Generates a single markdown file containing all
  documentation from the `<main>` element. The conversion handles common HTML
  elements but may not cover all edge cases in rustdoc HTML.
- **Browse command**: Accepts crate name and optional item parameter but does
  not display documentation yet.
