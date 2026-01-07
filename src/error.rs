//! Centralized error types for cargo-docmd.
//!
//! This module defines all error types used throughout the application,
//! providing consistent error handling and user-friendly error messages.

use std::fmt;
use std::path::{Path, PathBuf};

/// Result type alias for convenience.
///
/// Using this alias throughout the application simplifies error handling
/// and ensures consistent error types.
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level error type for cargo-docmd operations.
///
/// This enum wraps specific error types for different operations,
/// allowing for targeted error handling while maintaining a common
/// error type for the application.
#[derive(Debug)]
pub enum Error {
    /// Errors that occur during the build process
    Build(BuildError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Build(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::Build(err)
    }
}

impl From<HtmlExtractError> for Error {
    fn from(err: HtmlExtractError) -> Self {
        Error::Build(err.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Build(BuildError::markdown_write_failed(
            &PathBuf::from("<unknown>"),
            err.to_string(),
        ))
    }
}

/// Errors that occur during HTML extraction operations.
///
/// These errors cover low-level operations when extracting data from HTML,
/// such as selector parsing failures or missing elements. These errors do
/// not contain file paths - paths are added by the caller when wrapping
/// these errors in BuildError::HtmlParseFailed.
#[derive(Debug)]
pub enum HtmlExtractError {
    /// CSS selector failed to parse
    SelectorParseFailed { selector: String, error: String },
    /// Required HTML element not found
    ElementNotFound { selector: String },
}

impl fmt::Display for HtmlExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlExtractError::SelectorParseFailed { selector, error } => {
                write!(f, "Failed to parse selector '{}': {}", selector, error)
            }
            HtmlExtractError::ElementNotFound { selector } => {
                write!(f, "Element not found with selector '{}'", selector)
            }
        }
    }
}

impl std::error::Error for HtmlExtractError {}

/// Errors that occur during the build process.
///
/// These errors cover all build operations including cargo command execution,
/// HTML parsing, markdown generation, and file system operations.
#[derive(Debug)]
pub enum BuildError {
    /// Cargo command execution failed
    CargoExecutionFailed { crate_name: String, output: String },
    /// Failed to create output directory
    OutputDirCreationFailed { path: PathBuf, error: String },
    /// HTML parsing failed
    HtmlParseFailed {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Documentation was not generated for the crate
    DocNotGenerated {
        crate_name: String,
        expected_path: PathBuf,
    },
    /// Failed to write markdown file
    MarkdownWriteFailed { path: PathBuf, error: String },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::CargoExecutionFailed { crate_name, output } => {
                write!(
                    f,
                    "Failed to execute cargo doc for crate '{}':\n{}",
                    crate_name, output
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
            BuildError::HtmlParseFailed { path, source } => {
                write!(
                    f,
                    "Failed to parse HTML file '{}': {}",
                    path.display(),
                    source
                )
            }
            BuildError::DocNotGenerated {
                crate_name,
                expected_path,
            } => {
                write!(
                    f,
                    "Documentation was not generated for crate '{}'. Expected directory at '{}'",
                    crate_name,
                    expected_path.display()
                )
            }
            BuildError::MarkdownWriteFailed { path, error } => {
                write!(
                    f,
                    "Failed to write markdown file '{}': {}",
                    path.display(),
                    error
                )
            }
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildError::HtmlParseFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<HtmlExtractError> for BuildError {
    fn from(err: HtmlExtractError) -> Self {
        BuildError::HtmlParseFailed {
            path: PathBuf::from("<unknown>"),
            source: Box::new(err),
        }
    }
}

impl BuildError {
    /// Creates a new MarkdownWriteFailed error from a path and error message.
    pub fn markdown_write_failed(path: &Path, error: String) -> Self {
        BuildError::MarkdownWriteFailed {
            path: path.to_path_buf(),
            error,
        }
    }
}

/// Wrap a result with a path error context.
///
/// This helper function wraps any error into BuildError::HtmlParseFailed,
/// adding the file path information for better error reporting.
pub fn wrap_with_path<T, E>(result: std::result::Result<T, E>, path: &Path) -> Result<T>
where
    E: std::error::Error + Send + Sync + 'static,
{
    result.map_err(|error| {
        BuildError::HtmlParseFailed {
            path: path.to_path_buf(),
            source: Box::new(error),
        }
        .into()
    })
}
