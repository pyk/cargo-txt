# Rust Coding Guidelines Index

CRITICAL: You must follow the strict Rust coding guidelines below. Read the
appropriate module(s) based on the task at hand.

## Quick Reference

| Module                             | Purpose                                             | When to Read                               |
| ---------------------------------- | --------------------------------------------------- | ------------------------------------------ |
| **writing-rust-control-flow.md**   | Core coding patterns, control flow, function design | When writing or reviewing any Rust code    |
| **writing-rust-error-handling.md** | Centralized error handling architecture             | When working with errors or `src/error.rs` |
| **writing-rust-testing.md**        | Test organization and test selection                | When writing or reviewing tests            |
| **writing-rust-documentation.md**  | Doc comment and API reference style                 | When adding or updating documentation      |

## Module Overview

### 1. writing-rust-control-flow.md

**Core Principles Covered:**

- Linear control flow (guard clauses over nesting)
- Data extraction vs. validation ("Peel the Onion")
- Data flow optimization (move over clone)
- Fail-fast parsing
- Combinators over explicit matching
- General writing principles (concise, direct language)
- Descriptive variable naming
- Self-contained functions with clear data flow
- Module-prefixed function calls

**When to read:** Always. This contains the foundational principles that apply
to nearly all Rust code you write or review.

### 2. writing-rust-error-handling.md

**Topics Covered:**

- Centralized error module structure in `src/error.rs`
- Error hierarchy (top-level, category-level, low-level)
- Error conversion and propagation via `From` traits
- Context helper functions
- Main function error pattern
- Debug trait hack for user-friendly error output

**When to read:** When working with error types, implementing error handling, or
modifying `src/error.rs`. This module is self-contained and comprehensive.

### 3. writing-rust-testing.md

**Topics Covered:**

- Test organization (single `mod tests` per file)
- Test grouping by behavior with naming prefixes
- Avoiding unnecessary tests (trait implementation, method existence)

**When to read:** When writing tests, organizing test code, or reviewing test
quality.

### 4. writing-rust-documentation.md

**Topics Covered:**

- Doc comments as API references (what/why, not how)
- Top-level module documentation with `//!`
- Avoiding bullet points and unnecessary headers
- Concise paragraph-style documentation
- Simple, direct English avoiding complex words

**When to read:** When adding or updating doc comments (`///`), module
documentation (`//!`), or API references.

## Cross-Module Principles

These principles apply across all modules:

- **Be concise and direct**: Avoid filler words, marketing language, and
  unnecessary complexity
- **Use simple English**: Favor plain language over jargon or academic phrasing
- **Focus on practical outcomes**: Explain what users can do, not how impressive
  something is
- **Self-documenting code**: Use descriptive names and clear structure to reduce
  the need for comments
- **Fail fast**: Return errors immediately rather than swallowing them
- **Linear flow**: Keep code readable with guard clauses and minimal nesting

## Reading Strategy

**For new code implementation:**

1. Read `writing-rust-control-flow.md` completely
2. Read specific module(s) for your task (e.g., error handling, testing)
3. Reference `writing-rust-documentation.md` when adding docs

**For code review:**

1. Read all relevant modules based on what was changed
2. Apply principles systematically rather than guessing

**For refactoring:**

1. Read `writing-rust-control-flow.md` for function design principles
2. Read `writing-rust-error-handling.md` if changing error types
3. Read `writing-rust-testing.md` if reorganizing tests

## File Structure

All guideline files are located in `.zed/agent/guidelines/`:

```
.zed/agent/guidelines/
├── writing-rust.md                    # This file (index)
├── writing-rust-control-flow.md       # Core principles
├── writing-rust-error-handling.md     # Error handling architecture
├── writing-rust-testing.md            # Testing guidelines
└── writing-rust-documentation.md      # Documentation style
```
