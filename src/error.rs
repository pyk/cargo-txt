//! Centralized error types for cargo-docmd.
//!
//! This module defines all error types used throughout the application,
//! providing consistent error handling and user-friendly error messages.

use std::fmt;
use std::path::PathBuf;

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
pub enum Error {
    /// Errors that occur during the build process
    Build(BuildError),
    /// Errors that occur during the open process
    Open(OpenError),
    /// CSS selector failed to parse
    HtmlSelectorParseFailed { selector: String, error: String },
    /// Required HTML element not found
    HtmlElementNotFound { selector: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Build(err) => write!(f, "{}", err),
            Error::Open(err) => write!(f, "{}", err),
            Error::HtmlSelectorParseFailed { selector, error } => {
                write!(f, "Failed to parse selector '{}': {}", selector, error)
            }
            Error::HtmlElementNotFound { selector } => {
                write!(f, "Element not found with selector '{}'", selector)
            }
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::Build(err)
    }
}

impl From<OpenError> for Error {
    fn from(err: OpenError) -> Self {
        Error::Open(err)
    }
}

/// Errors that occur during the build process.
///
/// These errors cover all build operations including cargo command execution,
/// HTML parsing, markdown generation, and file system operations.
pub enum BuildError {
    /// Cargo command execution failed
    CargoDocExecFailed { crate_name: String, output: String },
    /// Failed to execute cargo metadata command
    CargoMetadataExecFailed { output: String },
    /// Requested crate name is not an installed dependency
    InvalidCrateName {
        requested: String,
        available: Vec<String>,
    },
    /// Failed to create output directory
    OutputDirCreationFailed {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Failed to read file
    FileReadFailed {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Failed to parse cargo doc output to find the generated directory
    CargoDocOutputParseFailed { output_preview: String },
    /// Documentation index file (all.html) not found
    DocIndexNotFound {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to write markdown file
    MarkdownWriteFailed {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::CargoDocExecFailed { crate_name, output } => {
                write!(
                    f,
                    "Failed to execute cargo doc for crate '{}':\n{}",
                    crate_name, output
                )
            }
            BuildError::CargoMetadataExecFailed { output } => {
                write!(f, "Failed to execute cargo metadata command:\n{}", output)
            }
            BuildError::InvalidCrateName {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Crate '{}' is not an installed dependency.\n\nAvailable crates: {}\n\nOnly installed dependencies can be built. Add the crate to Cargo.toml as a dependency first.",
                    requested,
                    available.join(", ")
                )
            }
            BuildError::OutputDirCreationFailed { path, source } => {
                write!(
                    f,
                    "Failed to create output directory '{}': {}",
                    path.display(),
                    source
                )
            }
            BuildError::FileReadFailed { path, source } => {
                write!(f, "Failed to read file '{}': {}", path.display(), source)
            }

            BuildError::CargoDocOutputParseFailed { output_preview } => {
                write!(
                    f,
                    "Failed to parse cargo doc output - could not find 'Generated' line.\nOutput preview:\n{}",
                    output_preview
                )
            }
            BuildError::DocIndexNotFound { path, source } => {
                write!(
                    f,
                    "Documentation index file '{}' not found: {}",
                    path.display(),
                    source
                )
            }
            BuildError::MarkdownWriteFailed { path, source } => {
                write!(
                    f,
                    "Failed to write markdown file '{}': {}",
                    path.display(),
                    source
                )
            }
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildError::DocIndexNotFound { source, .. } => Some(source.as_ref()),

            BuildError::FileReadFailed { source, .. } => Some(source.as_ref()),
            BuildError::OutputDirCreationFailed { source, .. } => Some(source.as_ref()),
            BuildError::MarkdownWriteFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl fmt::Debug for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Errors that occur during the open command.
///
/// These errors cover path resolution, file reading, and item lookup
/// operations for displaying documentation.
pub enum OpenError {
    /// Documentation index file (all.html) not found
    DocIndexNotFound {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Invalid item path format
    InvalidItemPath { item_path: String },
    /// Markdown file not found
    MarkdownNotFound {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Failed to resolve item path to markdown file
    ItemPathResolutionFailed {
        item_path: String,
        attempted_paths: Vec<PathBuf>,
    },
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenError::DocIndexNotFound { path, source } => {
                write!(
                    f,
                    "Documentation index file '{}' not found: {}",
                    path.display(),
                    source
                )
            }
            OpenError::InvalidItemPath { item_path } => {
                write!(
                    f,
                    "Invalid item path '{}'. Expected format: <crate> or <crate>::<item> (e.g., 'serde' or 'serde::Error').",
                    item_path
                )
            }
            OpenError::MarkdownNotFound { path, source } => {
                write!(
                    f,
                    "Markdown file '{}' not found: {}",
                    path.display(),
                    source
                )
            }
            OpenError::ItemPathResolutionFailed {
                item_path,
                attempted_paths,
            } => {
                write!(
                    f,
                    "Could not resolve item path '{}'.\n\nAttempted paths:\n{}",
                    item_path,
                    attempted_paths
                        .iter()
                        .map(|p| format!("  - {}", p.display()))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }
}

impl std::error::Error for OpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OpenError::DocIndexNotFound { source, .. } => Some(source.as_ref()),
            OpenError::MarkdownNotFound { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl fmt::Debug for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
