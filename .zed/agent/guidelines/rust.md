# Rust Coding Guidelines

> [!IMPORTANT]
>
> Follow these guidelines strictly. Other Rust coding guidelines can be enforced
> by running `rust-lint`.

1. Avoid unnecessary parameter passing. Don't pass intermediate results used
   only by one function. Let the callee generate what it needs internally.
2. Test business logic, not language features. Don't test trait implementations,
   method existence, or type properties. Focus on behavior, error handling, and
   edge cases.
3. Extract common test setup into helper functions to avoid duplication.
4. Explain what and why in doc comments, not how. Comments must describe intent
   and purpose, not implementation details.
5. Use simple, clear language in documentation. Avoid jargon. Replace "utilize"
   with "use", "facilitate" with "enable", "leverage" with "use".
6. Document behavior and error conditions. Focus on what functions do and when
   they fail. Include examples for non-obvious APIs.
7. Chain error context at multiple levels in binary applications. Add context at
   each layer to build clear error chains that help debugging.
8. Write LLM-friendly error messages. Distinguish user errors from internal bugs
   clearly. For user errors, explain what went wrong and suggest solutions. For
   internal errors, state it's a bug and provide version, path, and details for
   reporting.
9. Use anyhow for binary error handling and thiserror for library error
   handling. Binaries use dynamic errors and context chaining. Libraries use
   structured error types as part of their public API.
10. Define structured error types for libraries. Use explicit error enums as
    part of your public API instead of dynamic errors.
11. Provide static context in error variant fields. Include file paths, field
    names, and operation details in error variant fields rather than building
    them dynamically.
12. Use three-level error hierarchy for libraries. Top-level errors group
    operations with context. Low-level errors are context-free and reusable.
13. Document error variants. Add doc comments to error types and variants
    explaining when each error occurs.
14. Avoid panics in public API. Never use unwrap(), expect(), or panic!() in
    public functions. Always return Result<T> for fallible operations.
