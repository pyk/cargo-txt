---
type: normal
title: "Rename crate to txt"
seq: 012
slug: "rename-crate-to-txt"
created: "2026-01-08T07:27:00Z"
status: completed
---

# Rename crate to txt

This plan renames the crate from "docmd" to "txt" and the binary from
"cargo-docmd" to "cargo-txt" to align with the available crates.io package name.
Users will install with `cargo install txt` and run with `cargo txt`.

## Current Problems

The crate is named "docmd" in Cargo.toml, which doesn't match the intended "txt"
name available on crates.io. The binary name is "cargo-docmd" but should be
"cargo-txt".

```toml
# Cargo.toml - Current state
[package]
name = "docmd"

[[bin]]
name = "cargo-docmd"
```

```rust
// src/main.rs - Current state
#[command(name = "cargo docmd")]
#[command(bin_name = "cargo docmd")]
```

Documentation throughout the codebase references "cargo-docmd" and "docmd",
which need updating to reflect the new names.

## Proposed Solution

1. Update Cargo.toml to rename the package to "txt" and binary to "cargo-txt"
2. Update CLI configuration in src/main.rs to use new names
3. Update all source code doc comments that reference the old names
4. Update README.md with new installation instructions and command examples
5. Update DOCS.md with new installation instructions and command examples
6. Test that the binary builds and runs with the new name

## Analysis Required

### Dependency Investigation

- [ ] Check for any external documentation or URLs that reference the old names

### Code Locations to Check

- `Cargo.toml` - Package name and binary name configuration
- `src/main.rs` - CLI configuration and module doc comments
- `src/commands/*.rs` - Module doc comments referencing the old name
- `README.md` - Installation instructions and command examples
- `DOCS.md` - Installation instructions and command examples

## Implementation Checklist

### Code Changes

- [x] Update `Cargo.toml`: Change `name = "docmd"` to `name = "txt"`
- [x] Update `Cargo.toml`: Change `[[bin]] name = "cargo-docmd"` to
      `name = "cargo-txt"`
- [x] Update `src/main.rs`: Change module doc comment from "cargo-docmd" to
      "cargo-txt"
- [x] Update `src/main.rs`: Change `#[command(name = "cargo docmd")]` to
      `#[command(name = "cargo txt")]`
- [x] Update `src/main.rs`: Change `#[command(bin_name = "cargo docmd")]` to
      `#[command(bin_name = "cargo txt")]`
- [x] Check and update `src/commands/build.rs` doc comments for any
      "cargo-docmd" or "docmd" references
- [x] Check and update `src/commands/open.rs` doc comments for any "cargo-docmd"
      or "docmd" references
- [x] Check and update `src/commands/mod.rs` doc comments for any "cargo-docmd"
      or "docmd" references
- [x] Check and update `src/cargo.rs` doc comments for any "cargo-docmd" or
      "docmd" references
- [x] Check and update `src/error.rs` doc comments for any "cargo-docmd" or
      "docmd" references
- [x] Check and update `src/html2md.rs` doc comments for any "cargo-docmd" or
      "docmd" references

### Documentation Updates

- [x] Update `README.md`: Change title from "cargo-docmd" to "cargo-txt"
- [x] Update `README.md`: Change installation from "cargo install docmd" to
      "cargo install txt"`
- [x] Update `README.md`: Replace all "cargo docmd" command examples with "cargo
      txt"
- [x] Update `README.md`: Update all other references to "cargo-docmd" or
      "docmd" throughout the document
- [x] Update `DOCS.md`: Change title from "cargo-docmd" to "cargo-txt"
- [x] Update `DOCS.md`: Change installation from "cargo install docmd" to "cargo
      install txt"`
- [x] Update `DOCS.md`: Replace all "cargo docmd" command examples with "cargo
      txt"`
- [x] Update `DOCS.md`: Update all other references to "cargo-docmd" or "docmd"
      throughout the document

### Test Updates

- [x] Run `cargo build` to verify compilation succeeds with new names
- [x] Run `cargo run -- --help` to verify CLI help text shows "cargo txt"
- [x] Run `cargo test` to ensure all tests pass
- [x] Run `cargo install --path .` to verify binary installs as "cargo-txt"
- [x] Verify `cargo txt --help` works after installation
- [x] Verify no changes were made to `.zed/agent/**/*.md` files (preserve
      historical references)

## Test Plan

### Verification Tests

- [x] Verify `cargo build` completes without errors
- [x] Verify `cargo run -- --help` displays "Usage: cargo txt [OPTIONS]"
- [x] Verify `cargo run -- build --help` displays build subcommand help
- [x] Verify `cargo run -- open --help` displays open subcommand help
- [x] Verify `cargo test` passes all unit tests
- [x] Verify binary installs with correct name: `which cargo-txt` after
      `cargo install --path .`
- [x] Verify installed binary works: `cargo txt --help` displays help

### Regression Tests

- [x] Verify build command still works: `cargo txt build <crate>`
- [x] Verify open command still works: `cargo txt open <item_path>`
- [x] Verify error messages still display correctly
- [x] Verify markdown generation still produces correct output

## Structure After Changes

### File Structure

```
cargo-txt/
├── Cargo.toml                 # Updated: name = "txt", binary name = "cargo-txt"
├── src/
│   ├── main.rs                # Updated: CLI names, module docs, and cargo subcommand pattern
│   ├── commands/
│   │   ├── build.rs           # Updated: doc comments if needed
│   │   ├── open.rs            # Updated: doc comments if needed
│   │   └── mod.rs             # Updated: doc comments if needed
│   ├── cargo.rs               # Updated: doc comments if needed
│   ├── error.rs               # Updated: doc comments if needed
│   └── html2md.rs             # Updated: doc comments if needed
├── README.md                  # Updated: all references to cargo-txt and txt
├── DOCS.md                    # Updated: all references to cargo-txt and txt
└── .zed/agent/                # UNCHANGED: keep historical references
```

