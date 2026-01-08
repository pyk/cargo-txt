//! Command implementations for cargo-docmd.
//!
//! This module contains all subcommand implementations, each in its own module.
//! Commands are organized by functionality and can be called directly from main.

pub mod build;
pub mod open;

pub use build::build;
pub use open::open;
