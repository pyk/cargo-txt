# Rust Coding Guidelines: Error Handling for Binaries

> [!IMPORTANT]
>
> Follow these Rust coding guidelines strictly

## 1. Use anyhow::Result Throughout

Import `anyhow::Result` and use it for all fallible functions. This provides
uniform error handling with automatic source chaining.

🛑 Bad:

```rust
fn parse_html(path: &Path) -> std::result::Result<Document, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    // ...
}
```

✅ Good:

```rust
use anyhow::Result;

fn parse_html(path: &Path) -> Result<Document> {
    let content = std::fs::read_to_string(path)?;
    // ...
}
```

## 2. Use .context() for Error Context

Add context to errors using `.context()` to make errors more understandable.
Context appears in error chains, providing LLM-friendly messages.

🛑 Bad:

```rust
fn parse_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    serde_json::from_str(&content)?
}
```

✅ Good:

```rust
use anyhow::{Context, Result};

fn parse_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context(format!("failed to read config file '{}'", path.display()))?;

    serde_json::from_str(&content)
        .context("failed to parse config as JSON")?
}
```

**Output with context:**

```
Error: failed to parse config as JSON

Caused by:
  failed to read config file 'config.json'

Caused by:
  No such file or directory (os error 2)
```

## 3. Use bail! for Early Returns

Use the `bail!` macro for early error returns with descriptive messages.

🛑 Bad:

```rust
fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("path does not exist"));
    }
    Ok(())
}
```

✅ Good:

```rust
use anyhow::{bail, Result};

fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("path does not exist");
    }
    Ok(())
}
```

## 4. Use ensure! for Validation Conditions

Use the `ensure!` macro to validate conditions with custom error messages.

🛑 Bad:

```rust
fn validate_path(path: &Path) -> Result<()> {
    if !path.is_dir() {
        bail!("path '{}' is not a directory", path.display());
    }
    Ok(())
}
```

✅ Good:

```rust
use anyhow::{ensure, Result};

fn validate_path(path: &Path) -> Result<()> {
    ensure!(
        path.is_dir(),
        "path '{}' is not a directory",
        path.display()
    );
    Ok(())
}
```

## 5. Return Result from main()

For binary applications, return `anyhow::Result<()>` from `main()`. Rust's
standard library automatically displays errors with their context chains and
exits with status code 1.

🛑 Bad:

```rust
fn main() {
    let path = std::env::args().nth(1).unwrap();
    if let Err(e) = validate_path(&path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

✅ Good:

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("missing path argument")?;

    validate_path(&path)?;
    Ok(())
}
```

## 6. Avoid Panics in Production Code

Use `Result` instead of `unwrap()`, `expect()`, or `panic!`. Handle errors
gracefully through proper error propagation with `anyhow`.

🛑 Bad:

```rust
fn parse_config(path: &Path) -> Config {
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).expect("Invalid config")
}
```

✅ Good:

```rust
use anyhow::{Context, Result};

fn parse_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("failed to read config file")?;

    serde_json::from_str(&content)
        .context("failed to parse config as JSON")
}
```

## 7. Write LLM-Friendly Error Messages

Distinguish between **user errors** (user did something wrong) and **internal
errors**.

### User Errors Examples

```
Path '{path}' does not exist or is invalid.

If the path should exist, suggest the user verify the path is correct and they have permission to access it.
```

```
Path '{path}' is not a directory.

Suggest the user use the read_file tool to read files instead.
```

### Internal Errors Examples

```
Internal error: Lint analysis failed for '{path}': {details}

This is a bug in bident. Inform the user to report this issue with:
- Bident version
- Path being linted: {path}
- Error details: {details}
```

```
Internal error: Audit analysis failed for '{path}': {details}

This is a bug in bident. Inform the user to report this issue with:
- Bident version
- Path being audited: {path}
- Error details: {details}
```

## 8. Chain Context at Multiple Levels

Add context at each layer of your call stack to build a clear error chain. This
helps both humans and LLMs understand where errors occurred.

🛑 Bad (Single context):

```rust
fn main() -> Result<()> {
    execute_lint(&path)
        .context("lint failed")?;
    Ok(())
}

fn execute_lint(path: &Path) -> Result<()> {
    std::fs::read_to_string(path.join("config.json"))?;
    Ok(())
}
```

✅ Good (Context chain):

```rust
fn main() -> Result<()> {
    execute_lint(&path)
        .context("lint command failed")?;
    Ok(())
}

fn execute_lint(path: &Path) -> Result<()> {
    let config = load_config(path)
        .context("failed to load lint configuration")?;
    Ok(())
}

fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path.join("config.json"))
        .context("failed to read config file")?;
    // ...
    Ok(config)
}
```

**Error output:**

```
Error: lint command failed

Caused by:
  failed to load lint configuration

Caused by:
  failed to read config file

Caused by:
  No such file or directory (os error 2)
```
