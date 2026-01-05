# CLI Reference

This document provides detailed reference information for all cargo-docmd
command-line interface commands and options.

## Overview

cargo docmd converts rustdoc JSON output into markdown documentation optimized
for coding agents. The tool provides two primary modes of operation: generate
markdown files or browse documentation interactively.

The data source comes from rustdoc JSON output, which you can generate using:

```shell
cargo +nightly rustdoc -- --output-format json -Z unstable-options
```

For specific crates:

```shell
cargo +nightly rustdoc -p serde -- --output-format json -Z unstable-options
```

## Commands

### generate

Generate markdown documentation from rustdoc JSON files.

```shell
cargo docmd generate --crate <CRATE>
```

#### Options

- `--crate <CRATE>` (or `-c`)
    - **Required**: Crate name to generate documentation for
    - Example: `--crate serde`

- `--output <OUTPUT>` (or `-o`)
    - **Optional (advanced)**: Output directory for generated markdown files.
      Defaults to `./docs`.
    - Example: `--output ./docs/serde`

#### Examples

Generate documentation for serde crate:

```shell
# Use default output directory
cargo docmd generate --crate serde

# Specify custom output directory
cargo docmd generate --crate serde --output ./docs/serde
```

Generate with verbose output:

```shell
cargo docmd -v generate --crate serde
```

#### Limitations

The generate command currently accepts arguments but does not yet produce actual
markdown files. This is a placeholder implementation that will be completed in
future iterations.

### browse

Browse crate documentation interactively in the terminal.

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
actual documentation. Interactive browsing functionality will be implemented in
future iterations.

## Global Options

These options apply to all subcommands.

### `--verbose` / `-v`

Increase verbosity of output. Use multiple times for more verbose output (e.g.,
`-vv`, `-vvv`).

Examples:

```shell
cargo docmd -v generate --crate serde --output ./docs
cargo docmd -vv browse --crate serde
```

### `--config` / `-c`

Path to configuration file. This option is currently reserved for future
implementation.

Example:

```shell
cargo docmd --config ./config.toml generate --crate serde --output ./docs
```

#### Limitations

Configuration file parsing is not yet implemented. The `--config` option is
accepted but has no effect.

### `--help` / `-h`

Print help information for the command or subcommand.

Examples:

```shell
cargo docmd --help
cargo docmd generate --help
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

- **Generate command**: Accepts crate name and output directory but does not
  produce markdown files yet.
- **Browse command**: Accepts crate name and optional item parameter but does
  not display documentation yet.
- **Configuration**: The `--config` option is available but configuration file
  parsing is not implemented.
- **JSON source**: The tool does not automatically generate rustdoc JSON files.
  You must run the `cargo rustdoc` command manually before using cargo docmd.
- **Verbosity**: The `--verbose` flag is accepted but does not affect output
  behavior yet.

## Error Handling

The CLI uses clap for argument parsing, which provides helpful error messages
for:

- Missing required arguments
- Invalid argument values
- Unknown subcommands
- Incorrect option syntax

Example error messages:

```
error: the following required arguments were not provided:
  --crate <CRATE>

Usage: cargo docmd generate --crate <CRATE>

For more information, try '--help'.
```

```
error: unrecognized subcommand 'invalid_command'

Usage: cargo docmd [OPTIONS] <COMMAND>

For more information, try '--help'.
```

## Exit Codes

- `0`: Command executed successfully
- `1`: Command failed (argument parsing errors, missing required options, etc.)

## Future Enhancements

Planned features for future versions:

- Full markdown generation from rustdoc JSON
- Interactive terminal-based documentation browser
- Configuration file support
- Automatic rustdoc JSON generation
- Custom output formatting options
- Support for multiple crates simultaneously
- Search and filter capabilities in browse mode
