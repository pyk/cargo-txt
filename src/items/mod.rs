//! Item parsing modules for rustdoc HTML documentation.
//!
//! This module contains parsers for different Rust item types extracted from
//! rustdoc HTML files. Currently only type aliases are supported, with plans
//! to add support for structs, enums, and unions in future phases.

pub mod type_alias;
