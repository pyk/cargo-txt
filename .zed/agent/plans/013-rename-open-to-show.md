---
type: normal
title: "Rename open command to show"
seq: 013
slug: "rename-open-to-show"
created: "2025-01-09T12:00:00Z"
status: in_progress
---

# Rename open command to show

The `open` command name feels awkward when using the tool. Users expect to
"show" documentation rather than "open" it. This plan renames the command from
`open` to `show` throughout the codebase for better semantic clarity and more
intuitive usage.

## Current Problems

The command name `open` is semantically awkward for a tool that displays
documentation to stdout:

```rust
// Current CLI usage
Command::Open { item_path } // Doesn't match the action of showing content

// Current user-facing command
cargo txt open serde // "Open" suggests opening a file or editor, not displaying text
```

The naming inconsistency becomes apparent when users interact with the tool, as
`open` typically implies opening in an external application rather than
displaying content directly.

## Proposed Solution

1. Rename the CLI subcommand from `open` to `show`
2. Rename the error type from `OpenError` to `ShowError`
3. Rename the module file from `open.rs` to `show.rs`
4. Update all documentation to use `show` instead of `open`
5. Preserve all existing functionality and behavior

## Analysis Required

### Code Locations to Check

- `src/commands/open.rs` - Main implementation file (will be renamed)
- `src/main.rs` - CLI definition and command matching
- `src/commands/mod.rs` - Module exports
- `src/error.rs` - Error type definitions
- `README.md` - User documentation
- `DOCS.md` - Technical documentation

## Implementation Checklist

### Code Changes

- [x] Rename `src/commands/open.rs` to `src/commands/show.rs`
- [x] Update `src/commands/mod.rs`: Change `pub mod open` to `pub mod show`
- [x] Update `src/commands/mod.rs`: Change `pub use open::open` to
      `pub use show::show`
- [x] Update `src/main.rs`: Change `Command::Open` variant to `Command::Show`
- [x] Update `src/main.rs`: Change command description from "Open and display"
      to "Show and display"
- [x] Update `src/main.rs`: Change match arm from `Command::Open { item_path }`
      to `Command::Show { item_path }`
- [x] Update `src/main.rs`: Update function call from `open(item_path)` to
      `show(item_path)`
- [x] Update `src/main.rs`: Update import from `commands::{build, open}` to
      `commands::{build, show}`
- [x] Update `src/error.rs`: Change `Open(OpenError)` variant to
      `Show(ShowError)` in Error enum
- [x] Update `src/error.rs`: Rename `OpenError` enum to `ShowError`
- [x] Update `src/error.rs`: Update `From<OpenError>` implementation to
      `From<ShowError>`
- [x] Update `src/commands/show.rs`: Change function name from `pub fn open` to
      `pub fn show`
- [x] Update `src/commands/show.rs`: Update module doc comment if it mentions
      "open"
- [x] Update `src/commands/show.rs`: Update debug log message from "Open
      command" to "Show command"
- [x] Update `src/commands/show.rs`: Update error references from
      `error::OpenError` to `error::ShowError`
- [x] Verify tests in `src/commands/show.rs` still compile (they test logic, not
      command name)

### Documentation Updates

- [x] Update `README.md`: Change all occurrences of `cargo txt open` to
      `cargo txt show`
- [x] Update `README.md`: Change "Open Command" section header to "Show Command"
- [x] Update `README.md`: Change "### Open Command" to "### Show Command"
- [x] Update `README.md`: Update instruction block from
      `cargo txt open <crate-item>` to `cargo txt show <crate-item>`
- [x] Update `README.md`: Change all code examples from `open serde` to
      `show serde`
- [x] Update `README.md`: Update Current Status section from "Open command" to
      "Show command"
- [x] Update `DOCS.md`: Change "### open" section header to "### show"
- [x] Update `DOCS.md`: Change code examples from `cargo txt open` to
      `cargo txt show`
- [x] Update `DOCS.md`: Update all references to "open command" to "show
      command"
- [x] Update `DOCS.md`: Update "Auto-Build:" section references if they mention
      open

### Test Updates

- [x] Verify existing tests in `src/commands/show.rs` still pass (no test name
      changes needed)
- [x] Run `cargo test` to verify all tests pass

## Test Plan

### Verification Tests

- [x] Run `cargo build` to verify compilation succeeds
- [x] Run `cargo txt show serde` to verify the new command works
- [x] Run `cargo txt show serde::Error` to verify item-specific display works
- [x] Run `cargo txt --help` to verify help text shows "show" command
- [x] Run `cargo txt show --help` to verify show-specific help text displays
      correctly

### Regression Tests

- [x] Verify auto-build still triggers when documentation doesn't exist
- [x] Verify error messages still display correctly with new error type names
- [x] Verify `cargo txt build` still works independently

## Structure After Changes

### File Structure

```
src/
├── commands/
│   ├── build.rs       # Unchanged
│   ├── show.rs        # Renamed from open.rs
│   └── mod.rs         # Updated exports
├── error.rs           # Updated error types
├── main.rs            # Updated CLI definition
└── ...
```

### CLI Usage After Changes

```rust
// BEFORE
cargo txt open serde
cargo txt open serde::Error

// AFTER
cargo txt show serde
cargo txt show serde::Error
```

### Error Types After Changes

```rust
// BEFORE
pub enum Error {
    Build(BuildError),
    Open(OpenError),
    // ...
}

pub enum OpenError {
    InvalidItemPath { item_path: String },
    // ...
}

// AFTER
pub enum Error {
    Build(BuildError),
    Show(ShowError),
    // ...
}

pub enum ShowError {
    InvalidItemPath { item_path: String },
    // ...
}
```

## Design Considerations

1. **Command name choice**: "show" is more semantically accurate for displaying
   documentation to stdout than "open" which suggests opening in an external
   application.
2. **Backward compatibility**: This is a breaking change. Users will need to
   update their scripts and workflows to use `show` instead of `open`.
3. **Alternative considered**: `display` was also considered but rejected
   because it's longer and less common as a CLI subcommand.
4. **Consistency**: The new name aligns better with common CLI patterns (e.g.,
   `git show`, `docker show`).

## Success Criteria

- All code compiles without errors
- `cargo txt show serde` successfully displays documentation
- Help text shows "show" command with appropriate description
- Error messages use "ShowError" consistently
- All tests pass
- README.md and DOCS.md reference only `show`, never `open`

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

Successfully completed all code changes and documentation updates:

**Code Changes:**

- Renamed `open.rs` to `show.rs` and updated all references
- Updated `main.rs` to use `Command::Show` instead of `Command::Open`
- Renamed `OpenError` to `ShowError` in `error.rs` with all associated
  implementations
- Updated all doc comments and debug log messages in `show.rs`

**Documentation Updates:**

- Updated `README.md` and `DOCS.md` to use `show` command instead of `open`
- Added "Why I'm building this" narrative section to README.md explaining pain
  points and motivation
- Restructured README.md Features section to be feature-focused while addressing
  user problems
- Updated README.md warning to explicitly state "expect breaking changes"
- Updated README.md argument descriptions from "open" to "show"
- Set logo image height to 64 and width to auto

**Verification:**

- All tests pass (43 tests)
- Verified auto-build triggers when documentation doesn't exist
- Verified item-specific display works (e.g.,
  `cargo txt show serde::de::value::Error`)
- Confirmed old `open` command is no longer available
- Help text correctly shows "show" command
- Code passes clippy checks

Track any issues or special considerations during implementation.
