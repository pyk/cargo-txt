# Rust Coding Guidelines

CRITICAL: You must follow the strict Rust coding guidelines below.

## 1. Favor linear control flow (guard clauses) over nesting

Avoid deep nesting (`if let` chains). Instead, extract values into named locals
and use guard clauses (`continue`, `return`, `break`) to exit early. This keeps
the indentation flat and the logic linear.

Specific preferences:

- Avoid `if let` when it hides the "else" / "error" path or encourages nesting.
- Prefer `let-else` for early returns.
- Prefer explicit `match` if multiple arms are needed, but keep the arms
  shallow.

🛑 Bad (Nested):

```rust
for item in &items {
    if let ItemType::Process(item) = item {
        if let Status::Active = item.status {
            match process_item(item) {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }
    }
}
```

✅ **Good (Linear):**

```rust
for item in &items {
    let process = match item {
        ItemType::Process(item) => item,
        _ => continue,
    };

    // Extract status check to a boolean for clarity
    let is_active = matches!(process.status, Status::Active);
    if !is_active {
        continue;
    }

    results.push(process_item(process)?);
}
```

## 2. Separate data extraction from validation ("Peel the Onion")

Do not combine complex destructuring and boolean logic in a single `match` arm.
Do not use "hero one-liners" that match deeply into a structure (like `syn`
ASTs) all at once.

Strategy:

