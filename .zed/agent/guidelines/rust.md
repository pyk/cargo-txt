# Rust Coding Guidelines Index

> [!IMPORTANT]
>
> Follow these Rust coding guidelines strictly. Read the appropriate module(s)
> based on the task at hand.

## Quick Reference

| Module                     | Purpose                                             | When to Read                               |
| -------------------------- | --------------------------------------------------- | ------------------------------------------ |
| rust-control-flow.md       | Core coding patterns, control flow, function design | When writing or reviewing any Rust code    |
| rust-error-handling-bin.md | anyhow-based error handling for CLI tools           | When working with errors in binaries       |
| rust-error-handling-lib.md | thiserror-based error handling for libraries        | When working with errors in libraries      |
| rust-testing.md            | Test organization and test selection                | When writing or reviewing tests            |
| rust-documentation.md      | Doc comment and API reference style                 | When adding or updating Rust documentation |

## Reading Strategy

For new code implementation:

1. Read `rust-control-flow.md` completely
2. Read specific module(s) for your task:
    - For CLI tools: `rust-error-handling-bin.md` (anyhow)
    - For libraries: `rust-error-handling-lib.md` (thiserror)
3. Reference `rust-documentation.md` when adding docs

For code review:

1. Read all relevant modules based on what was changed
2. Apply principles systematically rather than guessing

For refactoring:

1. Read `rust-control-flow.md` for function design principles
2. Read error handling guide based on target:
    - For CLI tools: `rust-error-handling-bin.md` (anyhow)
    - For libraries: `rust-error-handling-lib.md` (thiserror)
3. Read `rust-testing.md` if reorganizing tests

## File Structure

All guideline files are located in `.zed/agent/guidelines/`:

```
.zed/agent/guidelines/
├── rust.md                       # This file (index)
├── rust-control-flow.md          # Core principles
├── rust-error-handling-bin.md    # Error handling for binaries (anyhow)
├── rust-error-handling-lib.md    # Error handling for libraries (thiserror)
├── rust-testing.md               # Testing guidelines
└── rust-documentation.md         # Documentation style
```
