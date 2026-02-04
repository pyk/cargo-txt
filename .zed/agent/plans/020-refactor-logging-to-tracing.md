---
type: normal
title: "Refactor logging to tracing"
seq: 020
slug: "refactor-logging-to-tracing"
created: "2026-02-04T07:15:10Z"
status: completed
---

# Refactor logging to tracing

Replace the `env_logger` and `log` crates with `tracing` and
`tracing-subscriber` for improved logging capabilities. This change enables
better structured logging, more flexible filtering, and consistency with modern
Rust logging practices.

## Current Problems

The current implementation uses `env_logger` with `log` crate macros. The
initialization in `main.rs` converts verbosity flags to environment variables
and passes them to env_logger.

```rust
// Current src/main.rs initialization
let verbosity_level = args.verbosity.log_level_filter().to_string();
let env = env_logger::Env::default().default_filter_or(verbosity_level);
env_logger::Builder::from_env(env).init();
```

All modules import log macros directly:

```rust
// Example from src/cargo.rs
use log::{debug, trace};
```

This approach works but lacks the flexibility and structured logging
capabilities that `tracing` provides. The user wants to match the pattern used
in their `forger` CLI tool.

## Proposed Solution

1. Remove `log` and `env_logger` dependencies from `Cargo.toml`
2. Update `src/main.rs` to use `tracing_subscriber` with `Verbosity<InfoLevel>`
   default
3. Replace all `use log::` imports with `use tracing::` in module files
4. Configure tracing to use simple format (compact, no time, no target) for
   non-trace levels

## Analysis Required

No extensive analysis is required as this is a straightforward mechanical
refactoring. Verify that `clap-verbosity-flag` version 3.0.4 supports the
`InfoLevel` trait and `tracing_level_filter()` method.

### Dependency Investigation

- [ ] Confirm `clap-verbosity-flag` 3.0.4 API is compatible with the forger
      example pattern

### Code Locations to Check

- `src/main.rs` - Current env_logger initialization (lines 39-42)
- `src/cargo.rs` - Log macro usage
- `src/commands/build.rs` - Log macro usage
- `src/commands/list.rs` - Log macro usage
- `src/commands/show.rs` - Log macro usage

## Implementation Checklist

### Code Changes

- [x] Remove `log = "0.4.29"` and `env_logger = "0.11.8"` from `Cargo.toml`
- [x] Update `src/main.rs`:
    - Change `Verbosity` to `Verbosity<InfoLevel>` in the `Args` struct
    - Replace env_logger initialization with tracing_subscriber initialization
    - Use conditional formatting based on `LevelFilter::TRACE` check
- [x] Update `src/cargo.rs`: Replace `use log::{debug, trace};` with
      `use tracing::{debug, trace};`
- [x] Update `src/commands/build.rs`: Replace `use log::{debug, info};` with
      `use tracing::{debug, info};`
- [x] Update `src/commands/list.rs`: Replace `use log::{debug, trace};` with
      `use tracing::{debug, trace};`
- [x] Update `src/commands/show.rs`: Replace `use log::{debug, trace};` with
      `use tracing::{debug, trace};`

## Test Plan

### Verification Tests

- [x] Run `cargo build` and ensure compilation succeeds without warnings
- [x] Run `cargo test` and verify all tests pass
- [x] Run `cargo clippy` and ensure no warnings
- [x] Test with `-v` flag: verify debug-level messages appear
- [x] Test with `-vv` flag: verify info-level messages appear
- [x] Test with `-vvvv` flag: verify trace-level messages appear with detailed
      format
- [x] Test with no verbosity flags: verify default InfoLevel behavior
- [x] Run each command (`build`, `list`, `show`) to verify logging works
      correctly

### Regression Tests

- [x] Run `cargo txt build <crate>` with no flags (default behavior)
- [x] Run `cargo txt list <lib>` with no flags
- [x] Run `cargo txt show <item>` with no flags
- [x] Verify error handling still works and errors are logged appropriately

## Structure After Changes

### Dependencies in Cargo.toml

```toml
# BEFORE
log = "0.4.29"
env_logger = "0.11.8"

# AFTER
(removed)
```

### Main.rs Initialization

```rust
// BEFORE
let verbosity_level = args.verbosity.log_level_filter().to_string();
let env = env_logger::Env::default().default_filter_or(verbosity_level);
env_logger::Builder::from_env(env).init();

// AFTER
if args.verbosity.tracing_level_filter() == LevelFilter::TRACE {
    tracing_subscriber::fmt().with_max_level(args.verbosity).init();
} else {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_max_level(args.verbosity)
        .init();
}
```

### Module Imports

```rust
// BEFORE
use log::{debug, trace};

// AFTER
use tracing::{debug, trace};
```

## Design Considerations

1. **Verbosity Default**: Changed from `Verbosity` to `Verbosity<InfoLevel>` to
   match the forger pattern and use InfoLevel as the default.
    - **Alternative**: Keep default WarnLevel for less verbose output.
    - **Resolution**: Use InfoLevel as specified in requirements, matching the
      forger CLI pattern.

2. **Conditional Formatting**: Use detailed format for TRACE level and compact
   format for all other levels.
    - **Alternative**: Use detailed format for DEBUG and above.
    - **Resolution**: Only use detailed format for TRACE, matching the forger
      example.

3. **Environment Variable Support**: The forger example doesn't show explicit
   RUST_LOG support.
    - **Alternative**: Add env-filter feature to support RUST_LOG environment
      variable.
    - **Resolution**: Keep it simple for now. The tracing-subscriber already
      includes env-filter feature in Cargo.toml, so RUST_LOG will still work.

## Success Criteria

- `cargo build` succeeds without compilation errors or warnings
- All existing tests pass (`cargo test`)
- `rust-lint` passes without warnings
- `cargo clippy` passes without warnings
- Commands produce expected output with different verbosity levels
- TRACE level uses detailed format (with time and target)
- All other levels use compact format (no time, no target)
- Default behavior shows info-level messages

## Implementation Notes

The `tracing-subscriber` dependency in `Cargo.toml` already includes the
`env-filter` feature, so the `RUST_LOG` environment variable will continue to
work even with tracing. The initialization pattern follows the user's forger CLI
example exactly.

The main difference from the current env_logger approach is that tracing uses
structured logging with spans, but for this refactoring, we're only replacing
the macros and initialization. Future enhancements could leverage tracing's more
advanced features like spans and instrumentation.
