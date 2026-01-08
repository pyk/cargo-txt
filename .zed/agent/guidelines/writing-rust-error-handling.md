# Rust Coding Guidelines: Error Handling

CRITICAL: All error handling must follow these strict guidelines for centralized
error management in `src/error.rs`.

## Overview

Error handling must be centralized in a single `src/error.rs` module with a
clear error hierarchy. This provides consistent error types, user-friendly
messages, and proper error chain propagation.

## Error Module Structure

Define a top-level `Error` enum that wraps specific error categories. Each
category has its own enum with specific variants. All error types implement
`std::error::Error` with proper `Display` and `source()` implementations.

```rust
// src/error.rs

/// Result type alias for convenience.
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level error type for the application.
#[derive(Debug)]
pub enum Error {
    /// Errors that occur during the build process
    Build(BuildError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Build(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
```

## Error Hierarchy Design

Create a clear separation of concerns with three levels:

1. **Top-level errors**: Application-wide errors that wrap all specific error
   types
2. **Category-level errors**: Errors for specific operations (e.g., BuildError,
   NetworkError)
3. **Low-level errors**: Detailed errors for specific operations (e.g.,
   HtmlExtractError)

Low-level errors should not contain context information like file paths. Add
context at the category level when wrapping low-level errors.

```rust
/// Low-level HTML parsing errors (no file paths).
#[derive(Debug)]
pub enum HtmlExtractError {
    SelectorParseFailed { selector: String, error: String },
    ElementNotFound { selector: String },
}

impl fmt::Display for HtmlExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlExtractError::SelectorParseFailed { selector, error } => {
                write!(f, "Failed to parse selector '{}': {}", selector, error)
            }
            HtmlExtractError::ElementNotFound { selector } => {
                write!(f, "Element not found with selector '{}'", selector)
            }
        }
    }
}

impl std::error::Error for HtmlExtractError {}

/// Build process errors (adds file path context).
#[derive(Debug)]
pub enum BuildError {
    HtmlParseFailed {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    MarkdownWriteFailed { path: PathBuf, error: String },
    // ... other variants
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::HtmlParseFailed { path, source } => {
                write!(f, "Failed to parse HTML file '{}': {}", path.display(), source)
            }
            BuildError::MarkdownWriteFailed { path, error } => {
                write!(f, "Failed to write markdown file '{}': {}", path.display(), error)
            }
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildError::HtmlParseFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}
```

## Error Conversion and Propagation

Implement `From` traits to enable automatic error conversion through the `?`
operator. The conversion chain should flow from low-level to high-level errors.

```rust
// Low-level to category-level
impl From<HtmlExtractError> for BuildError {
    fn from(err: HtmlExtractError) -> Self {
        BuildError::HtmlParseFailed {
            path: PathBuf::from("<unknown>"),
            source: Box::new(err),
        }
    }
}

// Category-level to top-level
impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::Build(err)
    }
}

// Low-level directly to top-level (for convenience)
impl From<HtmlExtractError> for Error {
    fn from(err: HtmlExtractError) -> Self {
        Error::Build(err.into())
    }
}
```

## Helper Functions for Context

Provide helper functions to add context when wrapping errors. This keeps error
handling clean while providing useful debugging information.

```rust
/// Wrap a result with a path error context.
pub fn wrap_with_path<T, E>(result: std::result::Result<T, E>, path: &Path) -> Result<T>
where
    E: std::error::Error + Send + Sync + 'static,
{
    result.map_err(|error| {
        BuildError::HtmlParseFailed {
            path: path.to_path_buf(),
            source: Box::new(error),
        }
        .into()
    })
}

// Usage
let type_alias = error::wrap_with_path(
    items::type_alias::TypeAlias::from_str(&html_content),
    &path,
)?;
```

## Main Function Pattern

For binary applications, use `Result<()>` as the return type for `main()`.
Rust's standard library automatically displays errors via the `Display` trait
and shows the full error chain via `source()`. The program exits with status
code 1 on error.

```rust
fn main() -> error::Result<()> {
    let crate_name = std::env::args().nth(1)
        .ok_or_else(|| error::Error::Build(BuildError::MissingCrateName))?;

    cargo::rustdoc(&crate_name)?;

    Ok(())
}
```

## Debug Trait Hack for User-Friendly Error Output

By default, when `main()` returns an error, Rust's standard library prints the
error using the `Debug` format, which produces ugly output like:

```
Error: Build(InvalidCrateName { requested: "foo", available: ["bar", "baz"] })
```

To display user-friendly error messages instead of Debug output, implement a
custom `Debug` trait that delegates to the `Display` implementation. This allows
`main()` to remain simple while forcing the standard library to print your
custom message.

```rust
// Remove #[derive(Debug)] from error types
pub enum BuildError {
    // ... variants
}

// Implement Display for human-readable message
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildError::InvalidCrateName { requested, available } => {
                write!(
                    f,
                    "Crate '{}' is not installed.\n\nAvailable: {}",
                    requested,
                    available.join(", ")
                )
            }
            // ... other variants
        }
    }
}

// **THE TRICK**: Override Debug to delegate to Display
impl fmt::Debug for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
```

Apply this pattern to all error types in the hierarchy (`Error`, `BuildError`,
`HtmlExtractError`, etc.). The result is clean, user-friendly output:

```
Error: Crate 'foo' is not installed.

Available: bar, baz
```

**Benefits:**

- Clean `fn main() -> Result<(), Error>` signature
- User-friendly error messages without manual error printing
- No ugly Debug output with struct field names
- Consistent error display across all error types

## Error Usage Guidelines

- Use `error::Result<T>` (not `std::result::Result<T, Error>`) throughout the
  application
- Propagate errors using the `?` operator, relying on `From` implementations
- Add context at appropriate boundaries (e.g., file paths when parsing files)
- Implement `source()` only when wrapping another error
- Provide user-friendly error messages via `Display`, focusing on what went
  wrong and where
- Avoid panics in production code; use `Result` instead
- For binary applications, return `Result<()>` from `main()`

## Examples

🛑 Bad (Scattered Error Types):

```rust
// In src/html.rs
#[derive(Debug)]
pub enum HtmlError {
    ParseFailed(String),
}

// In src/markdown.rs
#[derive(Debug)]
pub enum MarkdownError {
    WriteFailed(String),
}

// In src/main.rs
fn process() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Multiple error types mixed together
}
```

✅ **Good (Centralized Error Handling):**

```rust
// All errors in src/error.rs
#[derive(Debug)]
pub enum Error {
    Build(BuildError),
}

#[derive(Debug)]
pub enum BuildError {
    HtmlParseFailed { path: PathBuf, source: Box<dyn std::error::Error + Send + Sync> },
    MarkdownWriteFailed { path: PathBuf, error: String },
}

// Consistent Result type everywhere
fn parse_html(path: &Path) -> error::Result<Document> {
    let content = std::fs::read_to_string(path)?;
    // Error propagates as error::Error via From trait
}

fn main() -> error::Result<()> {
    let document = parse_html(&path)?;
    // Automatic error display and exit on error
}
```
