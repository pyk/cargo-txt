# Planning Mode

When creating a plan:

1. Review `README.md` to understand the project.
2. Review `DOCS.md` (if exists) to understand the implementation.
3. Follow instructions in `.zed/agent/instructions/create-plan.md`.
4. Use the thinking tool.
5. Include README.md and DOCS.md updates in the plan.
6. Review existing plans for reference.

# Building Mode

When implementing a plan:

1. Review `README.md` to understand the project.
2. Review `DOCS.md` (if exists) to understand the implementation.
3. Use the thinking tool.
4. Follow guidelines in `.zed/agent/guidelines/` for files being edited:
    - `writing-rust.md` for Rust files
    - `writing-readme.md` for README.md
    - `writing-docs.md` for DOCS.md
5. Do not use git commands (can cause data loss).
6. Use `cargo test` to run tests and `rust-check` after changes.
