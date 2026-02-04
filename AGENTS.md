# Tools

## cargo-txt

Use `cargo txt` to access the crate documentation.

The workflow is:

1. Build documentation: `cargo txt build <crate>`
2. List all items: `cargo txt list <lib_name>`
3. View specific item: `cargo txt show <lib_name>::<item>`

For example:

```shell
# Build the serde crate documentation
cargo txt build serde

# List all items in serde
cargo txt list serde

# View serde crate overview
cargo txt show serde

# View serde::Deserialize trait documentation
cargo txt show serde::Deserialize
```

# Guidelines

- [README Writing Guidelines](.zed/agent/guidelines/readme.md)
- [Rust Coding Guidelines: Documentation](.zed/agent/guidelines/rust.md)

# Instructions

- [Create Plan Intruction](.zed/agent/instructions/create-plan.md)
- [Review Changes Instruction](.zed/agent/instructions/review-changes.md)
- [Create Git Commit Message Instruction](.zed/agent/instructions/create-git-commit-message.md)

# Agent Modes

## Planning Mode

When creating or updating a plan:

1. Read README.md to understand the project.
2. Use the guidelines.
3. Use the thinking tool.
4. Use `cargo-txt` if needed.
5. Include README.md updates in the plan.
6. **IMPORTANT**: Every plan's Success Criteria section MUST include these base
   criteria:
    - `rust-lint` passes
    - `cargo clippy -- -D warnings` passes
    - `cargo build` succeeds
    - `cargo test` passes
7. Read & follow **Create Plan Instruction**.

## Building Mode

When implementing a plan:

1. Update the plan status as in progress.
2. Read README.md to understand the project.
3. Use the guidelines.
4. Use the `cargo-txt` tool.
5. Use the thinking tool.
6. **IMPORTANT**: Do not use git restore commands (can cause data loss).
7. **IMPORTANT**: Review and update the plan checklist after implementation.

## Review Mode

When reviewing staged changes:

1. Read README.md to understand the project.
2. Use the guidelines.
3. Use the thinking tool.
4. Read & follow **Review Changes Instruction**.
5. Use `cargo test` to run tests and `rust-lint` to verify changes.

## Git Commit Mode

When writing Git Commit message:

1. Read README.md to understand the project.
2. Use the guidelines.
3. Use the thinking tool.
4. Read & follow **Create Git Commit Message Instruction**.
