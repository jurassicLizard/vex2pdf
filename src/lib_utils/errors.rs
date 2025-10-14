//! This module contains all crate related error handling logic
use std::fmt::{Display, Formatter};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
/// This enum encapsulates all errors that might be thrown in this crate
#[derive(Debug)]
pub enum Vex2PdfError {
    /// Good old IO errors
    Io(io::Error),
    /// Invalid output path. This usually triggers when a file is given instead of a path for the output dir parameter
    InvalidOutputDir(PathBuf),
    /// Passed a file where we could not extract the stem from the file name
    InvalidFileStem(PathBuf),
    /// Bom parsing errors
    Parse(String),
    /// Unsupported file type error
    UnsupportedFileType,
    /// File was ignored explicitly through user input
    IgnoredByUser,
    /// Concurrency error
    ConcurrencyError(String),
}

impl Display for Vex2PdfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vex2PdfError::Io(e) => e.fmt(f),
            Vex2PdfError::InvalidOutputDir(path) => {
                write!(f, "`{}` should be a directory but isn't", path.display())
            }
            Vex2PdfError::InvalidFileStem(path) => write!(
                f,
                "failed to extract filename stem from `{}`",
                path.display()
            ),
            Vex2PdfError::Parse(message) => write!(f, "{}", message),
            Vex2PdfError::UnsupportedFileType => write!(f, "Unsupported file type for parsing"),
            Vex2PdfError::IgnoredByUser => write!(f, "file ignored explicitly by user"),
            Vex2PdfError::ConcurrencyError(s) => write!(f, "Concurrency error : {s}"),
        }
    }
}

impl std::error::Error for Vex2PdfError {}

impl From<io::Error> for Vex2PdfError {
    fn from(value: io::Error) -> Self {
        Vex2PdfError::Io(value)
    }
}

impl<T> From<mpsc::SendError<T>> for Vex2PdfError {
    fn from(value: mpsc::SendError<T>) -> Self {
        Vex2PdfError::ConcurrencyError(format!(
            "Attempted to send where there are no more receivers. {}",
            value.to_string()
        ))
    }
}
