# Rust Coding Guidelines: Error Handling for Libraries

> [!IMPORTANT]
>
> Follow these Rust coding guidelines strictly

## 1. Define Structured Error Types with thiserror

Use `#[derive(Error, Debug)]` from `thiserror` to create explicit error enums.
Error types should be part of your public API.

🛑 Bad (dynamic errors):

```rust
pub fn parse_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    serde_json::from_str(&content)?
}
```

✅ Good (structured errors):

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{0}'")]
    ReadFailed(PathBuf, #[source] std::io::Error),

    #[error("failed to parse config as JSON: {0}")]
    ParseFailed(#[source] serde_json::Error),

    #[error("config file '{0}' is missing required field '{1}'")]
    MissingField(PathBuf, String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

pub fn parse_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadFailed(path.to_path_buf(), e))?;
    serde_json::from_str(&content).map_err(ConfigError::ParseFailed)?
}
```

## 2. Use Field-Level Error Messages

Place error messages directly on error variant fields using `#[error(...)]`
attribute. This makes error types self-documenting.

🛑 Bad (manual Display impl):

```rust
#[derive(Debug)]
pub enum ConfigError {
    ReadFailed(PathBuf, std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::ReadFailed(path, _) => {
                write!(f, "failed to read config file '{}'", path.display())
            }
        }
    }
}
```

✅ Good (field-level messages):

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{0}'")]
    ReadFailed(PathBuf, #[source] std::io::Error),
}
```

## 3. Capture Source Errors with #[source]

Use `#[source]` attribute to wrap underlying errors. This provides access to the
cause via `.source()` method while preserving clean error messages.

🛑 Bad (no source):

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {err}")]
    ReadFailed { path: PathBuf, err: String },
}
```

✅ Good (with source):

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{0}'")]
    ReadFailed(PathBuf, #[source] std::io::Error),
}
```

## 4. Provide Context in Error Variants

Include context (file paths, field names, operation details) in error variant
fields rather than building it dynamically.

🛑 Bad (dynamic context):

```rust
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse failed: {message}")]
    Failed { message: String },
}

// Later
return Err(ParseError::Failed {
    message: format!("missing field '{}' in config", field_name),
});
```

✅ Good (static context):

```rust
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing required field '{1}' in config file '{0}'")]
    MissingField(PathBuf, String),

    #[error("invalid type for field '{1}' in config file '{0}'")]
    InvalidType(PathBuf, String),
}
```

## 5. Implement From for Error Conversions

Implement `From` trait to automatically convert lower-level errors to your error
types. This makes error propagation clean.

🛑 Bad (manual conversions):

```rust
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadFailed(path.to_path_buf(), e))?;
    // ...
}
```

✅ Good (From impls):

```rust
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::ReadFailed(PathBuf::new(), err)
    }
}

pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    // ...
}
```

Note: Only use `From` when the conversion doesn't need additional context. When
context is needed (like file path), use explicit `.map_err()`.

## 6. Document Error Variants

Add doc comments to error types and variants to explain when each error occurs.
This helps library users understand error conditions.

🛑 Bad (undocumented):

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("file not found: {0}")]
    NotFound(PathBuf),
}
```

✅ Good (documented):

```rust
/// Errors that can occur when loading or parsing configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Config file does not exist at the specified path.
    #[error("config file not found: '{0}'")]
    NotFound(PathBuf),

    /// Failed to read config file due to permissions or I/O error.
    #[error("failed to read config file '{0}'")]
    ReadFailed(PathBuf, #[source] std::io::Error),

    /// Config file contains invalid JSON or YAML syntax.
    #[error("failed to parse config: {0}")]
    ParseFailed(#[source] Box<dyn std::error::Error + Send + Sync>),
}
```

## 7. Use Three-Level Error Hierarchy

Organize errors into top-level -> category-level -> low-level. Top-level errors
include context, low-level errors are context-free.

🛑 Bad (flat structure):

```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("config file '{path}' not found")]
    ConfigNotFound { path: PathBuf },

    #[error("lint failed for '{path}': {details}")]
    LintFailed { path: PathBuf, details: String },

    #[error("audit failed for '{path}': {details}")]
    AuditFailed { path: PathBuf, details: String },
}
```

✅ Good (three-level):

```rust
// Top-level: groups by operation with context
#[derive(Debug, Error)]
pub enum Error {
    #[error("config error: {0}")]
    Config(#[source] ConfigError),

    #[error("lint error for '{path}': {source}")]
    Lint { path: PathBuf, source: LintError },

    #[error("audit error for '{path}': {source}")]
    Audit { path: PathBuf, source: AuditError },
}

// Low-level: context-free, reusable
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("file not found")]
    NotFound,

    #[error("read failed: {0}")]
    ReadFailed(#[source] std::io::Error),
}

#[derive(Debug, Error)]
pub enum LintError {
    #[error("path validation failed: {0}")]
    InvalidPath(#[source] std::io::Error),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("path validation failed: {0}")]
    InvalidPath(#[source] std::io::Error),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),
}
```

## 8. Override Debug to Delegate to Display

Implement custom `Debug` that delegates to `Display` for clean error output. Use
`#[error]` attribute instead of manual impls.

🛑 Bad (manual impl):

```rust
#[derive(Debug)]
pub enum Error {
    #[error("failed to read: {0}")]
    ReadFailed(PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ReadFailed(path) => write!(f, "failed to read '{}'", path.display()),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
```

✅ Good (thiserror handles it):

```rust
#[derive(Error, Debug)]  // Both Display and Debug are auto-generated
pub enum Error {
    #[error("failed to read: {0}")]
    ReadFailed(PathBuf),
}
```

## 9. Define Result Type Alias

Create a `Result<T>` type alias for your error type. This makes function
signatures cleaner and consistent.

🛑 Bad (verbose):

```rust
pub fn parse_config(path: &Path) -> std::result::Result<Config, Error> {
    // ...
}

pub fn load_config(path: &Path) -> std::result::Result<Config, Error> {
    // ...
}
```

✅ Good (type alias):

```rust
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_config(path: &Path) -> Result<Config> {
    // ...
}

pub fn load_config(path: &Path) -> Result<Config> {
    // ...
}
```

## 10. Avoid Panics in Public API

Never use `unwrap()`, `expect()`, or `panic!()` in public functions. Always
return `Result<T>` for fallible operations.

🛑 Bad:

```rust
pub fn get_config_value(path: &Path, key: &str) -> String {
    let config = load_config(path).unwrap();
    config.get(key).expect("key not found").clone()
}
```

✅ Good:

```rust
pub fn get_config_value(path: &Path, key: &str) -> Result<String, ConfigError> {
    let config = load_config(path)?;
    config.get(key)
        .cloned()
        .ok_or_else(|| ConfigError::MissingKey(key.to_string()))
}
```

---

**Key Differences from Binary Error Handling:**

| Aspect         | Binary (anyhow)           | Library (thiserror)           |
| -------------- | ------------------------- | ----------------------------- |
| Error Type     | Dynamic (`anyhow::Error`) | Structured enums              |
| API Stability  | Not critical              | Critical - part of public API |
| Context        | Added via `.context()`    | Part of error variants        |
| Documentation  | Less strict               | Must document all errors      |
| Error Matching | Requires downcasting      | Direct pattern matching       |
| Dependencies   | anyhow                    | thiserror                     |
