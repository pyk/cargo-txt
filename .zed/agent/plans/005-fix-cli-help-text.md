---
type: normal
title: "Fix CLI Help Text Issues"
seq: 005
slug: "fix-cli-help-text"
created: "2026-01-07T08:39:10Z"
status: completed
---

# Fix CLI Help Text Issues

Fix multiple issues with the CLI help text display and argument structure,
including incorrect binary name display, unnecessary global flags, description
formatting, and incorrect argument types for the crate parameter.

## Current Problems

The current CLI help text has several issues that need to be fixed:

**Problem 1: Incorrect binary name in usage**

```
Usage: cargo-docmd [OPTIONS] <COMMAND>
```

Expected:

```
Usage: cargo docmd [OPTIONS] <COMMAND>
```

The tool is invoked as a cargo subcommand using a space, not a hyphen.

**Problem 2: Unnecessary global flags**

Current help shows:

```
Options:
  -v, --verbose...       Increase verbosity of output
  -c, --config <CONFIG>  Path to configuration file
```

These flags are not implemented and should be removed:

- `-v, --verbose` - Not implemented yet
- `-c, --config` - Not implemented yet

**Problem 3: Crate argument should be positional, not a flag**

Current:

```
Usage: cargo-docmd build --crate <CRATE>

Options:
  -c, --crate <CRATE>    Crate name to build documentation for
```

Expected:

```
Usage: cargo docmd build <CRATE>

Arguments:
  <CRATE>    Crate name to build documentation for
```

## Proposed Solution

1. Change the clap `name` attribute from `cargo-docmd` to `cargo docmd`
2. Remove the global `verbose` and `config` fields from the Args struct
3. Change `--crate` flag to positional argument in build command
4. Change `--crate` flag to positional argument in browse command for
   consistency
5. Simplify build command description to a single line for brevity
6. Update all documentation (README.md and DOCS.md) to reflect these changes

## Analysis Required

### Dependency Investigation

- [x] Review clap documentation for positional arguments

### Code Locations to Check

- [x] `src/main.rs` - CLI structure with clap derives
- [x] `src/commands/build.rs` - Build command handler
- [x] `src/commands/browse.rs` - Browse command handler
- [x] `README.md` - Usage examples and documentation
- [x] `DOCS.md` - CLI reference documentation

## Implementation Checklist

### Code Changes

- [x] Update `src/main.rs`: Change `#[command(name = "cargo-docmd")]` to
      `#[command(name = "cargo docmd")]`
- [x] Update `src/main.rs`: Remove `verbose` field from Args struct
- [x] Update `src/main.rs`: Remove `config` field from Args struct
- [x] Update `src/main.rs`: Remove clap attributes for verbose and config
- [x] Update `src/main.rs`: Change Build subcommand `--crate` flag to positional
      argument:
    ```rust
    Build {
        /// Crate name to build documentation for
        #[arg(value_name = "CRATE")]
        crate_name: String,
    }
    ```
- [x] Update `src/main.rs`: Change Browse subcommand `--crate` flag to
      positional argument:

    ```rust
    Browse {
        /// Crate name to browse
        #[arg(value_name = "CRATE")]
        crate_name: String,

        /// Optional specific item to display
        #[arg(short, long, value_name = "ITEM")]
        item: Option<String>,
    }
    ```

- [x] Update `src/main.rs`: Simplify Build command description to a single line
      for better readability:
    ```rust
    /// Generate markdown documentation from rustdoc HTML for coding agents.
    ```
- [x] Verify build and browse command handlers still receive correct arguments

### Documentation Updates

- [x] Update `README.md`: Change all examples from `--crate <CRATE>` to
      positional `<CRATE>`
- [x] Update `README.md`: Remove references to `-v, --verbose` and
      `-c, --config` flags
- [x] Update `README.md`: Update "Quick Start" section examples
- [x] Update `README.md`: Update "Usage" section examples
- [x] Update `DOCS.md`: Remove global options section for verbose and config
- [x] Update `DOCS.md`: Change build command examples to use positional crate
      argument
- [x] Update `DOCS.md`: Change browse command examples to use positional crate
      argument
- [x] Update `DOCS.md`: Remove all references to `-v, --verbose` in examples
- [x] Update `DOCS.md`: Remove all references to `-c, --config` in examples

### Test Updates

- [x] Run `cargo run -- --help` to verify top-level help shows correct binary
      name
- [x] Run `cargo run -- build --help` to verify build command help shows
      positional crate argument
- [x] Run `cargo run -- browse --help` to verify browse command help shows
      positional crate argument
- [x] Run `cargo run -- build serde` to verify positional argument works
- [x] Run `cargo run -- browse serde` to verify positional argument works
- [x] Run `cargo run -- browse serde --item Serialize` to verify item flag still
      works with positional crate
- [x] Verify no compiler warnings with `cargo clippy`

## Test Plan

### Verification Tests

- [x] Verify `cargo run -- --help` displays:
      `     Usage: cargo docmd [OPTIONS] <COMMAND>     ` (with space, not
      hyphen)
- [x] Verify `cargo run -- --help` does NOT show `-v, --verbose` option
- [x] Verify `cargo run -- --help` does NOT show `-c, --config` option
- [x] Verify `cargo run -- build --help` displays:
      `     Usage: cargo docmd build <CRATE>     `
- [x] Verify `cargo run -- build --help` shows crate as "Arguments:" not
      "Options:"
