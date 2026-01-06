# CLI Reference

This document provides detailed reference information for all cargo-docmd
command-line interface commands and options.

## Overview

cargo docmd converts rustdoc JSON output into markdown documentation optimized
for coding agents. The tool provides two primary modes of operation: build
markdown files or browse documentation.

The build command automatically generates rustdoc JSON using the nightly
toolchain and creates markdown files in `$CARGO_TARGET_DIR/docmd`.

## Commands

### build

Generate rustdoc JSON and create markdown documentation from it.

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

1. Checks that the nightly toolchain is installed
2. Runs
   `cargo +nightly rustdoc -p <crate> -- --output-format json -Z unstable-options`
3. Parses the generated JSON file
4. Creates the output directory if needed
5. Logs a summary of parsed items by type

#### Requirements

- Rust nightly toolchain must be installed. Install it with:
    ```shell
    rustup install nightly
    ```

#### Limitations

The build command generates rustdoc JSON and parses it, but markdown file
generation is not yet implemented. The command creates the output directory and
logs item counts to verify the JSON was parsed correctly.

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

## Current Limitations

This section documents the current limitations of cargo docmd as of version
0.1.0.

- **Build command**: Generates rustdoc JSON and parses it, but does not yet
  produce markdown files. It creates the output directory and logs item counts.
- **Browse command**: Accepts crate name and optional item parameter but does
  not display documentation yet.
- **Configuration**: The `--config` option is available but configuration file
  parsing is not implemented.
- **Verbosity**: The `--verbose` flag is accepted but does not affect output
  behavior yet.

## Error Handling

The CLI provides helpful error messages for common issues.

### Missing Nightly Toolchain

If nightly is not installed, you will see:

```
Error: Nightly toolchain is not installed. Install it with: rustup install nightly
```

### Failed Cargo Execution

If cargo rustdoc fails (e.g., crate not found), you will see:

```
Error: Failed to execute cargo rustdoc for crate 'crate_name':
<command output>
```

### JSON Not Found

If the expected JSON file is missing, you will see:

```
Error: Expected JSON file not found at 'path/to/json'
```

## Exit Codes

- `0`: Command executed successfully
- `1`: Command failed (missing nightly, cargo execution error, JSON parsing
  error, etc.)

## Future Enhancements

Planned features for future versions:

- Full markdown generation from rustdoc JSON
- Interactive terminal-based documentation browser
- Configuration file support
- Custom output formatting options
- Support for multiple crates simultaneously
- Search and filter capabilities in browse mode
