---
type: normal
title: "Support Cargo Feature Flags"
seq: 007
slug: "support-feature-flags"
created: "2026-01-07T10:45:00Z"
status: completed
---

# Support Cargo Feature Flags

Automatically detect and use cargo feature flags from `Cargo.toml` when building
documentation. The tool reads the enabled features from the dependency metadata
and passes them to `cargo doc`, ensuring documentation matches the project's
actual feature configuration.

## Current Problems

The `cargo docmd build` command runs `cargo doc` without feature flags, even
when dependencies are specified with particular features in `Cargo.toml`. This
results in documentation that doesn't reflect the actual feature configuration
used in the project.

```toml
# Example Cargo.toml
[dependencies]
clap = { version = "4.5.54", features = ["derive"] }
```

When building documentation for `clap`:

```rust
// Current cargo::Dependency struct in src/cargo.rs
#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
}
```

Issues:

- Dependency struct only tracks the crate name, not enabled features
- `cargo doc` is invoked without feature flags, generating docs with default
  features only
- Documentation may include APIs that aren't available when using the crate with
  specific features
- Documentation may exclude APIs that are available due to enabled features

## Proposed Solution

1. Extend the `Dependency` struct to include `features` and
   `uses_default_features` fields
2. Update the `build()` command to extract feature information from the
   dependency metadata
3. Update the `cargo::doc()` function to accept and pass feature flags to cargo
4. No CLI changes needed - feature detection is automatic based on Cargo.toml

## Analysis Required

### Dependency Investigation

- [x] Verify the exact JSON structure of dependency features in cargo metadata
      output
- [x] Confirm cargo doc accepts the same feature flag syntax

### Code Locations to Check

- [x] `src/cargo.rs` - Update `Dependency` struct and `doc()` function
- [x] `src/commands/build.rs` - Extract features from dependency and pass to
      `doc()`
- [x] `README.md` - Document automatic feature detection
- [x] `DOCS.md` - Update build command documentation

## Implementation Checklist

### Code Changes

#### Cargo Module Updates

- [x] Update `Dependency` struct to include feature information:
    ```rust
    #[derive(Debug, Deserialize)]
    pub struct Dependency {
        pub name: String,
        pub features: Vec<String>,
        pub uses_default_features: bool,
    }
    ```
- [x] Update `doc()` function signature to accept feature parameters:
    ```rust
    pub fn doc(
        crate_name: &str,
        target_dir: &str,
        features: &[&str],
        use_default_features: bool,
    ) -> error::Result<()>
    ```
- [x] Update `doc()` implementation to construct and pass feature flags:
    - If `use_default_features` is false, add `--no-default-features`
    - If `features` is not empty, add `--features` with comma-separated values
- [x] Add helper function to construct feature flags string if needed

#### Build Command Updates

- [x] Update `build()` function to find the specific dependency:
    ```rust
    let dependency = metadata.packages[0]
        .dependencies
        .iter()
        .find(|dep| dep.name == crate_name)
        .ok_or_else(|| error::BuildError::InvalidCrateName { ... })?;
    ```
- [x] Extract feature information from the dependency
- [x] Pass `dependency.features` and `dependency.uses_default_features` to
      `cargo::doc()`
- [x] Add log message showing which features are being used (optional)

#### Error Handling

- [x] No new error types needed - existing `InvalidCrateName` covers dependency
      not found
- [x] Ensure empty feature arrays are handled correctly (no `--features` flag
      needed)

### Documentation Updates

- [x] Update README.md to explain automatic feature detection:

    ````markdown
    ## Feature Detection

    cargo-docmd automatically detects which features are enabled for a crate
    from your Cargo.toml. When you run `cargo docmd build <crate>`, the tool:

    1. Reads your Cargo.toml via cargo metadata
    2. Extracts the enabled features for the specified crate
    3. Passes those features to cargo doc when generating documentation

    Example:

    ```toml
    [dependencies]
    clap = { version = "4.5", features = ["derive"] }
    ```
    ````

    Running `cargo docmd build clap` will automatically use the `derive` feature
    when generating documentation.

    ```

    ```

- [x] Update DOCS.md "What It Does" subsection to mention feature extraction
- [x] Add note about feature detection behavior in DOCS.md "build" command
      section

## Test Plan

### Verification Tests

- [x] Build documentation for clap with `derive` feature enabled in Cargo.toml
- [x] Verify `cargo doc` is called with `--features derive`
- [x] Build documentation for crate without features (default features only)
- [x] Verify `cargo doc` is called without `--features` flag
- [x] Build documentation for crate with `default-features = false`
- [x] Verify `cargo doc` is called with `--no-default-features`
- [x] Build documentation for crate with multiple features
- [x] Verify `cargo doc` is called with comma-separated features

### Regression Tests

- [x] Ensure existing build functionality still works
- [x] Ensure all existing tests pass
- [x] Ensure `cargo clippy` produces no warnings
- [x] Verify documentation generation still produces correct output

## Structure After Changes

### File Structure

```
cargo-docmd/
├── src/
│   ├── cargo.rs             # Updated Dependency struct and doc() function
│   ├── commands/
│   │   └── build.rs         # Updated to extract and pass features
│   └── ...
└── .zed/agent/plans/
    └── 007-support-feature-flags.md
```

### Dependency Struct

```rust
// src/cargo.rs

/// Dependency information for a package.
#[derive(Debug, Deserialize)]
pub struct Dependency {
    /// Name of the dependency crate
    pub name: String,

    /// Features enabled for this dependency
    pub features: Vec<String>,

    /// Whether default features are enabled
    pub uses_default_features: bool,
}
```

