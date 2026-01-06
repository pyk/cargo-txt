# Implement Advanced Type Generators

Implement markdown generators for advanced and specialized Rust items including
modules, macros, static items, and module-level declarations. These generators
handle the remaining item types that complete the rustdoc coverage.

## Current Problems

No generators exist for advanced and specialized Rust items:

1. **Modules**: Need to show nested structure and documentation
2. **Macros**: Need to handle both declarative and procedural macros
3. **Static items**: Need to show type, visibility, and thread safety
4. **Module-level items**: Extern crate, use, trait alias, extern type,
   primitive types

These items are important for understanding code organization, compile-time
behavior, and type system features.

## Proposed Solution

Create generators for advanced items following the established framework:

1. Implement `src/markdown/module.rs` for module items
2. Implement `src/markdown/macro.rs` for both macro types
3. Implement `src/markdown/static_item.rs` for static items
4. Implement `src/markdown/trait_alias.rs` for trait aliases
5. Implement `src/markdown/extern_crate.rs` for extern crate items
6. Implement `src/markdown/use_item.rs` for use items
7. Implement `src/markdown/primitive.rs` for primitive types
8. Implement `src/markdown/extern_type.rs` for extern types
9. Integrate generators into build command
10. Add comprehensive tests

## Implementation Checklist

### Module Generator

