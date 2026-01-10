# Planning Mode

When creating or updating a plan:

1. Review README.md to understand the project.
2. Review DOCS.md (if exists) to understand the implementation.
3. Ensure plans for Rust files adhere to guidelines in
   `.zed/agent/guidelines/rust.md` strictly.
4. Ensure plans for DOCS.md updates adhere to guidelines in
   `.zed/agent/guidelines/docs.md` strictly.
5. Ensure plans for README.md updates adhere to guidelines in
   `.zed/agent/guidelines/readme.md` strictly.
6. Use the thinking tool.
7. Include README.md and DOCS.md updates in the plan.
8. Use `rust-lint`, `cargo clippy -- -D warnings`, `cargo build`, and
   `cargo test` as success criteria.
9. Follow instructions in `.zed/agent/instructions/create-plan.md`.

# Building Mode

When implementing a plan:

1. Update the plan status as in progress.
2. Review README.md to understand the project.
3. Review DOCS.md (if exists) to understand the implementation.
4. Follow guidelines in `.zed/agent/guidelines/rust.md` strictly when editing
   Rust files.
5. Follow guidelines in `.zed/agent/guidelines/docs.md` strictly when editing
   DOCS.md.
6. Follow guidelines in `.zed/agent/guidelines/readme.md` strictly when editing
   README.md.
7. Use the thinking tool.
8. Do not use git restore commands (can cause data loss).
9. Use `cargo test` to run tests and `rust-lint` to check coding conventions.
10. Review and update the plan checklist after implementation.
11. Use `cargo install --path .` before running `cargo txt`.

# Reviewing Mode

When reviewing staged changes:

1. Update the plan status as in progress.
2. Review README.md to understand the project.
3. Review DOCS.md (if exists) to understand the implementation.
4. Ensure Rust files adhere to guidelines in `.zed/agent/guidelines/rust.md`
   strictly.
5. Ensure DOCS.md updates adhere to guidelines in
   `.zed/agent/guidelines/docs.md` strictly.
6. Ensure README.md updates adhere to guidelines in
   `.zed/agent/guidelines/readme.md` strictly.
7. Use the thinking tool.
8. Follow the instructions in `.zed/agent/instructions/review-changes.md`.
9. Use `cargo test` to run tests and `rust-lint` to verify changes.

# Git Commit Mode

When writing Git Commit message:

1. Review README.md to understand the project.
2. Review DOCS.md (if exists) to understand the current implementation.
3. Use the thinking tool.
4. Follow instructions in
   `.zed/agent/instructions/create-git-commit-message.md`.
