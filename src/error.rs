//! Centralized error types for cargo-docmd.
//!
//! This module defines all error types used throughout the application,
//! providing consistent error handling and user-friendly error messages.

use std::fmt;
use std::path::PathBuf;

/// Top-level error type for cargo-docmd operations.
///
/// This enum wraps specific error types for different operations,
/// allowing for targeted error handling while maintaining a common
/// error type for the application.
#[derive(Debug)]
pub enum Error {
    /// Errors that occur during the build process
    Build(BuildError),
    /// Errors that occur during markdown generation
    Markdown(MarkdownError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Build(err) => write!(f, "{}", err),
            Error::Markdown(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for convenience.
///
/// Using this alias throughout the application simplifies error handling
/// and ensures consistent error types.
pub type Result<T> = std::result::Result<T, Error>;

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::Build(err)
    }
}

impl From<MarkdownError> for Error {
    fn from(err: MarkdownError) -> Self {
        Error::Markdown(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Markdown(MarkdownError::FileWriteFailed(
            PathBuf::from("<unknown>"),
            err.to_string(),
        ))
    }
}

/// Errors that occur during the build process.
///
/// These errors cover operations such as checking for the nightly toolchain,
/// executing cargo commands, parsing JSON output, and managing output directories.
#[derive(Debug)]
pub enum BuildError {
    /// Nightly toolchain is not installed
    NightlyNotInstalled,
    /// Cargo command execution failed
    CargoExecutionFailed { crate_name: String, output: String },
    /// Expected JSON file was not found
    JsonNotFound(PathBuf),
    /// JSON parsing failed
    JsonParseError { path: PathBuf, error: String },
    /// Failed to create output directory
    OutputDirCreationFailed { path: PathBuf, error: String },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::NightlyNotInstalled => {
                write!(
                    f,
                    "Nightly toolchain is not installed. Install it with: rustup install nightly"
                )
            }
            BuildError::CargoExecutionFailed { crate_name, output } => {
                write!(
                    f,
                    "Failed to execute cargo rustdoc for crate '{}':\n{}",
                    crate_name, output
                )
            }
            BuildError::JsonNotFound(path) => {
                write!(f, "Expected JSON file not found at '{}'", path.display())
            }
            BuildError::JsonParseError { path, error } => {
                write!(
                    f,
                    "Failed to parse JSON file '{}': {}",
                    path.display(),
                    error
                )
            }
            BuildError::OutputDirCreationFailed { path, error } => {
                write!(
                    f,
                    "Failed to create output directory '{}': {}",
                    path.display(),
                    error
                )
            }
        }
    }
}

impl std::error::Error for BuildError {}

/// Errors that occur during markdown generation.
///
/// These errors cover file and directory operations when creating
/// markdown documentation files.
#[derive(Debug)]
pub enum MarkdownError {
    /// Failed to write a markdown file
    FileWriteFailed(PathBuf, String),
    /// Failed to create a directory
    DirectoryCreationFailed(PathBuf, String),
    /// Expected item not found in the index
    ItemNotFound(String),
}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownError::FileWriteFailed(path, error) => {
                write!(f, "Failed to write file '{}': {}", path.display(), error)
            }
            MarkdownError::DirectoryCreationFailed(path, error) => {
                write!(
                    f,
                    "Failed to create directory '{}': {}",
                    path.display(),
                    error
                )
            }
            MarkdownError::ItemNotFound(id) => {
                write!(f, "Item '{}' not found in documentation index", id)
            }
        }
    }
}

impl std::error::Error for MarkdownError {}
