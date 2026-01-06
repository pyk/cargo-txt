# Type Alias `Result`

**Namespace:** `serde_json`

**Definition:**

```rust
pub type Result<T> = Result<T, Error>;
```

**Aliased Type:**

```rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

### Description

Alias for a `Result` with the error type `serde_json::Error`.

---

## Variants

| Variant | Type    | Description                |
| ------- | ------- | -------------------------- |
| `Ok`    | `T`     | Contains the success value |
| `Err`   | `Error` | Contains the error value   |

---

## Implementations

This type inherits all implementations from `core::result::Result<T, E>`.

### Inspectors

```rust
pub const fn is_ok(&self) -> bool
pub fn is_ok_and<F>(self, f: F) -> bool where F: FnOnce(T) -> bool
pub const fn is_err(&self) -> bool
pub fn is_err_and<F>(self, f: F) -> bool where F: FnOnce(E) -> bool
```

### Converters

```rust
pub fn ok(self) -> Option<T>
pub fn err(self) -> Option<E>
pub const fn as_ref(&self) -> Result<&T, &E>
pub const fn as_mut(&mut self) -> Result<&mut T, &mut E>
pub fn as_deref(&self) -> Result<&T::Target, &E> where T: Deref
pub fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E> where T: DerefMut
```

### Transformers

```rust
pub fn map<U, F>(self, op: F) -> Result<U, E> where F: FnOnce(T) -> U
pub fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U
pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where D: FnOnce(E) -> U, F: FnOnce(T) -> U
pub fn map_err<F, O>(self, op: O) -> Result<T, F> where O: FnOnce(E) -> F
pub fn inspect<F>(self, f: F) -> Result<T, E> where F: FnOnce(&T)
pub fn inspect_err<F>(self, f: F) -> Result<T, E> where F: FnOnce(&E)
```

### Combinators

```rust
pub fn and<U>(self, res: Result<U, E>) -> Result<U, E>
pub fn and_then<U, F>(self, op: F) -> Result<U, E>
    where F: FnOnce(T) -> Result<U, E>
pub fn or<F>(self, res: Result<T, F>) -> Result<T, F>
pub fn or_else<F, O>(self, op: O) -> Result<T, F>
    where O: FnOnce(E) -> Result<T, F>
```

### Extractors (Unwrap)

```rust
pub fn expect(self, msg: &str) -> T where E: Debug
pub fn unwrap(self) -> T where E: Debug
pub fn unwrap_or(self, default: T) -> T
pub fn unwrap_or_else<F>(self, op: F) -> T where F: FnOnce(E) -> T
pub fn unwrap_or_default(self) -> T where T: Default
pub fn expect_err(self, msg: &str) -> E where T: Debug
pub fn unwrap_err(self) -> E where T: Debug
```

### Unsafe

```rust
pub unsafe fn unwrap_unchecked(self) -> T
pub unsafe fn unwrap_err_unchecked(self) -> E
```

### Iterators

```rust
pub fn iter(&self) -> Iter<'_, T>
pub fn iter_mut(&mut self) -> IterMut<'_, T>
```

### Specialized Impls (Type Constructors)

- `impl Result<&T, E>`: `copied`, `cloned`
- `impl Result<&mut T, E>`: `copied`, `cloned`
- `impl Result<Option<T>, E>`: `transpose`
- `impl Result<Result<T, E>, E>`: `flatten`
- `impl IntoIterator for Result<T, E>`: `into_iter`

---

## Trait Implementations

- **`Clone`**, **`Copy`**: (if `T` and `E` satisfy bounds)
- **`Debug`**, **`Display`**: (if `T` and `E` satisfy bounds)
- **`PartialEq`**, **`Eq`**, **`PartialOrd`**, **`Ord`**, **`Hash`**: (standard
  comparison traits)
- **`FromIterator<Result<A, E>>`**: Collects iterator of results into
  `Result<V, E>`. Short-circuits on `Err`.
- **`Product<Result<U, E>>`**, **`Sum<Result<U, E>>`**: Aggregates results.
- **`Try`**: Enables the `?` operator.
- **`Termination`**: Allows `Result` as a return type for `main` functions.
- **`IntoIterator`**: Yields the value if `Ok`, empty if `Err`.
