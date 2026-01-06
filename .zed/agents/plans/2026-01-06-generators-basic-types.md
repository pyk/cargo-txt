# Implement Basic Type Generators

Implement markdown generators for fundamental Rust data types: structs, enums,
unions, and type aliases. These generators handle the core data structures that
form the backbone of Rust codebases.

## Current Problems

No generators exist for basic Rust types. These are essential data structures
that coding agents need to understand:

1. **Structs**: Need to show fields, visibility, and documentation
2. **Enums**: Need to show variants, discriminants, and documentation
3. **Unions**: Need to show fields and safety considerations
4. **Type aliases**: Need to show target type and documentation

## Proposed Solution

Create dedicated generators for each basic type following the established
markdown framework:

1. Implement `src/markdown/struct.rs` for struct items
2. Implement `src/markdown/enum.rs` for enum items
3. Implement `src/markdown/union.rs` for union items
4. Implement `src/markdown/alias.rs` for type alias items
5. Integrate generators into build command
6. Add comprehensive tests for each generator

## Implementation Checklist

### Struct Generator

- [ ] Create `src/markdown/struct.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_struct(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract struct data from item
    - Render header with struct name
    - Render documentation text
    - Render field list with types and visibility
    - Add "Next Actions" section with view source and related items
- [ ] Implement
      `render_fields(fields: &[Id], krate: &Crate, item_map: &Index) -> String`:
    - Generate bullet list of fields
    - Include type information
    - Mark visibility (pub or not)
    - Include field documentation if available
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with plain struct (no generics)
    - Test with generic struct
    - Test with tuple struct
    - Test with unit struct
    - Test with missing documentation
    - Test with private fields

### Enum Generator

- [ ] Create `src/markdown/enum.rs` module
- [ ] Create `src/markdown/enum.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_enum(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract enum data from item
    - Render header with enum name
    - Render documentation text
    - Render variant list with types and discriminants
    - Add "Next Actions" section
- [ ] Implement
      `render_variants(variants: &[Id], krate: &Crate, item_map: &Index) -> String`:
    - Generate bullet list of variants
    - Include associated data types
    - Show discriminant values if explicit
    - Include variant documentation
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple enum (no data)
    - Test with data-carrying variants
    - Test with explicit discriminants
    - Test with missing documentation

### Union Generator

- [ ] Create `src/markdown/union.rs` module
- [ ] Create `src/markdown/union.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_union(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract union data from item
    - Render header with union name
    - Render documentation text
    - Render field list
    - Add safety note section about unsafe access
    - Add "Next Actions" section
- [ ] Implement
      `render_union_fields(fields: &[Id], krate: &Crate, item_map: &Index) -> String`:
    - Generate bullet list of fields
    - Include type information
    - Add safety context
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with union containing various types
    - Test with missing documentation
    - Verify safety note is included

### Type Alias Generator

- [ ] Create `src/markdown/alias.rs` module
- [ ] Create `src/markdown/alias.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_alias(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract type alias data from item
    - Render header with alias name
    - Render documentation text
    - Render target type in code block
    - Add "Next Actions" section
- [ ] Implement `render_target_type(type_: &Type) -> String`:
    - Format type as inline code or code block
    - Handle complex generics
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple alias (e.g.,
      `type Result<T> = std::result::Result<T, Error>;`)
    - Test with complex generic bounds
    - Test with missing documentation

### Integration

- [ ] Update `src/markdown/mod.rs`:
    - Add `pub mod struct_;` (use trailing underscore to avoid keyword conflict)
    - Add `pub mod enum_;`
    - Add `pub mod union;`
    - Add `pub mod alias;`
    - Re-export `Result` from error module for convenience
- [ ] Update `src/commands/build.rs`:
    - Import the new generator modules
    - Add logic to dispatch to appropriate generator based on item type
    - Generate markdown for all struct items in the crate
    - Generate markdown for all enum items in the crate
    - Generate markdown for all union items in the crate
    - Generate markdown for all type alias items in the crate
- [ ] Update `DOCS.md` with examples:
    - Show generated struct markdown
    - Show generated enum markdown
    - Show generated union markdown
    - Show generated type alias markdown

### Tests

- [ ] Add integration test for full struct generation
- [ ] Add integration test for full enum generation
- [ ] Add integration test for full union generation
- [ ] Add integration test for full type alias generation
- [ ] Verify all tests pass with `cargo test`

## Test Plan

### Verification Tests

#### Struct Generator

- [ ] Verify plain struct shows all fields
- [ ] Verify generic struct shows type parameters
- [ ] Verify tuple struct shows unnamed fields with positions
- [ ] Verify unit struct shows minimal content
- [ ] Verify field visibility is correctly indicated
- [ ] Verify field documentation is included
- [ ] Verify generated file name follows convention

#### Enum Generator

- [ ] Verify simple enum shows all variants
- [ ] Verify data-carrying variants show types
- [ ] Verify explicit discriminants are shown
- [ ] Verify variant documentation is included
- [ ] Verify generated file name follows convention

#### Union Generator

- [ ] Verify union shows all fields
- [ ] Verify safety note is included
- [ ] Verify field types are correctly shown
- [ ] Verify generated file name follows convention

#### Type Alias Generator

- [ ] Verify target type is correctly displayed
- [ ] Verify generic parameters are shown
- [ ] Verify documentation is included
- [ ] Verify generated file name follows convention

### Integration Tests

- [ ] Verify build command generates all struct files
- [ ] Verify build command generates all enum files
- [ ] Verify build command generates all union files
- [ ] Verify build command generates all type alias files
- [ ] Verify index links work to generated files

### Regression Tests

- [ ] Verify index page still works
- [ ] Verify no compiler warnings
- [ ] Verify existing tests still pass

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── error.rs                # Centralized error definitions (from core infrastructure)
│   ├── markdown/
│   │   ├── mod.rs              # Updated with new exports
│   │   ├── index.rs            # Existing
│   │   ├── utils.rs            # Existing
│   │   ├── struct_.rs          # NEW: Struct generator
│   │   ├── enum_.rs            # NEW: Enum generator
│   │   ├── union.rs            # NEW: Union generator
│   │   └── alias.rs            # NEW: Type alias generator
│   └── commands/
│       └── build.rs            # Updated to use generators
```

### Module Exports

```rust
// src/markdown/mod.rs
pub mod index;
pub mod utils;
pub mod struct_;
pub mod enum_;
pub mod union;
pub mod alias;

// Re-export from error module for convenience
pub use crate::error::{Error, MarkdownError, Result};

// Re-export commonly used functions
pub use struct_::generate_struct;
pub use enum_::generate_enum;
pub use union::generate_union;
pub use alias::generate_alias;
```

### Example Generated Struct Markdown

```markdown
# Point

A 2D point in Cartesian coordinates.

## Fields

- `x: f64` - X coordinate
- `y: f64` - Y coordinate

## Next Actions

- View source: `cargo docmd browse --item 0:3:4`
- Find related structs: `cargo docmd browse --type struct`
```

### Example Generated Enum Markdown

```markdown
# Option

A type representing a value that may or may not exist.

## Variants

- `Some(T)` - Some value of type `T`
- `None` - No value

## Next Actions

- View source: `cargo docmd browse --item 0:3:5`
- Find related enums: `cargo docmd browse --type enum`
```

### Example Generated Union Markdown

```markdown
# Any

A dynamically-typed value.

**Safety**: Accessing union fields requires unsafe code. Only access the field
that was most recently written to.

## Fields

- `integer: i64`
- `float: f64`
- `text: *const u8`

## Next Actions

- View source: `cargo docmd browse --item 0:3:6`
- Find related unions: `cargo docmd browse --type union`
```

### Example Generated Type Alias Markdown

````markdown
# Result

Result type alias for convenience.

## Type

```rust
type Result<T> = std::result::Result<T, Error>;
```
````

## Next Actions

- View source: `cargo docmd browse --item 0:3:7`
- Find related aliases: `cargo docmd browse --type type-alias`

```

## Design Considerations

### 1. Module Naming

**Decision**: Use trailing underscore for `struct_` and `enum_`.

- **Alternative**: Use different names like `structure` and `enumeration`.
    - Rejected: Less conventional and more verbose
- **Alternative**: Use `type_` pattern.
    - Rejected: `type` is a more common keyword to collide with than `struct`
- **Resolution**: Trailing underscore is the Rust convention for avoiding keyword
  conflicts

### 2. Field Visibility

**Decision**: Show visibility indicators for struct fields.

- **Alternative**: Only show public fields.
    - Rejected: Private fields are still relevant for understanding the struct
- **Alternative**: Don't show visibility at all.
    - Rejected: Visibility is important for understanding API surface
- **Resolution**: Mark public fields explicitly, leave others unmarked

### 3. Safety Notes for Unions

**Decision**: Include explicit safety note for unions.

- **Alternative**: Let documentation speak for itself.
    - Rejected: Unions are inherently unsafe, explicit reminder is helpful
- **Alternative**: Mark union documentation as unsafe with formatting.
    - Rejected: Simpler to just add a clear note section
- **Resolution**: Add a dedicated safety note section before the field list

### 4. Discriminant Display

**Decision**: Show explicit discriminants for enums.

- **Alternative**: Only show implicit discriminants.
    - Rejected: Explicit discriminants are important for understanding the enum
- **Alternative**: Always show all discriminant values.
    - Rejected: Clutter when all are implicit
- **Resolution**: Show discriminants only when explicitly set by the user

### 4. Type Alias Display

**Decision**: Show target type in code block.

- **Alternative**: Show as inline code.
    - Rejected: Complex types with generics are hard to read inline
- **Alternative**: Show both inline and in code block.
    - Rejected: Redundant, one format is sufficient
- **Resolution**: Code block provides best readability for complex types

### 5. Error Handling

**Decision**: Use centralized error types from `src/error.rs`.

- **Alternative**: Define local error types in each generator.
    - Rejected: Centralized errors provide consistency across the codebase
- **Resolution**: Import `Result` and `MarkdownError` from error module for
  consistent error handling and messaging

## Success Criteria

- [ ] All four generators are implemented
- [ ] Generators handle all variants of their respective types
- [ ] Generated markdown follows standard structure
- [ ] All generated files use correct naming convention
- [ ] Build command generates files for all basic type items
- [ ] Index page links to generated files correctly
- [ ] Error messages include full paths on failure
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] No compiler warnings
- [ ] Documentation is complete and clear

## Implementation Status: 🟡 NOT STARTED

## Implementation Notes

Space for recording specific technical details or roadblocks encountered during
work.
```
