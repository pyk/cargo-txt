//! Markdown generation framework for rustdoc JSON output.
//!
//! This module provides infrastructure for generating markdown files from
//! rustdoc JSON, optimized for consumption by coding agents. The framework
//! handles file naming conventions, common markdown formatting utilities, and
//! generates an index page listing all public items.

pub mod r#enum;
pub mod index;
pub mod r#struct;
pub mod type_alias;
pub mod union;
pub mod utils;

/// Standard header level for item titles
pub const ITEM_HEADER_LEVEL: usize = 1;

/// Standard header level for item sections
pub const SECTION_HEADER_LEVEL: usize = 2;
