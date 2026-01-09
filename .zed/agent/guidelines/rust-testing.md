# Rust Coding Guidelines: Testing

> [!IMPORTANT]
>
> Follow these strict guidelines for test organization, naming, and quality.

## 1. Organize Tests in Single Module

Use a single `mod tests` per file with comment separators for visual grouping.
Prefix test names with category (e.g., `execution_`, `error_`).

🛑 Bad (Multiple Modules):

```rust
mod execution_tests {
    #[test]
    fn test_simple() {}
}

mod error_tests {
    #[test]
    fn test_missing() {}
}
```

✅ Good:

```rust
///////////////////////////////////////////////////////////////////////////////
// Execution Tests

#[test]
fn execution_simple_request_response() {}

#[test]
fn execution_with_timeout() {}

///////////////////////////////////////////////////////////////////////////////
// Error Tests

#[test]
fn error_when_field_missing() {}
```

## 2. Use Descriptive Test Names

Follow pattern: `<category>_<scenario>_<expected_result>`. Categories match
visual groups.

🛑 Bad:

```rust
#[test]
fn test_1() {}
#[test]
fn test_timeout() {}
```

✅ Good:

```rust
#[test]
fn execution_timeout_cancels_long_operation() {}
#[test]
fn error_when_field_missing_returns_error() {}
#[test]
fn config_default_values_are_applied() {}
```

Common categories: `execution_`, `error_`, `config_`, `validation_`, `edge_`.

## 3. Avoid Testing Language Guarantees

Don't test trait implementations, method existence, or type properties. The
compiler enforces these.

🛑 Bad (Testing Clone):

```rust
#[test]
fn builder_is_cloneable() {
    let builder1 = Builder::new();
    let builder2 = builder1.clone();
    let _ = (builder1.build(), builder2.build());
}
```

🛑 Bad (Testing Trait Existence):

```rust
#[test]
fn handler_has_process_method() {
    let mut handler = Handler::new();
    fn has_process<T>(t: &mut T) where T: Processor {
        let _result = t.process();
    }
    has_process(&mut handler);
}
```

✅ Good (Testing Behavior):

```rust
#[test]
fn execution_process_returns_correct_result() {
    let mut handler = Handler::new();
    let result = handler.process(&Input::new("test"));
    assert_eq!(result.status, Status::Success);
}

#[test]
fn error_invalid_input_returns_error() {
    let mut handler = Handler::new();
    let result = handler.process(&Input::empty());
    assert!(matches!(result, Err(Error::InvalidInput)));
}
```

## 4. Test Business Logic Over Implementation Details

Focus on behavior, error handling, edge cases, and integration. Avoid testing
language features, type system, or trivial getters/setters.

✅ Good:

```rust
#[test]
fn execution_timeout_cancels_long_operation() {
    let handler = Handler::with_timeout(Duration::from_millis(100));
    let slow_operation = SlowOperation::with_delay(Duration::from_secs(1));
    let result = handler.execute(&slow_operation);
    assert!(matches!(result, Err(Error::Timeout)));
}
```

## 5. Extract Common Setup into Helpers

Create helper functions for repeated test setup.

🛑 Bad:

```rust
#[test]
fn execution_success_on_first_attempt() {
    let handler = Handler::new()
        .with_timeout(Duration::from_millis(100))
        .with_retry_count(3);
    // test...
}

#[test]
fn execution_retries_on_failure() {
    let handler = Handler::new()
        .with_timeout(Duration::from_millis(100))
        .with_retry_count(3);
    // test...
}
```

✅ Good:

```rust
fn create_test_handler() -> Handler {
    Handler::new()
        .with_timeout(Duration::from_millis(100))
        .with_retry_count(3)
}

#[test]
fn execution_success_on_first_attempt() {
    let handler = create_test_handler();
    // test...
}
```
