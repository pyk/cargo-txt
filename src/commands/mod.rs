//! Command implementations for cargo-docmd.
//!
//! This module contains all subcommand implementations, each in its own module.
//! Commands are organized by functionality and can be called directly from main.

pub mod browse;
pub mod build;

pub use browse::browse;
pub use build::build;
