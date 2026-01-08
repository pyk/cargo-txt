# Rust Coding Guidelines: Testing

CRITICAL: All test code must follow these strict guidelines for organization and
test selection.

## Test Organization

Organize unit tests into a single `tests` module per file. Use comment
separators to create distinct visual groups based on functionality or user
workflows (e.g., Execution, Errors, Config). Prefix test function names with
their group category (e.g., `execution_`, `error_`).

Do not create multiple test modules within a single file. All tests should be
grouped within one `mod tests` block using comment separators for organization.

✅ Good (Organized Tests):

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

#[test]
fn error_invalid_format() {}

///////////////////////////////////////////////////////////////////////////////
// Config Tests

#[test]
fn config_default_values() {}
```

## Test Naming Conventions

Use descriptive test names that follow this pattern:

`<category>_<scenario_under_test>_<expected_result>`

Categories should match the visual group separators. Common categories:

- `execution_` - Tests for happy path execution
- `error_` - Tests for error conditions
- `config_` - Tests for configuration handling
- `validation_` - Tests for input validation
- `edge_` - Tests for edge cases

## Avoid Unnecessary Tests

Don't write tests that verify language guarantees or basic type properties.
These tests add no value because the compiler already enforces them. Focus on
actual behavior and integration instead.

### Tests to Avoid

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

This test is unnecessary because:

- The compiler enforces `Clone` implementation at compile time
- The test doesn't verify the clone produces correct results
- If `Clone` weren't implemented, the test would fail to compile anyway

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

This test is unnecessary because:

- The compiler enforces trait bounds at compile time
- The test doesn't verify the method produces correct results
- If the trait weren't implemented, the code wouldn't compile

✅ Good (Behavior-Focused Tests):

```rust
#[test]
fn execution_process_returns_correct_result() {
    let mut handler = Handler::new();
    let input = Input::new("test data");

    let result = handler.process(&input);

    assert_eq!(result.status, Status::Success);
    assert_eq!(result.output, "processed: test data");
}

#[test]
fn error_invalid_input_returns_error() {
    let mut handler = Handler::new();
    let input = Input::empty();

    let result = handler.process(&input);

    assert!(matches!(result, Err(Error::InvalidInput)));
}
```

## Test Quality Guidelines

### Test What Matters

Focus tests on:

- **Business logic**: Does the function produce the correct output?
- **Error handling**: Are errors returned in the right conditions?
- **Edge cases**: What happens with empty inputs, boundary values, etc.?
- **Integration**: Do components work together correctly?

Avoid tests for:

- **Language features**: The compiler ensures these work
- **Type system**: Rust's type system is already tested
- **Trivial getters/setters**: Unless they have complex logic

### Make Tests Readable

Tests should document expected behavior:

✅ Good (Self-Documenting):

```rust
#[test]
fn execution_timeout_cancels_long_operation() {
    let handler = Handler::with_timeout(Duration::from_millis(100));
    let slow_operation = SlowOperation::with_delay(Duration::from_secs(1));

    let result = handler.execute(&slow_operation);

    assert!(matches!(result, Err(Error::Timeout)));
}
```

### Use Test Helpers

Extract common test setup into helper functions:

```rust
fn create_test_handler() -> Handler {
    Handler::new()
        .with_timeout(Duration::from_millis(100))
        .with_retry_count(3)
}

#[test]
fn execution_success_on_first_attempt() {
    let handler = create_test_handler();
    // ... test logic
}
```
