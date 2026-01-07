# Type Alias `serde_json::Result`

```rust
pub type Result<T> = Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.

## Aliased Type

```rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

## Variants

- `Ok(T)`: Contains the success value
- `Err(Error)`: Contains the error value

## Implementations

### impl<T, E> Result<&T, E>

#### pub const fn copied(self) -> Result<T, E> where T: Copy

Maps a `Result<&T, E>` to a `Result<T, E>` by copying the contents of the `Ok`
part.

#### pub fn cloned(self) -> Result<T, E> where T: Clone

Maps a `Result<&T, E>` to a `Result<T, E>` by cloning the contents of the `Ok`
part.

<other implementations>

## Trait Implementations

### impl<T, E> Clone for Result<T, E> where T: Clone, E: Clone,

#### fn clone(&self) -> Result<T, E>

Returns a duplicate of the value.

<other trait implementations>
