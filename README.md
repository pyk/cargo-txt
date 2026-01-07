# cargo-docmd

`cargo doc` for coding agents.

cargo-docmd generates markdown documentation from rustdoc HTML files, optimized
for consumption by coding agents. It provides both generation and browsing
capabilities.

## Features

- Generate markdown from rustdoc HTML by extracting the `<main>` element
- Documentation browsing at crate and item level

## Installation

Install the cargo-docmd binary:

```shell
cargo install docmd
```

Once installed, use it as a cargo subcommand:

```shell
cargo docmd --help
```

## Quick Start

Generate markdown documentation:

```shell
cargo docmd build serde
```

Browse crate documentation:

```shell
cargo docmd browse serde
```

## Usage

### Build Command

Generate markdown documentation for a crate:

```shell
cargo docmd build <CRATE>
```

Arguments:

- `<CRATE>` - Crate name to build documentation for (required)

This command generates HTML documentation using `cargo doc`, extracts the
`<main>` element from `index.html`, converts it to markdown, and writes it to
the output directory. Output is placed in the target directory's `docmd`
subdirectory (determined by cargo metadata, typically
`./target/docmd/<crate>/index.md`).

**Note**: Only installed dependencies listed in your `Cargo.toml` can be built.
You cannot build documentation for arbitrary crates from crates.io.

Example:

```shell
cargo docmd build serde
```

Output:

```shell
Generated markdown: /path/to/project/target/docmd/serde/index.md
```

Error example:

```shell
$ cargo docmd build random-crate
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

### Feature Detection

cargo-docmd automatically detects which features are enabled for a crate from
your Cargo.toml. When you run `cargo docmd build <crate>`, the tool:

1. Reads your Cargo.toml via cargo metadata
2. Extracts the enabled features for the specified crate
3. Passes those features to cargo doc when generating documentation

Example:

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

Running `cargo docmd build clap` will automatically use the `derive` feature
when generating documentation.

### Browse Command

Browse crate documentation:

```shell
cargo docmd browse <CRATE>
```

Arguments:

- `<CRATE>` - Crate name to browse (required)

Options:

- `-i, --item <ITEM>` - Optional specific item to display

Examples:

```shell
# Browse entire crate
cargo docmd browse serde

# Browse specific item
cargo docmd browse serde --item Serialize
```

## Current Status

- **Build command**: Fully implemented. Generates HTML documentation using
  stable `cargo doc`, extracts the `<main>` element from `index.html`, converts
  it to markdown using the `scraper` crate, and writes a single `index.md` file
  to `target/docmd/<crate>/`. The conversion handles headings, paragraphs, code
  blocks, links, lists, and other common HTML elements.
- **Browse command**: Placeholder implementation. Accepts crate name and item
  parameters but does not display documentation yet.

## Development

To install the binary locally for development:

```shell
cargo install --path .
```

This installs the `cargo-docmd` binary directly from the source code in the
current directory.

To build and run the binary without installing:

```shell
cargo run -- --help
```

Run tests:

```shell
cargo test
```

Check code with clippy:

```shell
cargo clippy
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Run tests**: Always run `cargo test` before submitting changes
2. **Check formatting**: Use `cargo clippy` to ensure code quality
3. **Update documentation**: Keep README.md and DOCS.md in sync with changes
4. **Follow Rust guidelines**: Adhere to the Rust Coding Guidelines in AGENTS.md
5. **Test locally**: Install and test the binary with `cargo install --path .`

For detailed development guidelines, see `AGENTS.md`.
