---
type: normal
title: "Remove println usage"
seq: 022
slug: "remove-println-usage"
created: "2026-02-04T09:20:08Z"
status: completed
---

# Remove println usage

Replace all `println!` usage with `info!` from the `tracing` crate in the build
command to ensure consistent logging behavior across the application.

## Current Problems

The current implementation uses `println!` for some logging messages in the
build command, while using `info!` from the `tracing` crate for others. This
creates inconsistent behavior:

1. Some log messages respect verbosity flags, while others always appear
2. The success messages in `build.rs` use `println!` instead of `info!`
3. Inconsistent logging makes it harder for users to control output verbosity

Example output from `cargo txt build serde`:

```shell
 INFO Running cargo doc --package serde --no-deps
 INFO Converted 61 items to markdown
✓ Built documentation for serde (61 items)         <- println! (not using tracing)
  Run `cargo txt list serde` to see all items      <- println! (not using tracing)
 INFO Successfully saved documentation             <- info! (using tracing)
```

The lines with checkmarks and the hint message use `println!` and are not
controlled by the tracing verbosity level.

## Proposed Solution

Replace all `println!` statements in `src/commands/build.rs` with `info!` macro
from the `tracing` crate. This ensures:

1. Consistent logging behavior across all log messages
2. Proper respect for verbosity flags (`-v`, `-vv`, etc.)
3. All informational messages follow the same logging pattern

**Note**: The `list` and `show` commands intentionally use `println!` to output
their main results (the markdown content) to stdout. These should NOT be changed
as they are the primary output of those commands.

## Analysis Required

Minimal analysis is required. Need to verify:

1. Confirm that the `info!` macro is already imported in `src/commands/build.rs`
2. Verify that replacing the two `println!` statements with `info!` won't break
   existing functionality

### Code Locations to Check

- `src/commands/build.rs` - Lines 349-354 in the `save_doc` function

## Implementation Checklist

### Code Changes

- [x] Replace
      `println!("✓ Built documentation for {} ({} items)", lib_name, item_count)`
      with
      `info!("Built documentation for {} ({} items)", lib_name, item_count)`
      (also removed the ✓ symbol)
- [x] Replace `println!("  Run `cargo txt list {}` to see all items", lib_name)`
      with `info!("Run `cargo txt list {}` to see all items", lib_name)` (also
      removed the leading spaces)
- [x] Verify that `use tracing::{debug, info}` is present in imports (should
      already exist)

## Test Plan

### Verification Tests

- [x] Run `cargo build` and ensure compilation succeeds
- [x] Run `cargo test` and verify all tests pass
- [x] Run `cargo clippy -- -D warnings` and ensure no warnings
- [x] Run `rust-lint` and ensure no warnings
- [x] Test `cargo txt build serde` with default verbosity and verify info
      messages appear
- [x] Test `cargo txt build serde -q` and verify info messages are suppressed
- [x] Test `cargo txt build serde -vv` and verify info messages appear with
      other info-level logs
- [x] Verify the output format remains unchanged when info messages are visible

### Regression Tests

- [x] Run `cargo txt list serde` and verify it still outputs markdown content
      (not affected by changes)
- [x] Run `cargo txt show serde` and verify it still outputs documentation
      content (not affected by changes)
- [x] Verify that the markdown files are still generated correctly in
      `target/docmd/`

## Structure After Changes

### Before

```rust
// src/commands/build.rs, lines 349-357
let item_count = total_files.saturating_sub(3);

println!(
    "✓ Built documentation for {} ({} items)",
    lib_name, item_count
);
println!("  Run `cargo txt list {}` to see all items", lib_name);

info!("Successfully saved documentation");
```

### After

```rust
// src/commands/build.rs, lines 349-357
let item_count = total_files.saturating_sub(3);

info!(
    "Built documentation for {} ({} items)",
    lib_name, item_count
);
info!("Run `cargo txt list {}` to see all items", lib_name);

info!("Successfully saved documentation");
```

## Design Considerations

1. **Preserve User-Facing Messages**: The success messages should remain
   user-friendly and appear at the default verbosity level (InfoLevel).

2. **Consistency with Existing Logging**: The build command already uses `info!`
   for the final "Successfully saved documentation" message. The new changes
   make the preceding messages consistent.

3. **Output Format**: The indentation and checkmark symbols are part of the
   message string and will be preserved.

4. **List and Show Commands**: These commands use `println!` to output their
   primary results (markdown content) and must NOT be changed. They are
   outputting data, not logging.

## Success Criteria

- `rust-lint` passes without warnings
- `cargo clippy -- -D warnings` passes without warnings
- `cargo build` succeeds without compilation errors or warnings
- `cargo test` passes all tests
- All success messages in the build command use `info!` instead of `println!`
- Success messages appear with default verbosity (`-vv` or no flags)
- Success messages are suppressed with `-q` flag
- The output format and content remain unchanged when messages are visible
- List and show commands continue to work correctly (not affected by changes)

## Implementation Notes

This is a straightforward mechanical change. The `tracing::info` macro is
already imported in `src/commands/build.rs`, so no import changes are needed.
The only change required is replacing `println!` with `info!` for the two
success messages in the `save_doc` function.

After the change, all three messages in the function will use consistent logging
with the `info!` macro.