### Updated doc() Function

```rust
// src/cargo.rs

/// Generate HTML documentation for a specific crate.
///
/// This function executes `cargo doc --package <crate> --no-deps` with the
/// specified feature flags, then validates that the output directory exists.
pub fn doc(
    crate_name: &str,
    target_dir: &str,
    features: &[&str],
    use_default_features: bool,
) -> error::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["doc", "--package", crate_name, "--no-deps"]);

    if !use_default_features {
        cmd.arg("--no-default-features");
    }

    if !features.is_empty() {
        cmd.arg("--features")
           .arg(features.join(","));
    }

    let output = cmd
        .output()
        .map_err(|e| error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: e.to_string(),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(error::BuildError::CargoDocExecFailed {
            crate_name: crate_name.to_string(),
            output: stderr,
        }
        .into());
    }

    let doc_dir = std::path::PathBuf::from(target_dir)
        .join("doc")
        .join(crate_name);

    if !doc_dir.exists() {
        return Err(error::BuildError::DocNotGenerated {
            crate_name: crate_name.to_string(),
            expected_path: doc_dir,
        }
        .into());
    }

    Ok(())
}
```

### Updated build() Function

```rust
// src/commands/build.rs

/// Build markdown documentation from rustdoc HTML.
///
/// This function takes a crate name, generates HTML documentation using cargo doc
/// with the features enabled in Cargo.toml, parses the HTML files for type aliases,
/// and generates markdown documentation.
pub fn build(crate_name: String) -> error::Result<()> {
    // Get cargo metadata
    let metadata = cargo::metadata()?;

    // Find the dependency for the requested crate
    let dependency = metadata
        .packages[0]
        .dependencies
        .iter()
        .find(|dep| dep.name == crate_name)
        .ok_or_else(|| {
            error::BuildError::InvalidCrateName {
                requested: crate_name.clone(),
                available: metadata
                    .packages[0]
                    .dependencies
                    .iter()
                    .map(|dep| dep.name.clone())
                    .collect(),
            }
        })?;

    // Log which features are being used
    if !dependency.features.is_empty() {
        println!(
            "Building documentation for {} with features: {}",
            crate_name,
            dependency.features.join(", ")
        );
    }

    // Generate HTML documentation with dependency's feature configuration
    cargo::doc(
        &crate_name,
        &metadata.target_directory,
        &dependency.features,
        dependency.uses_default_features,
    )?;

    let html_dir = get_html_dir(&crate_name, &metadata.target_directory)?;
    let output_dir = get_output_dir(&metadata.target_directory)?;

    create_output_directory(&output_dir)?;

    let type_alias_count = parse_html_directory(&html_dir, &output_dir)?;

    if type_alias_count > 0 {
        println!(
            "\nGenerated markdown documentation for {} type alias(es)",
            type_alias_count
        );
    } else {
        println!("\nNo type aliases found in documentation");
    }

    println!("Output directory: {}", output_dir.display());

    Ok(())
}
```

## Design Considerations

1. **Automatic vs Manual Feature Control**:
    - **Alternative**: Add CLI flags to override features from Cargo.toml
    - **Resolution**: Automatic detection is simpler and matches user
      expectations. The tool should build documentation matching the actual
      project configuration. If users want different features, they should
      modify Cargo.toml.

2. **Feature Flag Construction**:
    - **Alternative**: Pass features as individual `--features <feature>`
      arguments
    - **Resolution**: Pass as comma-separated single argument
      (`--features feat1,feat2`) to match cargo's documented behavior and keep
      command construction simple.

3. **Default Features Handling**:
    - **Alternative**: Always pass `--default-features` when
      `uses_default_features` is true
    - **Resolution**: Only pass `--no-default-features` when false. This is
      cargo's default behavior and keeps the command line shorter.

4. **Logging Behavior**:
    - **Alternative**: Always log feature information (even when empty)
    - **Resolution**: Only log when non-empty features are present to reduce
      noise for common case.

5. **Dependency Finding Logic**:
    - **Alternative**: Support multiple packages in metadata (for workspace
      projects)
    - **Resolution**: For now, use `metadata.packages[0]` which assumes single
      package. This matches current behavior. Workspace support can be added
      later if needed.

6. **Backward Compatibility**:
    - **Alternative**: Add new fields as optional in case older cargo versions
      don't include them
    - **Resolution**: The `features` and `uses_default_features` fields have
      been in cargo metadata output for many versions. Default values (empty
      vec, true) handle cases where they might be missing.

## Success Criteria

- [x] Building documentation for a crate with `features = ["derive"]` in
      Cargo.toml passes `--features derive` to cargo doc
- [x] Building documentation for a crate with `default-features = false` in
      Cargo.toml passes `--no-default-features` to cargo doc
- [x] Building documentation for a crate with multiple features passes all
      features comma-separated
- [x] Building documentation for a crate with no features specified passes no
      feature flags
- [x] All existing tests pass
- [x] No new `cargo clippy` warnings
- [x] Documentation is updated to explain automatic feature detection
- [x] README includes example showing the feature detection behavior

## Implementation Status: 🟢 COMPLETED

## Implementation Notes

- The feature information is already present in cargo metadata output - we just
  need to capture it
- No changes to the CLI are needed - this is purely internal implementation
- The implementation should be backward compatible with projects that don't
  specify features
- Consider adding verbose logging to show the exact cargo command being executed
  for debugging
- Future enhancements could include workspace support and feature override flags
  if needed
