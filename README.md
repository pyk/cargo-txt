<p align="center">
  <a href="https://pyk.sh">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/pyk/cargo-txt/refs/heads/main/.github/logo-dark.svg">
      <img alt="pyk/cargo-txt logo" src="https://raw.githubusercontent.com/pyk/cargo-txt/refs/heads/main/.github/logo-light.svg" height="64" width="auto">
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

> [!WARNING]
>
> `cargo-txt` is in early active development. Expect breaking changes on every
> release.

`cargo-txt` is a cargo subcommand that your LLM or Coding Agents can use to
access the crate documentation locally.

To use it, install the binary with this command:

```shell
cargo install txt
```

Add this instruction to your coding agent:

```markdown
Use `cargo txt show <crate-item>` to access the crate documentation.

For example:

- Access serde documentation index: `cargo txt show serde`
- Access `serde::Deserialize` trait doc: `cargo txt show serde::Deserialize`
```

## Why I'm building this

I work with [coding agents](https://zed.dev/agentic-engineering) daily now, and
I constantly hit the same wall. The agents try to help me write code, but they
keep making mistakes because they're working from outdated or incomplete
information in their training data. They hallucinate APIs that don't exist, miss
important details, or suggest deprecated methods.

The problem gets worse with complex crates. An agent might think it knows a
crate's API, but it's working with knowledge that's months or years out of date.
This leads to wasted time debugging code that never had a chance of working
correctly.

I needed a way to give agents access to the actual documentation, the real,
current docs locally. Not a cached version, not training data, but the latest
documentation.

That's why I built `cargo-txt`. It converts `cargo doc` HTML to markdown so
coding agents can read and understand crate documentation directly from my local
machine. No hallucinations, no outdated info, accurate, comprehensive
documentation that matches exactly what I'm working with.

Instead of agents guessing or working from stale knowledge, they now have full
access to the complete documentation. Debugging becomes faster, code is more
accurate, and the frustrating back-and-forth cycle of trial and error reduced.

## Features

- Simple command-line interface for coding agent.
- Local documentation access in markdown format.
- Crate and item-level browsing for targeted access, reducing the token usage.

## Usage

### Show Command

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
don't need to run `cargo txt build` separately.

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

### Build Command

Generate markdown documentation for a crate:

```shell
cargo txt build <CRATE>
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
cargo txt build serde
```

Output:

```shell
Generated markdown: /path/to/project/target/docmd/serde/index.md
```

Error example:

```shell
$ cargo txt build random-crate
Error: Crate 'random-crate' is not an installed dependency.

Available crates: clap, rustdoc-types, serde, serde_json, tempfile

Only installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.
```

## Current Status

- **Show command**: Fully implemented. Displays crate documentation to stdout.
  Opens master index (`all.md`) for crate-level requests or specific item
  documentation for full item paths. Auto-builds documentation when needed.
- **Build command**: Fully implemented. Generates HTML documentation using
  stable `cargo doc`, converts HTML files to markdown, and writes:
    - `all.md` - Master index of all items from `all.html`
    - `index.md` - Crate overview from `index.html`
    - Individual item markdown files (e.g., `struct.Error.md`,
      `trait.Serialize.md`)

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
