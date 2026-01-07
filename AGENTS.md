# Planning Mode

When creating or updating a plan:

1. Review `README.md` to understand the project.
2. Review `DOCS.md` (if exists) to understand the implementation.
3. Follow instructions in `.zed/agent/instructions/create-plan.md`.
4. Use the thinking tool.
5. Include README.md and DOCS.md updates in the plan.
6. Review existing plans for reference.

# Building Mode

When implementing a plan:

1. Update the plan status as in progress.
2. Review `README.md` to understand the project.
3. Review `DOCS.md` (if exists) to understand the implementation.
4. Use the thinking tool.
5. Follow guidelines in `.zed/agent/guidelines/` for files being edited:
    - `writing-rust.md` for Rust files
    - `writing-readme.md` for README.md
    - `writing-docs.md` for DOCS.md
    - **CRITICAL**: All files in `.zed/agent/guidelines/` should read complete
      full content, not just the outline.
6. Do not use git restore commands (can cause data loss).
7. Use `cargo test` to run tests and `rust-check` after changes.
8. Review and update the plan checklist after implementation.

# Reviewing Mode

When reviewing staged changes:

1. Update the plan status as in progress.
2. Review `README.md` to understand the project.
3. Review `DOCS.md` (if exists) to understand the implementation.
4. Use the thinking tool.
5. Follow the instructions in `.zed/agent/instructions/review-changes.md`.
6. Use `cargo test` to run tests and `rust-check` to verify changes.
7. Review the edited files:
    - DOCS.md must strictly follow the guideline defined in
      `.zed/agent/guidelines/writing-docs.md`.
    - README.md must strictly follow the guideline defined in
      `.zed/agent/guidelines/writing-readme.md`.
    - Rust files must strictly follow the guideline defined in
      `.zed/agent/guidelines/writing-rust.md`.
    - **CRITICAL**: All files in `.zed/agent/guidelines/` should read complete
      full content, not just the outline.

# Git Commit Mode

When writing Git Commit message:

1. Review `README.md` to understand the project.
2. Review `DOCS.md` (if exists) to understand the current implementation.
3. Use the thinking tool.
4. Follow instructions in
   `.zed/agent/instructions/create-git-commit-message.md`.
