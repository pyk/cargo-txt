# cargo-docmd

`cargo doc` for coding agents.

cargo-docmd generates markdown documentation from rustdoc HTML files, optimized
for consumption by coding agents. It provides both generation and browsing
capabilities.

## Features

- Generate markdown from rustdoc HTML
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

This command generates HTML documentation using `cargo doc` and parses it to
create markdown files. Output is placed in `$CARGO_TARGET_DIR/docmd` (defaults
to `./target/docmd`).

Example:

```shell
cargo docmd build serde
```

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

- **Build command**: Fully implemented for type aliases. Generates HTML
  documentation using stable `cargo doc`, parses type alias HTML files, and
  creates markdown output following the specified format. Other item types
  (structs, enums, unions) will be added in future phases.
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
