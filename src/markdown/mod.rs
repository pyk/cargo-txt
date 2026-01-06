//! Markdown generation framework for rustdoc JSON output.
//!
//! This module provides infrastructure for generating markdown files from
//! rustdoc JSON, optimized for consumption by coding agents. The framework
//! handles file naming conventions, common markdown formatting utilities, and
//! generates an index page listing all public items.

pub mod index;
pub mod utils;

// Will add more generator modules later
// pub mod module;
// pub mod struct_;
// pub mod enum_;
// etc.

/// Standard header level for item titles
pub const ITEM_HEADER_LEVEL: usize = 1;

/// Standard header level for item sections
pub const SECTION_HEADER_LEVEL: usize = 2;
