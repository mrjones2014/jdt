use std::{fmt::Display, path::PathBuf, string::FromUtf8Error};

use image_repo::types::ChecksumError;
use reqwest::{StatusCode, Url};

/// Errors that can occur dealing with syncing local storage
#[derive(Debug)]
pub enum Error {
    /// Failed to get OS-specific storage directory.
    FailedToGetStorageDir,
    /// Failed to serialize repository JSON
    Serialization(serde_json::Error),
    /// HTTP request failed
    Http(reqwest::Error),
    /// HTTP request returned a bad status code
    HttpStatus(StatusCode),
    /// Downloaded image does not match the checksum from the repository JSON
    InvalidChecksum(ChecksumError),
    /// Failed to decode image bytes
    FailedToDecodeImageBytes(std::io::Error),
    /// Failed to decode JSON string from bytes
    FailedToDecodeUtf8(FromUtf8Error),
    /// File already exists and you specified `overwrite = false`
    FileAlreadyExists(PathBuf),
    /// Local resource did not exist
    FileNotFound(PathBuf),
    /// Failed to process file metadata
    FileMetadataFailed,
    /// Update URL field of JSON is different from the URL the file was downlaoded from.
    InvalidUpdateUrl((String, String)),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::FailedToGetStorageDir => "Failed to get local cache directory.".into(),
                Error::Serialization(e) => format!("Failed to serialize: {e}"),
                Error::Http(e) => format!("HTTP failed: {e}"),
                Error::HttpStatus(code) => format!("Bad HTTP status code: {code}"),
                Error::InvalidChecksum(e) => format!("{e}"),
                Error::FailedToDecodeImageBytes(e) => format!("Failed to decode image bytes: {e}"),
                Error::FailedToDecodeUtf8(e) => format!("Failed to decode UTF-8: {e}"),
                Error::FileAlreadyExists(path) =>
                    format!("File already exists: {}", path.to_string_lossy()),
                Error::FileNotFound(path) => format!("No such file: {}", path.to_string_lossy()),
                Error::FileMetadataFailed => "Failed to process file metadata.".to_string(),
                Error::InvalidUpdateUrl((expected, received)) => format!(
                    "Repository JSON file was downloaded from {} but specifies update URL as {}",
                    expected, received
                ),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::FailedToDecodeImageBytes(value)
    }
}

impl From<ChecksumError> for Error {
    fn from(value: ChecksumError) -> Self {
        Error::InvalidChecksum(value)
    }
}
