# cargo-docmd

`cargo doc` for coding agents.

cargo-docmd generates markdown documentation from rustdoc JSON files, optimized
for consumption by coding agents. It provides both generation and interactive
browsing capabilities.

## Features

- Generate markdown from rustdoc JSON
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

Generate markdown documentation from rustdoc JSON:

```shell
cargo docmd generate --crate serde
```

Browse crate documentation interactively:

```shell
cargo docmd browse --crate serde
```

## Usage

### Generate Command

Generate markdown documentation for a crate:

```shell
cargo docmd generate --crate <CRATE>
```

Options:

- `-c, --crate <CRATE>` - Crate name to generate documentation for (required)
- `-o, --output <OUTPUT>` - Output directory for generated markdown (optional,
  advanced)
- `-v, --verbose...` - Increase verbosity of output

Example:

```shell
cargo docmd generate --crate serde --output ./serde-docs
```

### Browse Command

Browse crate documentation interactively:

```shell
cargo docmd browse --crate <CRATE>
```

Options:

- `-c, --crate <CRATE>` - Crate name to browse (required)
- `-i, --item <ITEM>` - Optional specific item to display
- `-v, --verbose...` - Increase verbosity of output

Examples:

```shell
# Browse entire crate
cargo docmd browse --crate serde

# Browse specific item
cargo docmd browse --crate serde --item Serialize
```

### Global Options

These options apply to all subcommands:

- `-v, --verbose...` - Increase verbosity of output (use multiple times: -vv,
  -vvv)
- `-c, --config <CONFIG>` - Path to configuration file
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Current Status

The CLI structure is implemented with placeholder handlers. The `generate` and
`browse` commands accept arguments but do not yet produce actual documentation.

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