### Cargo Subcommand Pattern Implementation

The main.rs file implements the cargo subcommand pattern with a simple approach:
remove the "txt" argument when called via cargo and always show "cargo txt" as
the bin name:

```rust
/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo txt")]
#[command(bin_name = "cargo txt")]
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(subcommand)]
    command: Command,
}

fn main() -> error::Result<()> {
    // 1. Collect arguments
    let mut args: Vec<String> = std::env::args().collect();

    // 2. If called via `cargo txt`, Cargo appends subcommand name ("txt") as first arg.
    // We need to remove it so our actual subcommands (build, open) are recognized.
    if args.len() > 1 && args[1] == "txt" {
        args.remove(1);
    }

    // 3. Parse modified arguments using parse_from
    let args = Args::parse_from(&args);

    let verbosity_level = args.verbosity.log_level_filter().to_string();
    let env = env_logger::Env::default().default_filter_or(verbosity_level);
    env_logger::Builder::from_env(env).init();

    match args.command {
        Command::Build { crate_name } => build(crate_name)?,
        Command::Open { item_path } => open(item_path)?,
    }

    Ok(())
}
```

This implementation:

1. Uses `bin_name = "cargo txt"` so help text always shows "cargo txt"
2. Removes the "txt" argument when called via cargo subcommand
3. Works identically whether called as `cargo txt` or `cargo-txt`

Users can run the tool using both:

- `cargo txt <command>` (as a cargo subcommand)
- `cargo-txt <command>` (as a standalone binary)

### Key Changes

```toml
# BEFORE - Cargo.toml
[package]
name = "docmd"
version = "0.1.0"

[[bin]]
name = "cargo-docmd"

# AFTER - Cargo.toml
[package]
name = "txt"
version = "0.1.0"

[[bin]]
name = "cargo-txt"
```

````rust
// BEFORE - src/main.rs
//! cargo-docmd: A cargo doc for coding agents

#[command(name = "cargo docmd")]
#[command(bin_name = "cargo docmd")]

```rust
// AFTER - src/main.rs
//! cargo-txt: A cargo doc for coding agents

/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo txt")]
#[command(bin_name = "cargo txt")]
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(subcommand)]
    command: Command,
}

fn main() -> error::Result<()> {
    // 1. Collect arguments
    let mut args: Vec<String> = std::env::args().collect();

    // 2. If called via `cargo txt`, Cargo appends subcommand name ("txt") as first arg.
    // We need to remove it so our actual subcommands (build, open) are recognized.
    if args.len() > 1 && args[1] == "txt" {
        args.remove(1);
    }

    // 3. Parse modified arguments using parse_from
    let args = Args::parse_from(&args);

    let verbosity_level = args.verbosity.log_level_filter().to_string();
    let env = env_logger::Env::default().default_filter_or(verbosity_level);
    env_logger::Builder::from_env(env).init();

    match args.command {
        Command::Build { crate_name } => build(crate_name)?,
        Command::Open { item_path } => open(item_path)?,
    }

    Ok(())
}
````

```shell
# BEFORE - Installation
cargo install docmd

# AFTER - Installation
cargo install txt
```

```shell
# BEFORE - Usage
cargo docmd build serde
cargo docmd open serde::Error

# AFTER - Usage
cargo txt build serde
cargo txt open serde::Error
```

## Design Considerations

1. **Preserve Agent File References**: Why keep "cargo-docmd" in
   `.zed/agent/**/*.md`?
    - **Historical Context**: These files document the development history and
      should not be rewritten
    - **Clarity**: Helps understand the evolution of the project
    - **Resolution**: Explicitly exclude `.zed/agent/**/*.md` from updates

2. **Binary Naming Convention**: Why use "cargo-txt" instead of just "txt"?
    - **Cargo Subcommand**: Binary name must start with "cargo-" to be used as a
      cargo subcommand
    - **User Expectation**: Matches the pattern of other cargo subcommands
      (e.g., `cargo-watch`, `cargo-expand`)
    - **Resolution**: Binary name is "cargo-txt", users run it with `cargo txt`

3. **Crate Name vs Binary Name**: Why different names?
    - **Crate Name**: "txt" - Simple, matches the package published to crates.io
    - **Binary Name**: "cargo-txt" - Follows cargo subcommand convention
    - **Installation**: `cargo install txt` installs the "cargo-txt" binary
    - **Usage**: `cargo txt` runs the binary as a subcommand

## Success Criteria

- Crate name is "txt" in Cargo.toml
- Binary name is "cargo-txt" in Cargo.toml
- CLI help displays "cargo txt" in command name
- Users can install with `cargo install txt`
- Users can run with `cargo txt <command>` (cargo subcommand pattern)
- Users can run with `cargo-txt <command>` (standalone binary)
- Help text always displays "cargo txt" regardless of invocation method
- Tool correctly parses both invocation methods by removing "txt" arg when
  needed
- All documentation updated to use new names
- All tests pass
- No changes to `.zed/agent/**/*.md` files

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

- The "txt" crate name is available on crates.io per user requirement
- Agent files should remain untouched to preserve historical context
- After renaming, the tool will be installable via `cargo install txt` from
  crates.io
