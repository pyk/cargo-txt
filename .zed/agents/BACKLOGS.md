# Backlog 1: cargo docmd build

There are few improvements that we need to do for `cargo docmd build`.

It is about the `cargo docmd build --crate <name>` command.

First, cargo docmd should validate the crate name.

## Requirement #1: Crate name should be valid dependencies.

The list of dependencies is available by executing the following command:

```sh
cargo metadata --no-deps --format-version 1
```

the example output is as follows:

```json
{
    "packages": [
        {
            "name": "docmd",
            "version": "0.1.0",
            "id": "path+file:///home/pyk/bidentxyz/cargo-docmd#docmd@0.1.0",
            "license": null,
            "license_file": null,
            "description": null,
            "source": null,
            "dependencies": [
                {
                    "name": "clap",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^4.5.54",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": ["derive"],
                    "target": null,
                    "registry": null
                },
                {
                    "name": "rustdoc-types",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^0.57.0",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                },
                {
                    "name": "serde",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^1.0.228",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                },
                {
                    "name": "serde_json",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^1.0.148",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                },
                {
                    "name": "tempfile",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^3.24.0",
                    "kind": "dev",
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                }
            ],
            "targets": [
                {
                    "kind": ["bin"],
                    "crate_types": ["bin"],
                    "name": "cargo-docmd",
                    "src_path": "/home/pyk/bidentxyz/cargo-docmd/src/main.rs",
                    "edition": "2024",
                    "doc": true,
                    "doctest": false,
                    "test": true
                }
            ],
            "features": {},
            "manifest_path": "/home/pyk/bidentxyz/cargo-docmd/Cargo.toml",
            "metadata": null,
            "publish": null,
            "authors": [],
            "categories": [],
            "keywords": [],
            "readme": "README.md",
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2024",
            "links": null,
            "default_run": null,
            "rust_version": null
        }
    ],
    "workspace_members": [
        "path+file:///home/pyk/bidentxyz/cargo-docmd#docmd@0.1.0"
    ],
    "workspace_default_members": [
        "path+file:///home/pyk/bidentxyz/cargo-docmd#docmd@0.1.0"
    ],
    "resolve": null,
    "target_directory": "/home/pyk/bidentxyz/cargo-docmd/target",
    "build_directory": "/home/pyk/bidentxyz/cargo-docmd/target",
    "version": 1,
    "workspace_root": "/home/pyk/bidentxyz/cargo-docmd",
    "metadata": null
}
```

the cargo docmd should parse this file and collect the valid dependencies name.

if crate name is not found then return error with user friendly message that the
crate name is not available and shows the available crate name.

## rustdoc generation

the rustdoc command accept `features` flag:

```
cargo rustdoc --help
...
Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
...
```

when running the rustdoc we should include the specified feature in the
`cargo metadata` output.

for example, `clap` crate. When the docmd is executed with:

```sh
cargo docmd build --crate clap
```

it should look at metadata which feature is enabled, then use it to build the
rustdoc json generation command.

## rustdoc file name

The current approach to determining the file json path is over-engineered.

we should follow the simple approach: if crate name have hypen `-`, we should
simply replace it with the underscore.

## cargo target dir

currently we use `CARGO_TARGET_DIR` env directory, if we already parse the cargo
metadata, we can simply reuse the `target_directory` field in the parsed
metadata.

## remove `--crate` from build command

we should simply use crate name as positional required argument instead of flag.
