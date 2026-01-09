# Rust Coding Guidelines: Control Flow & Core Principles

> [!IMPORTANT]
>
> Follow these Rust coding guidelines strictly for control flow, function
> design, and general code structure.

## 1. Use Guard Clauses Over Deep Nesting

Use guard clauses (`continue`, `return`, `break`) over deep nesting. Extract
values to named locals.

🛑 Bad:

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

✅ Good:

```rust
for item in &items {
    let process = match item {
        ItemType::Process(item) => item,
        _ => continue,
    };
    if !matches!(process.status, Status::Active) { continue; }
    results.push(process_item(process)?);
}
```

## 2. Separate Data Extraction from Validation

Extract data step-by-step, returning errors immediately at each failure point.
Avoid complex destructuring in single `match` arms.

🛑 Bad:

```rust
if let Container::Item(ItemData { data: StringType(s), .. }) = val { /* ... */ }
```

✅ Good:

```rust
let container = match val {
    Data::Container(c) => c,
    _ => return Err("expected container"),
};
let item = match container.data {
    ItemType::StringItem(s) => s,
    _ => return Err("expected string item"),
};
```

## 3. Prefer Move or Borrow Over Clone

Prefer moving or borrowing data instead of cloning. Use `func(data)` or
`func(&data)` over `func(data.clone())`.

🛑 Bad:

```rust
fn process_data(config: Config) -> Result<String> {
    let data = fetch_data(&config)?;
    let parsed = parse_data(data.clone())?;
    let result = format_output(data, parsed)?;
    Ok(result)
}
```

✅ Good:

```rust
fn process_data(config: Config) -> Result<String> {
    let data = fetch_data(&config)?;
    let parsed = parse_data(data)?;
    let result = format_output(parsed)?;
    Ok(result)
}
```

## 4. Prefer Combinators Over Explicit Matching

Use `map`, `and_then`, `unwrap_or_else` for `Option`/`Result` transformation
instead of `if let`/`match`.

🛑 Bad:

```rust
let display_name = if let Some(name) = &config.display_name {
    name.to_string()
} else {
    config.default_name.clone()
};
```

✅ Good:

```rust
let display_name = config.display_name
    .as_ref()
    .map_or_else(|| config.default_name.clone(), |s| s.to_string());
```

## 5. Use Descriptive Variable Names

Avoid abbreviations. Use full words: `type_name`, `message`, `request`,
`response`, `argument`, `context`, `value`, `config` instead of `ty`, `msg`,
`req`, `resp`, `arg`, `ctx`, `val`, `cfg`.

🛑 Bad:

```rust
fn process(ty: Type, req: Request) -> Response {
    let msg = req.message;
    let ctx = req.context;
    let cfg = req.config;
}
```

✅ Good:

```rust
fn process(type_name: Type, request: Request) -> Response {
    let message = request.message;
    let context = request.context;
    let config = request.config;
}
```

## 6. Avoid Unnecessary Parameter Passing

Don't pass intermediate results used only by one function. Let the callee
generate what it needs internally.

🛑 Bad:

```rust
let call_expr = generate_call_expr(&metadata, &metadata.params);
let closure_body = generate_closure_body(&metadata, &call_expr);
```

✅ Good:

```rust
let closure_body = generate_closure_body_code(&metadata);
fn generate_closure_body_code(metadata: &method::Metadata) -> TokenStream {
    let call_expr = generate_call_expr_code(metadata);
    match &metadata.returns {
        ReturnKind::Infallible(_) => quote! { Ok(#call_expr) },
        ReturnKind::Fallible { .. } => quote! { #call_expr },
    }
}
```

## 7. Avoid Returning Tuples for Unrelated Data

Avoid returning tuples containing unrelated data. Split into single-purpose
functions for better readability and testing.

🛑 Bad:

```rust
let (params_type, call_args) = generate_param_code(&params);
```

✅ Good:

```rust
let params_type = generate_params_type_code(&params);
let call_args = generate_call_args_code(&params);
```

## 8. Use `_code` Suffix for TokenStream Functions

Name functions returning `TokenStream` with `_code` suffix for consistency.

🛑 Bad:

```rust
fn generate_struct(input: &StructInput) -> TokenStream { quote! { struct #input; } }
```

✅ Good:

```rust
fn generate_struct_code(input: &StructInput) -> TokenStream { quote! { struct #input; } }
```

## 9. Use Module-Prefixed Function Calls

Use `use crate::module_name;` then call as `module_name::function()`. Improves
origin clarity and prevents naming conflicts.

🛑 Bad:

```rust
use crate::cargo::{nightly, rustdoc};
nightly()?;
rustdoc(&crate_name)?;
```

✅ Good:

```rust
use crate::cargo;
cargo::nightly()?;
cargo::rustdoc(&crate_name)?;
```