- [x] Verify `cargo run -- browse --help` shows crate as positional argument
- [x] Verify `cargo run -- build serde` works (positional crate)
- [x] Verify `cargo run -- browse serde` works (positional crate)
- [x] Verify `cargo run -- browse serde --item Serialize` works (positional
      crate + optional item flag)

### Regression Tests

- [x] Ensure cargo build completes without errors
- [x] Ensure cargo test passes all tests
- [x] Ensure cargo clippy produces no warnings
- [x] Ensure existing functionality (build command) still works correctly

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   └── main.rs          # Updated CLI structure
├── README.md            # Updated documentation
├── DOCS.md              # Updated CLI reference
└── .zed/agent/plans/
    └── 005-fix-cli-help-text.md
```

### CLI Structure (src/main.rs)

```rust
/// A cargo doc for coding agents
#[derive(Parser)]
#[command(name = "cargo docmd")]  // Changed from "cargo-docmd"
#[command(version = "0.1.0")]
#[command(about = "A cargo doc for coding agents", long_about = None)]
struct Args {
    // verbose field removed
    // config field removed

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build markdown documentation from rustdoc HTML
    ///
    /// Generates rustdoc HTML using cargo doc and parses it to create
    /// markdown files suitable for coding agents. Output is placed in
    /// `$CARGO_TARGET_DIR/docmd`.
    ///
    /// (Optional: Can be simplified to a single line for brevity)
    Build {
        /// Crate name to build documentation for
        #[arg(value_name = "CRATE")]
        crate_name: String,
    },

    /// Browse crate documentation
    ///
    /// Displays crate documentation in a terminal-friendly format.
    /// Optionally, you can specify a specific item to display only that
    /// documentation.
    Browse {
        /// Crate name to browse
        #[arg(value_name = "CRATE")]
        crate_name: String,

        /// Optional specific item to display
        #[arg(short, long, value_name = "ITEM")]
        item: Option<String>,
    },
}
```

## Expected Help Text After Changes

### Main Help

```
A cargo doc for coding agents

Usage: cargo docmd [OPTIONS] <COMMAND>

Commands:
  build   Build markdown documentation from rustdoc HTML
  browse  Browse crate documentation
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

### Build Subcommand Help

```
Build markdown documentation from rustdoc HTML

Usage: cargo docmd build <CRATE>

Arguments:
  <CRATE>  Crate name to build documentation for

Options:
  -h, --help  Print help (see a summary with '-h')
```

(Optional simplified version):

```
Generate markdown documentation from rustdoc HTML for coding agents.

Usage: cargo docmd build <CRATE>

Arguments:
  <CRATE>  Crate name to build documentation for

Options:
  -h, --help  Print help (see a summary with '-h')
```

### Browse Subcommand Help

```
Browse crate documentation

Usage: cargo docmd browse <CRATE>

Arguments:
  <CRATE>  Crate name to browse

Options:
  -i, --item <ITEM>  Optional specific item to display
  -h, --help         Print help (see a summary with '-h')
```

## Design Considerations

1. **Positional vs Flag Arguments for Crate**:
    - **Alternative**: Keep `--crate` flag for clarity
    - **Resolution**: Use positional argument for better UX and consistency
    - **Rationale**: The crate name is always required, making it a better fit
      as a positional argument. This follows common CLI patterns (e.g.,
      `cargo build <package>` vs `cargo build --package <package>`)

2. **Browse Command Consistency**:
    - **Alternative**: Only change build command to positional
    - **Resolution**: Change both build and browse to positional for consistency
    - **Rationale**: Maintaining consistency across subcommands provides better
      user experience and reduces confusion

3. **Binary Name Format**:
    - **Alternative**: Keep "cargo-docmd" to match binary filename
    - **Resolution**: Use "cargo docmd" to match actual invocation
    - **Rationale**: Users invoke the command as `cargo docmd`, not
      `cargo-docmd`. The help text should reflect actual usage

4. **Removing Unimplemented Flags**:
    - **Alternative**: Keep flags with "not yet implemented" notes
    - **Resolution**: Remove flags entirely until implemented
    - **Rationale**: Showing options that don't work confuses users. Re-add when
      functionality is implemented

## Success Criteria

- [x] Running `cargo run -- --help` shows "Usage: cargo docmd" (with space)
- [x] Running `cargo run -- --help` shows no `-v, --verbose` option
- [x] Running `cargo run -- --help` shows no `-c, --config` option
- [x] Running `cargo run -- build --help` shows `<CRATE>` as positional argument
- [x] Running `cargo run -- browse --help` shows `<CRATE>` as positional
      argument
- [x] `cargo run -- build serde` works with positional argument
- [x] `cargo run -- browse serde` works with positional argument
- [x] `cargo run -- browse serde --item Serialize` works with positional crate
- [x] All existing tests pass (`cargo test`)
- [x] No compiler warnings (`cargo clippy`)
- [x] README.md examples updated to use positional crate argument
- [x] DOCS.md CLI reference updated to match new CLI structure

## Implementation Status: ✅ COMPLETED

## Implementation Notes

- The positional argument change will require updating all documentation
  examples
- Remove the conflict between `--config` (using `-c`) and `--crate` (also using
  `-c`) by removing `--config` entirely
- Consider whether browse command's optional `--item` should also be positional
  or remain as a flag (currently planning to keep it as a flag since it's
  optional)
- Test the actual invocation: when installed via `cargo install`, the binary
  will be named `cargo-docmd` but users invoke it as `cargo docmd`. The clap
  name attribute should reflect the user-facing invocation
