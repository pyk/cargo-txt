<p align="center">
  <a href="https://pyk.sh">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/pyk/cargo-txt/refs/heads/main/.github/logo-dark.svg">
      <img alt="pyk/cargo-txt logo" src="https://raw.githubusercontent.com/pyk/cargo-txt/refs/heads/main/.github/logo-light.svg" height="48" width="auto">
    </picture>
  </a>
</p>

<p align="center">
    <code>cargo doc</code> for coding agents
<p>

<p align="center">
  <img src="https://img.shields.io/crates/v/txt.svg?colorA=00f&colorB=fff&style=flat&logo=rust" alt="Crates.io">
  <img src="https://img.shields.io/crates/d/txt?colorA=00f&colorB=fff&style=flat&logo=rust" alt="Downloads">
  <img src="https://img.shields.io/github/license/pyk/cargo-txt?colorA=00f&colorB=fff&style=flat" alt="MIT License">
</p>

## Getting started

> [!IMPORTANT]
>
> `cargo-txt` is in early active development.

`cargo-txt` is a cargo subcommand that your LLM or Coding Agents can use to
access the crate documentation locally.

To use it, install the binary with this command:

```shell
cargo install txt
```

Add this instruction to your coding agent:

````markdown
Use `cargo txt` to access the crate documentation locally.

The workflow is:

1. Build documentation: `cargo txt build <crate>`
2. List all items: `cargo txt list <lib_name>`
3. View specific item: `cargo txt show <lib_name>::<item>`

For example:

```shell
# Build the serde crate documentation
cargo txt build serde

# List all items in serde
cargo txt list serde

# View serde crate overview
cargo txt show serde

# View serde::Deserialize trait documentation
cargo txt show serde::Deserialize
```
````

## Why I'm building this

Coding agents should use CLI for everything. Model Context Protocol servers are
overkill.

I built `cargo-txt` to feed my coding agent with up-to-date context and reduce
hallucination. It converts `cargo doc` HTML to markdown so agents can access
actual crate documentation from my local machine, not training data or outdated
information.

Agents get accurate, comprehensive documentation that matches exactly what I'm
working with. No more guessing, no more stale APIs, no more debugging code based
on hallucinated methods.

## Features

- Simple command-line interface for coding agents.
- Local documentation access in markdown format.
- Crate and item-level browsing for targeted access, reducing the token usage.
- Master index listing for comprehensive item discovery.

## Usage

### Build Command

```shell
$ cargo txt build --help
Generate markdown documentation from rustdoc HTML for coding agents

Usage: cargo txt build [OPTIONS] <CRATE>

Arguments:
  <CRATE>  Crate name to build documentation for

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

This command generates HTML documentation using `cargo doc`, converts all HTML
files to markdown, and writes them to the output directory. Output is placed in
the target directory's `docmd` subdirectory (determined by cargo metadata).

**Output Directory Structure:**

The output directory uses the **library name** (from `cargo doc` output), not
the crate name. For example, building `rustdoc-types` creates:

```
target/docmd/rustdoc_types/     # Library name directory (underscores)
├── metadata.json               # Contains crate_name, lib_name, and item_map
├── index.md                    # Crate overview
├── all.md                      # Master index of all items
└── struct.Item.md              # Individual item markdown files
```

**Note**: Only installed dependencies listed in your `Cargo.toml` can be built.
You cannot build documentation for arbitrary crates from crates.io.

Example:

```shell
# Build using crate name from Cargo.toml
cargo txt build rustdoc-types
```

Output:

```
✓ Built documentation for rustdoc_types (55 items)
  Run `cargo txt list rustdoc_types` to see all items
```

### List Command

List all items in a crate:

```shell
$ cargo txt list --help
List all items in a crate

Usage: cargo txt list [OPTIONS] <CRATE>

Arguments:
  <CRATE>  Crate name (e.g., 'serde')

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

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

Example output for `cargo txt list serde`:

````markdown
# serde

List of all items

### Structs

- serde::Error
- serde::de::IgnoredAny
- serde::ser::StdError

### Traits

- serde::Serialize
- serde::Deserialize

## Usage

To view documentation for a specific item, use the `show` command:

```shell
cargo txt show <ITEM_PATH>
```

Examples:

- Show struct: `cargo txt show serde::SomeStruct`
- Show trait: `cargo txt show serde::SomeTrait`
- Show enum: `cargo txt show serde::SomeEnum`
````

### Show Command

```shell
$ cargo txt show --help
Show and display crate documentation

Usage: cargo txt show [OPTIONS] <ITEM>

Arguments:
  <ITEM>  Item path (e.g., 'serde', 'serde::Error', 'serde::ser::StdError')

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

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

**Error Handling:**

- If `metadata.json` doesn't exist, shows error with available crate names from
  `cargo metadata`
- If item not found in metadata, suggests `cargo txt list <lib_name>` where
  lib_name comes from the metadata

### Verbosity

cargo-txt uses the `env_logger` and `log` crates for flexible logging. You can
control output verbosity using command-line flags or environment variables.

#### Command-line flags

- `-v` - Show warnings
- `-vv` - Show info messages (important milestones)
- `-vvv` - Show debug messages (detailed operational info, equivalent to old
  `--debug` flag)
- `-vvvv` - Show trace messages (very detailed diagnostic info)
- `-q, --quiet` - Suppress all output (only errors are shown)

Examples:

```shell
# Show warnings
cargo txt build serde -v

# Show info messages
cargo txt build serde -vv

# Show debug messages (equivalent to old --debug flag)
cargo txt build serde -vvv

# Show trace messages (very verbose)
cargo txt build serde -vvvv

# Suppress output (only errors)
cargo txt build serde -q
```

#### Environment variables

You can also control verbosity using the `RUST_LOG` environment variable:

```shell
# Show debug messages
RUST_LOG=debug cargo txt build serde

# Show warnings and errors only
RUST_LOG=warn cargo txt build serde

# Show info and above
RUST_LOG=info cargo txt build serde

# Show trace and above (very verbose)
RUST_LOG=trace cargo txt build serde

# Customize log levels for specific modules
RUST_LOG=txt=debug,cargo=warn cargo txt build serde
```

By default, cargo-txt shows only error messages.

## Current Status

- **Build command**: Fully implemented. Generates HTML documentation using
  stable `cargo doc`, converts HTML files to markdown, and writes:
    - `metadata.json` - Contains crate_name, lib_name, and item_map
    - `all.md` - Master index of all items from `all.html`
    - `index.md` - Crate overview from `index.html`
    - Individual item markdown files (e.g., `struct.Item.md`,
      `trait.Serialize.md`) Output directory uses library name (e.g.,
      `rustdoc_types`) instead of crate name (e.g., `rustdoc-types`).
- **List command**: Fully implemented. Lists all items in a crate by displaying
  the master index (`all.md`). Accepts library names.
- **Show command**: Fully implemented. Displays crate documentation to stdout.
  Opens crate overview (`index.md`) for library name requests or specific item
  documentation for full item paths. Uses metadata.json for fast lookups.

## Development

To install the binary locally for development:

```shell
cargo install --path .
```

This installs the `cargo-txt` binary directly from the source code in the
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
