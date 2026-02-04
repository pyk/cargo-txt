---
type: normal
title: Fix dev-dependency build panic
seq: 021
slug: fix-dev-dependency-build
created: "2026-02-04T08:38:24Z"
status: completed
---

# Fix dev-dependency build panic

This task fixes the cargo panic that occurs when trying to build documentation
for dev-dependencies like `serde_path_to_error`. The panic occurs because
dev-dependencies are not included in the `activated_features` list when cargo
builds documentation.

## Current Problems

When running `cargo txt build serde_path_to_error`, cargo panics with:

```
thread 'main' (962636) panicked at src/tools/cargo/src/cargo/core/resolver/features.rs:325:13:
did not find features for (PackageId { name: "serde_path_to_error", version: "0.1.20", source: "registry `crates-io`" }, NormalOrDev) within activated_features:
```

The problem is in the build command flow:

```rust
// In src/commands/build.rs
pub fn build(crate_name: &str) -> Result<()> {
    let cargo_metadata = cargo::metadata()?;
    validate_crate_name(crate_name, &cargo_metadata)?;  // ✅ Passes - serde_path_to_error IS in dependencies
    let cargo_doc_output_dir = cargo::doc(crate_name)?; // ❌ Panics - cargo can't find dev-dependency in activated_features
    // ...
}
```

The `validate_crate_name()` function checks if the crate exists in dependencies,
but it doesn't distinguish between regular dependencies and dev-dependencies.
Dev-dependencies are marked with `"kind":"dev"` in the metadata.

## Proposed Solution

1. Update the `Dependency` struct in `cargo.rs` to include the `kind` field
2. Modify `validate_crate_name()` to detect dev-dependencies and provide a
   helpful error message
3. Add enhanced debug logging to track the dependency type
4. Optionally, attempt alternative approaches for dev-dependencies

## Analysis Required

### Dependency Investigation

- [ ] Check the full Dependency struct from cargo metadata to see all available
      fields
- [ ] Verify if there's a way to build documentation for dev-dependencies using
      different cargo flags
- [ ] Test if removing `--no-deps` helps with dev-dependencies

### Code Locations to Check

- `src/cargo.rs` - Update Dependency struct and metadata parsing
- `src/commands/build.rs` - Update validate_crate_name to check dependency kind
- `src/cargo.rs` - Update doc() function to handle dev-dependencies gracefully

## Implementation Checklist

### Code Changes

- [x] Update `Dependency` struct in `src/cargo.rs` to include `kind` field:

    ```rust
    #[derive(Debug, Deserialize)]
    pub struct Dependency {
        pub name: String,
        pub kind: Option<String>,  // "dev" for dev-dependencies, null for regular dependencies
    }
    ```

- [x] Modify `validate_crate_name()` in `src/commands/build.rs` to detect
      dev-dependencies:

    ```rust
    fn validate_crate_name(crate_name: &str, cargo_metadata: &cargo::Metadata) -> Result<()> {
        let dependencies = &cargo_metadata.packages[0].dependencies;
        let dependency = dependencies.iter()
            .find(|dep| dep.name == crate_name);

        match dependency {
            Some(dep) => {
                if dep.kind.as_deref() == Some("dev") {
                    bail!(
                        concat!(
                            "Crate '{}' is a dev-dependency.\n",
                            "\n",
                            "Dev-dependencies cannot be built directly because they are not part of ",
                            "the regular dependency graph and cargo does not activate them for ",
                            "documentation generation.\n",
                            "\n",
                            "To build documentation for dev-dependencies, you can:\n",
                            "1. Move them to the [dependencies] section in Cargo.toml (temporary)\n",
                            "2. Use `cargo doc` in a temporary project with the crate as a regular dependency\n"
                        ),
                        crate_name
                    );
                }
                Ok(())
            },
            None => {
                let available_crates: Vec<&str> = dependencies.iter()
                    .map(|dep| dep.name.as_str())
                    .collect();
                bail!(
                    concat!(
                        "Crate '{}' is not an installed dependency.\n",
                        "\n",
                        "Available crates: {}\n",
                        "\n",
                        "Only installed dependencies can be built. ",
                        "Add the crate to Cargo.toml as a dependency first."
                    ),
                    crate_name,
                    available_crates.join(", ")
                )
            }
        }
    }
    ```

- [x] Add enhanced debug logging in `validate_crate_name()`:

    ```rust
    if let Some(dep) = dependency {
        debug!(
            "Found dependency '{}' with kind: {:?}",
            crate_name,
            dep.kind
        );
    }
    ```

- [x] Improve error handling in `cargo::doc()` to better capture dev-dependency
      panics:
    - Add debug logging before running cargo doc
    - Log the full stderr output including panic messages
    - Provide context about the dependency type in error messages

- [x] Add unit tests for dev-dependency detection:
    - Test that dev-dependencies are detected and return appropriate error
    - Test that regular dependencies pass validation
    - Test that missing dependencies return error

### Documentation Updates

- [x] Update README.md to document the limitation about dev-dependencies
    - Add a note in the Build Command section explaining that only regular
      dependencies can be built documentation
    - Provide workarounds for users who need to build dev-dependency
      documentation

### Test Plan

- [x] Run `cargo txt build serde_path_to_error` and verify it returns a helpful
      error message instead of panicking
- [x] Run `cargo txt build serde` and verify it still works correctly
- [x] Verify the error message includes guidance on how to work around the
      limitation

### Regression Tests

- [x] Run existing tests: `cargo test`
- [x] Verify that building documentation for regular dependencies still works
- [x] Check that the validation logic correctly identifies all dependency types

## Structure After Changes

### File Structure

```
src/
├── cargo.rs                 # Updated with Dependency.kind field
├── commands/
│   ├── build.rs             # Updated with dev-dependency detection
│   └── ...
└── ...
```

### Module Exports

No changes to module exports.

## Design Considerations

1. **Why detect dev-dependencies instead of trying to build them?**
    - **Alternative**: Try different cargo doc flags like removing `--no-deps`
    - **Resolution**: Tested this and cargo still panics. The issue is in
      cargo's feature resolution, not in the flags used. Dev-dependencies are
      not activated during doc generation.

2. **Should we provide a workaround or just an error message?**
    - **Alternative**: Automatically create a temporary project and build docs
      there
    - **Resolution**: Too complex and error-prone. Better to provide clear error
      messages with instructions.

3. **Why check in validate_crate_name instead of cargo::doc?**
    - **Alternative**: Check in cargo::doc() when running the command
    - **Resolution**: Better to fail fast during validation with a clear error
      message, rather than attempting cargo doc and catching the panic.

## Success Criteria

- `cargo txt build serde_path_to_error` returns a helpful error message
  explaining dev-dependencies cannot be built
- Error message includes workarounds for the user
- No cargo panics occur
- Building documentation for regular dependencies still works correctly
- `cargo test` passes
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- Debug logging provides clear information about dependency types

## Implementation Notes

- The panic occurs in cargo's internal features.rs at line 325, which is out of
  our control
- The best approach is to detect dev-dependencies early and prevent the panic
- Error messages should be clear and actionable
- Consider adding a flag like `--force` in the future to attempt workarounds for
  dev-dependencies