1. Extract the outer layer (e.g., ensure it's a Literal).
2. Check the specific type (e.g., ensure it's a String Literal).
3. Return errors immediately if a step fails.

🛑 Bad (Deep Match & Logic):

```rust
// Hard to debug: did it fail because it's not a Container? Or not a StringItem?
if let Container::Item(ItemData { data: StringType(s), .. }) = val {
   // ...
}

```

✅ **Good (Step-by-Step Extraction):**

```rust
// Step 1: Unwrap Container
let container = match val {
    Data::Container(c) => c,
    _ => return Err("expected container"),
};

// Step 2: Unwrap Item
let item = match container.data {
    ItemType::StringItem(s) => s,
    _ => return Err("expected string item"),
};
```

## 3. Optimize Data Flow (Move over Clone)

Be hyper-aware of ownership. If a function consumes data (takes `T` instead of
`&T`), move the original value instead of cloning it.

- Avoid `func(data.clone())` just to satisfy the compiler quickly.
- Prefer `func(data)` to transfer ownership. Refactor the flow if necessary so
  the move happens naturally (e.g., stop using the variable earlier).
- Clone is acceptable if the original is strictly needed in a later execution
  path.

## 4. Fail Fast in Parsing logic

Don't swallow errors in a fallback chain. If a user attempts a specific format
(e.g., `key = "value"`) but makes a syntax error, return the error immediately
rather than silently failing and trying the next option.

🛑 Bad (Swallowing Errors):

```rust
// If parsing fails, we ignore it and return None, confusing the user
match parse_complex(attr) {
    Ok(val) => return Some(val),
    Err(_) => return None,
}

```

✅ Good (Propagate Errors):

```rust
// If the user clearly tried to use this format but failed, tell them why.
let val = parse_complex(attr)?;
return Some(val);

```

## 5. Group tests by behavior and enforce naming prefixes

Organize unit tests into a single `tests` module per file. Use comment
separators to create distinct visual groups based on functionality or user
workflows (e.g., Execution, Errors, Config). Prefix test function names with
their group category (e.g., `execution_`, `error_`).

Do not create multiple test modules within a single file. All tests should be
grouped within one `mod tests` block using comment separators for organization.

```rust
///////////////////////////////////////////////////////////////////////////////
// Execution Tests

#[test]
fn execution_simple_request_response() {}

///////////////////////////////////////////////////////////////////////////////
// Error Tests

#[test]
fn error_when_field_missing() {}

```

## 6. Prefer combinators over explicit matching for assignment

When transforming `Option` or `Result` types for a variable assignment, prefer
functional combinators (`map`, `and_then`, `unwrap_or_else`) over multi-line
`if let` or `match` blocks.

✅ Good (Combinators):

```rust
let display_name = config
    .display_name
    .as_ref()
    .map_or_else(|| config.default_name.clone(), |s| s.to_string());

```

## 7. General Writing Principles

Doc comments must follow these general writing principles:

1. Be practical over promotional. Focus on what users can do, not on marketing
   language like "powerful," "revolutionary," or "best-in-class."
2. Be honest about limitations. When a feature is missing or limited, say so
   directly and provide workarounds or alternative workflows.
3. Be direct and concise. Use short sentences. Get to the point. Developers are
   scanning, not reading novels.
4. Use second person. Address the reader as "you." Avoid "the user" or "one."
5. Use present tense. "The application opens the file" not "The application will
   open the file."
6. Avoid superlatives without substance ("incredibly fast," "seamlessly
   integrated").
7. Avoid hedging language ("simply," "just," "easily")—if something is simple,
   the instructions will show it.
8. Avoid apologetic tone for missing features—state the limitation and move on.
9. Avoid comparisons that disparage other tools—be factual, not competitive.
10. Avoid meta-commentary about honesty ("the honest take is...", "to be
    frank...", "honestly...").
11. Avoid filler words ("entirely," "certainly," "deeply," "definitely,"
    "actually")—these add nothing.
12. Use simple, direct English. Avoid complex words and academic phrasing.
    Examples: "multiple concerns simultaneously" -> "several concerns",
    "unnecessary coupling" -> "extra dependencies", "convoluted" -> "complex",
    "facilitate" -> "help" or "enable", "in order to" -> "to".
13. Use active voice. "Add feature" not "Feature was added."
14. Keep sentences short and to the point.

## 8. Centralized Error Handling

All error handling must be centralized in a single `src/error.rs` module with a
clear error hierarchy. This provides consistent error types, user-friendly
messages, and proper error chain propagation.

### Error Module Structure

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

### Error Hierarchy Design

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

### Error Conversion and Propagation

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

### Helper Functions for Context

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

### Main Function Pattern

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

### Error Usage Guidelines

- Use `error::Result<T>` (not `std::result::Result<T, Error>`) throughout the
  application
- Propagate errors using the `?` operator, relying on `From` implementations
- Add context at appropriate boundaries (e.g., file paths when parsing files)
- Implement `source()` only when wrapping another error
- Provide user-friendly error messages via `Display`, focusing on what went
  wrong and where
- Avoid panics in production code; use `Result` instead
- For binary applications, return `Result<()>` from `main()`

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

## 9. Doc Comments for API References

Doc comments (`///`) must serve as an API reference, explaining the **what** and
**why** (intent) rather than **how** (implementation details).

- Every module file must include top-level documentation (`//!`) summarizing its
  role.
- Avoid bullet points and unnecessary headers such as "Architecture",
  "Arguments", "Returns". "Example" header is OK. Doc comments should be concise
  and flow as a paragraph. Simple lists are acceptable if they improve clarity.
- Avoid list items that start with bold labels (e.g., "**Important:**",
  "**Note:** "). Write the point directly in plain text instead.
- Write doc comments using clear, simple English that is easy to understand.
  Avoid unnecessarily complex words, jargon, or academic phrasing. Favor plain
  language that communicates concepts directly without ambiguity.

Words to avoid and their simpler alternatives:

- "constitutes" -> use "is", "represents", or "defines"
- "utilize" -> use "use"
- "facilitate" -> use "enable" or "help"
- "in order to" -> use "to"
- "subsequently" -> use "later" or "then"

The goal is to make API documentation accessible to all developers regardless of
their English proficiency or technical background. For end-user documentation,
see the Documentation Writing Guideline section.

🛑 Bad (Bullet Points & Headers):

```rust
/// A handler for network requests.
///
/// This trait defines the interface for processing incoming requests and
/// generating responses. Handlers operate independently of the transport layer.
///
/// - **Transport-agnostic**: Both producers and consumers work with any handler
/// - **Protocol-agnostic**: Handlers don't understand request/response structure
/// - **Bidirectional**: Same handler can be used for both reading and writing
/// - **Simple interface**: Just process data streams
```

🛑 Bad (Arguments/Returns Headers):

```rust
/// Send a data packet.
///
/// This is a low-level I/O operation. The packet should be properly formatted,
/// but the handler doesn't validate this.
///
/// # Arguments
///
/// * `packet` - The data packet to send

/// Receive a data packet.
///
/// This is a low-level I/O operation. The returned packet should be properly
/// formatted, but the handler doesn't validate this.
///
/// # Returns
///
/// The received data packet
```

✅ Good (Concise Paragraph):

```rust
/// A handler for network requests.
///
/// This trait defines the interface for processing requests and generating
/// responses. Handlers operate independently of the transport layer.
///
/// The design is transport-agnostic, allowing both producers and consumers to work
/// with any handler implementation. Handlers are bidirectional and can be used
/// for both read and write roles, with a simple interface focused only on
/// processing data streams.
```

✅ Good (List Format):

```rust
/// A protocol error.
///
/// This represents an error that can be returned by a server
/// or received by a client. It follows the protocol specification.
///
/// Standard error codes:
/// - 1001: Parse error
/// - 1002: Invalid request
/// - 1003: Operation not found
/// - 1004: Invalid parameters
/// - 1005: Internal error
/// - 2000 to 2999: Application error (implementation specific)
```

## 10. Avoid unnecessary tests

Don't write tests that verify language guarantees or basic type properties.
These tests add no value because the compiler already enforces them. Focus on
actual behavior and integration instead.

Avoid these tests:

- Type trait implementation tests (e.g., verifying a struct implements `Clone`)
- Method existence tests (e.g., confirming a trait's method is callable)
- Builder clone tests that don't verify actual behavior
- Any test that would fail to compile if the feature were missing

🛑 Bad (Trait Clone Test):

```rust
#[test]
fn builder_is_cloneable() {
    let builder1 = Builder::new().with_capacity(8192);
    let builder2 = builder1.clone();

    // Both should produce valid resources
    let resource1 = builder1.build();
    let resource2 = builder2.build();

    let _ = (resource1, resource2);
}
```

🛑 Bad (Method Existence Test):

```rust
#[test]
fn handler_has_process_method() {
    // Verify process method exists and is callable
    let mut handler = Handler::new();

    fn has_process<T>(t: &mut T)
    where
        T: Processor,
    {
        let _result = t.process();
    }

    has_process(&mut handler);
}
```

## 11. Use descriptive names for all variables

Use descriptive names for all variables instead of abbreviations. This improves
code readability and maintainability, especially for new contributors.

Common abbreviations to avoid:

- `ty` -> use `type_name` or `typ`
- `msg` -> use `message` or `msg_body`
- `req` -> use `request`
- `resp` -> use `response`
- `arg` -> use `argument` or `arg_value`
- `ctx` -> use `context`
- `val` -> use `value`
- `cfg` -> use `config`
- `str` -> use `string`
- `num` -> use `number`
- `idx` -> use `index`
- `len` -> use `length`
- `cnt` -> use `count`
- `res` -> use `result`

🛑 Bad (Abbreviations):

```rust
fn process(ty: Type, req: Request) -> Response {
    let msg = req.message;
    let ctx = req.context;
    let cfg = req.config;
    let val = req.value;

    // ... implementation
}
```

✅ Good (Descriptive):

```rust
fn process(type_name: Type, request: Request) -> Response {
    let message = request.message;
    let context = request.context;
    let config = request.config;
    let value = request.value;

    // ... implementation
}
```

## 12. Prefer self-contained functions with clear data flow

Functions should manage their own dependencies and generate what they need
internally rather than receiving data through unnecessary intermediate steps.
Each function should have a single, clear responsibility and be self-contained
where possible. This improves encapsulation, simplifies APIs, and creates
clearer data flow.

Principles:

- Functions should access data directly from their primary parameters rather
  than receiving redundant or derivable values as separate arguments
- If a value is only used by one function, generate it inside that function
  instead of passing it from the caller
- Avoid chaining function calls where an intermediate result exists only to be
  passed to the next function
- Split functions that return tuples into focused single-purpose functions
- Name functions that return `TokenStream` with the `_code` suffix for
  consistency

🛑 Bad (Unnecessary Parameter Passing):

```rust
// call_expr is only used by generate_closure_body, but it's created
// in the caller and passed through as a parameter
let call_expr = generate_call_expr(&metadata, &metadata.params);
let closure_body = generate_closure_body(&metadata, &call_expr);

fn generate_call_expr(
    metadata: &method::Metadata,
    params: &method::ParamKind,  // Redundant: metadata already contains params
) -> TokenStream {
    // ...
}

fn generate_closure_body(
    metadata: &method::Metadata,
    call_expr: &TokenStream,  // Unnecessary: could be generated internally
) -> TokenStream {
    // ...
}
```

✅ **Good (Self-Contained Functions):**

```rust
// Each function manages its own dependencies
let closure_body = generate_closure_body_code(&metadata);

fn generate_closure_body_code(metadata: &method::Metadata) -> TokenStream {
    let call_expr = generate_call_expr_code(metadata);  // Generated internally
    match &metadata.returns {
        ReturnKind::Infallible(_) => quote! { Ok(#call_expr) },
        ReturnKind::Fallible { .. } => quote! { #call_expr },
    }
}

fn generate_call_expr_code(metadata: &method::Metadata) -> TokenStream {
    let call_args = generate_call_args_code(&metadata.params);  // Direct access
    match metadata.receiver {
        ReceiverKind::Static => quote! { Self::#name(#call_args) },
        _ => quote! { self.#name(#call_args) },
    }
}
```

✅ **Good (Split Tuple Returns):**

```rust
// Instead of returning a tuple, split into focused functions
// Bad: let (params_type, call_args) = generate_param_code(&params);
// Good:
let params_type = generate_params_type_code(&params);
let call_args = generate_call_args_code(&params);  // Used only where needed
```

This approach provides:

- **Encapsulation**: Functions handle generating what they need internally
- **Simpler API**: Fewer parameters and intermediate variables
- **Better cohesion**: Each function is self-contained and focused
- **Clearer data flow**: Linear, easy-to-follow dependency chain
- **Easier testing**: Functions can be tested independently

## 13. Use module-prefixed function calls for internal modules

All internal modules should be consumed using module-prefixed function calls
rather than importing individual functions. This makes the origin of functions
clear and improves code readability by explicitly showing which module provides
each function.

Use `use crate::module_name;` then call functions as `module_name::function()`.

🛑 Bad (Individual imports):

```rust
use crate::cargo::{nightly, rustdoc};

nightly()?;
rustdoc(&crate_name)?;
```

✅ Good (Module prefix):

```rust
use crate::cargo;

cargo::nightly()?;
cargo::rustdoc(&crate_name)?;
```

Function names should be intuitive when used as `module_name::function`. The
function name should clearly indicate what it does, and when combined with the
module name, should describe the operation intuitively.

For example, `cargo::rustdoc` implies the function executes the `cargo rustdoc`
CLI command under the hood. This naming convention makes code self-documenting
and easy to understand at a glance.
