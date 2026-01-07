---
type: normal
title: "CLI Skeleton Implementation"
seq: 001
slug: "cli-overview"
created: "2026-01-07T04:33:00Z"
status: completed
---

# CLI Skeleton Implementation

Implement the foundational command-line interface structure for `cargo-docmd`,
including help text, subcommands, and descriptions. This establishes the
user-facing API and sets the architecture for all future CLI functionality.

## Current Problems

The project currently has no CLI structure, making it impossible for users to
interact with the tool through the command line.

```rust
// Current state in src/main.rs
fn main() {
    println!("Hello, world!");
}
```

Issues:

- No argument parsing or CLI library integration
- No help text or usage information
- No subcommand structure
- No way to specify input files or output directories
- No validation of command-line inputs

## Proposed Solution

1. Integrate `clap` crate for CLI argument parsing and help text generation
2. Define the main subcommand structure with `generate` and `browse` commands
3. Implement comprehensive help text at all levels (main, subcommands,
   arguments)
4. Set up the basic CLI flow with command modules in dedicated directory

## Analysis Required

### Dependency Investigation

- [ ] Check `clap` dependency in `Cargo.toml`

### Code Locations to Check

- `Cargo.toml` - Add `clap` dependency with appropriate features
- `src/main.rs` - Replace current placeholder with CLI structure

## Implementation Checklist

### Code Changes

- [x] Add `clap` dependency to `Cargo.toml` with `derive` feature enabled
- [x] Create `src/commands/mod.rs` for command module exports
- [x] Create `src/commands/generate.rs` for generate command implementation
- [x] Create `src/commands/browse.rs` for browse command implementation
- [x] Define top-level CLI struct in `src/main.rs` using `clap::Parser` derive
      macro
- [x] Implement main `Command` enum with subcommands:
    - `generate` - Generate markdown from rustdoc JSON
    - `browse` - Browse crate documentation interactively
- [x] Add argument fields to each subcommand:
    - `generate`: `crate` (crate name), `output` (optional output directory)
    - `browse`: `crate` (crate name), `item` (optional specific item)
- [x] Add top-level options:
    - `--verbose` / `-v` for verbose output
    - `--config` path to optional config file
- [x] Update `src/main.rs` to parse CLI arguments and dispatch to appropriate
      handler
- [x] Create placeholder handler functions in each command module that print
      "Not yet implemented"

### Documentation Updates

- [x] Update `README.md` with new CLI usage examples
- [x] Create `DOCS.md` with CLI reference section documenting all subcommands
      and options
- [x] Document the CLI architecture in `src/main.rs` with module-level
      documentation

### Test Updates

- [x] Add CLI parsing tests in `src/main.rs` using `clap`s testing utilities
      (removed per user request)
- [x] Test that invalid arguments produce appropriate error messages

## Test Plan

### Verification Tests

- [x] Verify `cargo-docmd --help` displays top-level help with all subcommands
      listed
- [x] Verify `cargo-docmd generate --help` shows generate subcommand usage
- [x] Verify `cargo-docmd browse --help` shows browse subcommand usage
- [x] Verify `--verbose` flag is parsed correctly
- [x] Verify missing required arguments produce helpful error messages
- [x] Verify unknown subcommands are rejected with clear error message

### Regression Tests

- [x] Ensure `cargo build` completes without errors
- [x] Ensure `cargo clippy` produces no warnings
- [x] Verify the binary can be executed from the target directory

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── main.rs            # CLI structure definition and command dispatch
│   └── commands/
│       ├── mod.rs         # Command module exports
│       ├── generate.rs    # Generate command implementation
│       └── browse.rs      # Browse command implementation
└── 2025-01-06-cli-overview.md
```

### Module Exports

```rust
// src/main.rs
mod commands;

use clap::Parser;
use commands::{generate, browse};

#[derive(Parser)]
#[command(name = "cargo-docmd")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    /// Increase verbosity of output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Path to configuration file
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate {
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<std::path::PathBuf>,
    },
    Browse {
        #[arg(short, long = "crate", value_name = "CRATE")]
        crate_name: String,
        #[arg(short, long, value_name = "ITEM")]
        item: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Generate { crate_name, output } => {
            generate(
                crate_name,
                output.unwrap_or_else(|| std::path::PathBuf::from("docs")),
            );
        }
        Command::Browse { crate_name, item } => {
            browse(crate_name, item);
        }
    }
}
```

```rust
// src/commands/mod.rs
pub mod generate;
pub mod browse;

pub use generate::generate;
pub use browse::browse;
```

```rust
// src/commands/generate.rs
use std::path::PathBuf;