- [ ] Create `src/markdown/module.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_module(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract module data from item
    - Render header with module name
    - Render documentation text
    - Render nested items summary (counts by type)
    - Render list of public items with links
    - Add "Next Actions" section
- [ ] Implement
      `render_nested_items(items: &[Id], krate: &Crate, item_map: &Index) -> String`:
    - Group items by type
    - Show counts for each type
    - List items with links to detail pages
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple module
    - Test with nested modules
    - Test with various item types
    - Test with missing documentation

### Macro Generator

- [ ] Create `src/markdown/macro.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_macro(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract macro data from item
    - Determine macro type (declarative or procedural)
    - Render header with macro name
    - Render documentation text
    - Render macro signature or definition
    - Add type indicator (declarative vs procedural)
    - Add "Next Actions" section
- [ ] Implement
      `render_macro_definition(macro_data: &str, kind: MacroKind) -> String`:
    - Format macro rules for declarative macros
    - Show attribute for procedural macros
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with declarative macro (macro_rules!)
    - Test with procedural macro (derive macro)
    - Test with attribute procedural macro
    - Test with function-like procedural macro
    - Test with missing documentation

### Static Item Generator

- [ ] Create `src/markdown/static_item.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_static(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract static data from item
    - Render header with static name
    - Render documentation text
    - Render type
    - Render mutability (mut vs immutable)
    - Add thread safety note
    - Add "Next Actions" section
- [ ] Implement `render_static_type(type_: &Type, mutable: bool) -> String`:
    - Show type in code block
    - Indicate mutability
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with immutable static
    - Test with mutable static
    - Test with generic static
    - Test with missing documentation
    - Verify thread safety note is included

### Trait Alias Generator

- [ ] Create `src/markdown/trait_alias.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_trait_alias(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract trait alias data from item
    - Render header with alias name
    - Render documentation text
    - Render aliased traits
    - Show generic parameters and bounds
    - Add "Next Actions" section
- [ ] Implement `render_aliased_traits(bounds: &[GenericBound]) -> String`:
    - Format trait list
    - Include generic parameters
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple trait alias
    - Test with complex generic bounds
    - Test with multiple traits
    - Test with missing documentation

### Extern Crate Generator

- [ ] Create `src/markdown/extern_crate.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_extern_crate(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract extern crate data from item
    - Render header with crate name
    - Render documentation text
    - Show rename if present
    - Add "Next Actions" section
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple extern crate
    - Test with renamed extern crate
    - Test with missing documentation

### Use Item Generator

- [ ] Create `src/markdown/use_item.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_use(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract use data from item
    - Render header showing the use path
    - Render documentation text
    - Show visibility
    - Add "Next Actions" section
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with simple use
    - Test with glob use
    - Test with nested use
    - Test with renamed use
    - Test with missing documentation

### Primitive Type Generator

- [ ] Create `src/markdown/primitive.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_primitive(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract primitive type data from item
    - Render header with type name
    - Render documentation text
    - Show type category (integer, float, bool, etc.)
    - Add "Next Actions" section
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with various primitive types
    - Test with missing documentation

### Extern Type Generator

- [ ] Create `src/markdown/extern_type.rs` module
- [ ] Import `Result` and `MarkdownError` from `src/error.rs`
- [ ] Implement
      `generate_extern_type(krate: &Crate, item: &Item, output_dir: &Path) -> Result<()>`:
    - Extract extern type data from item
    - Render header with type name
    - Render documentation text
    - Add note about FFI usage
    - Add "Next Actions" section
- [ ] Add module-level documentation
- [ ] Create unit tests:
    - Test with extern type
    - Test with missing documentation
    - Verify FFI note is included

### Integration

- [ ] Update `src/markdown/mod.rs`:
    - Add `pub mod module;`
    - Add `pub mod macro;`
    - Add `pub mod static_item;`
    - Add `pub mod trait_alias;`
    - Add `pub mod extern_crate;`
    - Add `pub mod use_item;`
    - Add `pub mod primitive;`
    - Add `pub mod extern_type;`
    - Re-export `Result` from error module for convenience
- [ ] Update `src/commands/build.rs`:
    - Import the new generator modules
    - Add dispatch logic for all new item types
    - Generate markdown for all module items
    - Generate markdown for all macro items
    - Generate markdown for all static items
    - Generate markdown for all other advanced items
- [ ] Update `DOCS.md` with examples

### Tests

- [ ] Add integration tests for each generator type
- [ ] Verify all tests pass with `cargo test`

## Test Plan

### Verification Tests

#### Module Generator

- [ ] Verify simple module shows nested items
- [ ] Verify nested modules are linked correctly
- [ ] Verify items are grouped by type
- [ ] Verify item counts are accurate
- [ ] Verify generated file name follows convention

#### Macro Generator

- [ ] Verify declarative macros show rules
- [ ] Verify procedural macros show attribute
- [ ] Verify macro type is indicated
- [ ] Verify generated file name follows convention

#### Static Item Generator

- [ ] Verify immutable static is shown correctly
- [ ] Verify mutable static is shown correctly
- [ ] Verify thread safety note is included
- [ ] Verify type is displayed
- [ ] Verify generated file name follows convention

#### Trait Alias Generator

- [ ] Verify aliased traits are shown
- [ ] Verify generic parameters are shown
- [ ] Verify bounds are displayed
- [ ] Verify generated file name follows convention

#### Extern Crate Generator

- [ ] Verify crate name is shown
- [ ] Verify rename is shown if present
- [ ] Verify generated file name follows convention

#### Use Item Generator

- [ ] Verify use path is shown in header
- [ ] Verify visibility is shown
- [ ] Verify generated file name follows convention

#### Primitive Type Generator

- [ ] Verify type name is shown
- [ ] Verify type category is indicated
- [ ] Verify generated file name follows convention

#### Extern Type Generator

- [ ] Verify type name is shown
- [ ] Verify FFI note is included
- [ ] Verify generated file name follows convention

### Integration Tests

- [ ] Verify build command generates all module files
- [ ] Verify build command generates all macro files
- [ ] Verify build command generates all static files
- [ ] Verify build command generates all other advanced item files
- [ ] Verify index links work to generated files
- [ ] Verify all 21 item types are handled

### Regression Tests

- [ ] Verify index page still works
- [ ] Verify basic type generators still work
- [ ] Verify trait and function generators still work
- [ ] Verify no compiler warnings
- [ ] Verify existing tests still pass

### Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── error.rs                # Centralized error definitions (from core infrastructure)
│   ├── markdown/
│   │   ├── mod.rs              # Updated with new exports
│   │   ├── index.rs            # Existing
│   │   ├── utils.rs            # Existing
│   │   ├── struct_.rs          # Existing
│   │   ├── enum_.rs            # Existing
│   │   ├── union.rs            # Existing
│   │   ├── alias.rs            # Existing
│   │   ├── trait_.rs           # Existing
│   │   ├── function.rs         # Existing
│   │   ├── impl_.rs            # Existing
│   │   ├── constant.rs         # Existing
│   │   ├── module.rs           # NEW: Module generator
│   │   ├── macro.rs            # NEW: Macro generator
│   │   ├── static_item.rs      # NEW: Static item generator
│   │   ├── trait_alias.rs      # NEW: Trait alias generator
│   │   ├── extern_crate.rs     # NEW: Extern crate generator
│   │   ├── use_item.rs         # NEW: Use item generator
│   │   ├── primitive.rs        # NEW: Primitive type generator
│   │   └── extern_type.rs      # NEW: Extern type generator
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
pub mod trait_;
pub mod function;
pub mod impl_;
pub mod constant;
pub mod module;
pub mod macro;
pub mod static_item;
pub mod trait_alias;
pub mod extern_crate;
pub mod use_item;
pub mod primitive;
pub mod extern_type;

// Re-export from error module for convenience
pub use crate::error::{Error, MarkdownError, Result};

// Re-export commonly used functions
pub use module::generate_module;
pub use macro::generate_macro;
pub use static_item::generate_static;
pub use trait_alias::generate_trait_alias;
pub use extern_crate::generate_extern_crate;
pub use use_item::generate_use;
pub use primitive::generate_primitive;
pub use extern_type::generate_extern_type;
```

### Example Generated Module Markdown

```markdown
# std::collections

Collection types and utilities.

## Contents

- 3 modules
- 12 structs
- 5 enums
- 8 functions

## Public Items

### Modules

- [std::collections::btree_map](std-collections-btree_map.md)
- [std::collections::btree_set](std-collections-btree_set.md)
- [std::collections::hash_map](std-collections-hash_map.md)

### Structs

- [std::collections::HashMap](std-collections-HashMap.md)
- [std::collections::HashSet](std-collections-HashSet.md)
- ...

## Next Actions

- View module source: `cargo docmd browse --item 0:3:1`
- Browse all modules: `cargo docmd browse --type module`
```

### Example Generated Macro Markdown

````markdown
# vec!

Creates a `Vec` containing the arguments.

**Type**: Declarative macro (macro_rules!)

## Definition

```rust
macro_rules! vec {
    ($($x:expr),*) => { ... };
}
```
````

## Documentation

This macro creates a new vector containing the provided arguments.

## Next Actions

- View source: `cargo docmd browse --item 0:3:20`
- Find related macros: `cargo docmd browse --type macro`

````

### Example Generated Static Item Markdown

```markdown
# CONFIG

Global configuration for the application.

**Type**: Immutable static

**Safety**: Accessing this static variable is thread-safe (it's immutable).

## Type

```rust
static CONFIG: Config
````

## Next Actions

- View source: `cargo docmd browse --item 0:3:21`
- Find related statics: `cargo docmd browse --type static`

````

### Example Generated Use Item Markdown

```markdown
# use std::collections::HashMap;

Import the HashMap type from std::collections.

**Visibility**: Private (not re-exported)

## Next Actions

- View source: `cargo docmd browse --item 0:3:25`
- Find related uses: `cargo docmd browse --type use`
````

## Design Considerations

### 1. Module Generator Complexity

**Decision**: Modules show summary of nested items with links.

- **Alternative**: Show full documentation for all nested items in module file.
    - Rejected: Would create very large files, defeats purpose of separate item
      pages
- **Alternative**: Don't show nested items at all.
    - Rejected: Modules should provide navigation overview
- **Resolution**: Show counts and links for navigation, detail in separate files

### 2. Macro Type Detection

**Decision**: Explicitly indicate macro type (declarative vs procedural).

- **Alternative**: Determine type from content alone.
    - Rejected: Not immediately obvious to users, explicit is better
- **Alternative**: Create separate generators for each macro type.
    - Rejected: Most logic is shared, separate files unnecessary
- **Resolution**: Single generator with type indicator

### 3. Static vs Constant

**Decision**: Static and constant generators are separate with different
messaging.

- **Alternative**: Combine into one generator.
    - Rejected: Static and constants have different semantics and lifetimes
- **Alternative**: Only generate constants, skip statics.
    - Rejected: Statics are important for understanding global state
- **Resolution**: Separate generators with appropriate messaging (thread safety
  for statics)

### 4. Module-Level Items

**Decision**: Generate separate files for extern crate, use, etc.

- **Alternative**: Skip these items in markdown generation.
    - Rejected: These items provide important context about imports
- **Alternative**: Group all module-level items in one file.
    - Rejected: Would create mismatch with other item types
- **Resolution**: Generate individual files for consistency

### 5. Primitive Types

**Decision**: Generate markdown for primitive types if documented.

- **Alternative**: Skip primitives (they're built-in).
    - Rejected: Primitives can have documentation in std
- **Alternative**: Only generate for std, skip for other crates.
    - Rejected: Inconsistent behavior, should handle uniformly
- **Resolution**: Generate if the item exists in rustdoc JSON

### 6. Extern Type

**Decision**: Generate for extern types with FFI context note.

- **Alternative**: Skip extern types (rare).
    - Rejected: Some crates use extern types for FFI
- **Alternative**: Treat like other type aliases.
    - Rejected: Extern types have different semantics
- **Resolution**: Generate with FFI context note

### 7. File Naming for Special Characters

**Decision**: Apply same hyphen replacement to all item IDs.

- **Alternative**: Handle special characters differently for different types.
    - Rejected: Inconsistent, harder to maintain
- **Alternative**: Use encoding for special characters.
    - Rejected: Unnecessary complexity
- **Resolution**: Consistent hyphen replacement across all types

## Success Criteria

- [ ] All eight advanced generators are implemented
- [ ] Module generator handles nested items correctly
- [ ] Macro generator handles both declarative and procedural macros
- [ ] Static generator handles both mutable and immutable
- [ ] All module-level items are handled
- [ ] All generated files use correct naming convention
- [ ] Build command generates files for all advanced item types
- [ ] Index page shows counts for all item types
- [ ] All 21 item types from rustdoc are handled
- [ ] Error messages include full paths on failure
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] No compiler warnings
- [ ] Documentation is complete and clear

## Implementation Status: 🟡 NOT STARTED

## Implementation Status

## Implementation Notes

Space for recording specific technical details or roadblocks encountered during
work.