pub fn generate(crate_name: String, output: PathBuf) {
    println!("Generate command: crate={}, output={:?}", crate_name, output);
    println!("Not yet implemented");
}
```

```rust
// src/commands/browse.rs
pub fn browse(crate_name: String, item: Option<String>) {
    println!("Browse command: crate={}, item={:?}", crate_name, item);
    println!("Not yet implemented");
}
```

## Expected Help Text

### Main Help

```
A cargo doc for coding agents

Usage: cargo-docmd [OPTIONS] <COMMAND>

Commands:
  generate  Generate markdown documentation from rustdoc JSON
  browse    Browse crate documentation interactively
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Increase verbosity of output

          Use multiple times for more verbose output (e.g., -vv, -vvv).

  -c, --config <CONFIG>
          Path to configuration file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Generate Subcommand Help

```
Generate markdown documentation from rustdoc JSON

Converts rustdoc JSON output into markdown files suitable for coding agents.
The generated documentation is optimized for API browsing and understanding.

Usage: cargo-docmd generate [OPTIONS] --crate <CRATE>

Options:
  -c, --crate <CRATE>
          Crate name to generate documentation for

  -o, --output <OUTPUT>
          Output directory for generated markdown

  -h, --help
          Print help (see a summary with '-h')
```

### Browse Subcommand Help

```
Browse crate documentation interactively

Displays crate documentation in a terminal-friendly format. Optionally,
you can specify a specific item to display only that documentation.

Usage: cargo-docmd browse [OPTIONS] --crate <CRATE>

Options:
  -c, --crate <CRATE>
          Crate name to browse

  -i, --item <ITEM>
          Optional specific item to display

  -h, --help
          Print help (see a summary with '-h')
```

## Design Considerations

1. **Derive vs Builder API for clap**:
    - **Alternative**: Use clap's builder API for programmatic construction
    - **Resolution**: Use derive API for cleaner, more maintainable code. Derive
      macros provide compile-time validation and reduce boilerplate.

2. **Subcommand Selection**:
    - **Alternative**: Start with fewer commands and add incrementally
    - **Resolution**: Define both primary commands now to establish the full CLI
      interface, even if implementation is incomplete. This prevents breaking
      changes later.

3. **Generate Command Input**:
    - **Alternative**: Take JSON file path as input
    - **Resolution**: Take crate name as input. The tool will internally locate
      or generate the rustdoc JSON file for the specified crate.

4. **Error Handling Strategy**:
    - **Alternative**: Custom error types for each CLI error case
    - **Resolution**: Let clap handle argument parsing errors initially.
      Implement custom error handling in handler functions in future iterations.

5. **Configuration Management**:
    - **Alternative**: No config file support initially
    - **Resolution**: Include `--config` option in CLI skeleton to establish
      pattern early, even if config parsing is deferred.

6. **Command Module Organization**:
    - **Alternative**: Keep all CLI logic in main.rs
    - **Resolution**: Separate command handlers into dedicated `src/commands/`
      directory for better organization and future extensibility. CLI structure
      remains in main.rs due to simplicity.

## Success Criteria

- [x] Running `cargo-docmd --help` displays comprehensive help text matching the
      expected output above
- [x] Running `cargo-docmd generate --help` shows detailed usage for the
      generate subcommand with all arguments documented
- [x] Running `cargo-docmd browse --help` shows detailed usage for the browse
      subcommand with all arguments documented
- [x] Running `cargo-docmd` without arguments produces helpful error suggesting
      `--help`
- [x] Both subcommands (`generate`, `browse`) are recognized and dispatch to
      their respective handler functions in `src/commands/`
- [x] `cargo build` completes without errors
- [x] No `cargo clippy` warnings
- [x] Binary can be built and executed successfully

## Implementation Status: ✅ COMPLETED

## Implementation Notes

- **Completed**: The foundational CLI structure is now in place with all
  placeholder handlers implemented.
- **Modifications from plan**:
    - `--crate-name` simplified to `--crate` for cleaner usage
    - `--output` flag made optional with default value of `./docs`
    - Documentation uses `cargo docmd` (subcommand) instead of `cargo-docmd`
      (binary)
    - Removed verbose and config file from Features section to focus on core
      functionality
    - Tests removed per user request as they were deemed unnecessary
- **Future implementations** will fill in the handler functions in
  `src/commands/` with actual logic.
- The CLI structure should be relatively stable after this implementation, but
  minor adjustments may be needed as features are implemented.
- Consider adding shell completion support in a future iteration once CLI
  structure stabilizes.
- The `generate` command takes crate name as input, allowing the tool to handle
  rustdoc JSON location internally based on standard cargo conventions.
