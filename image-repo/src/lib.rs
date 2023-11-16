#![deny(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    clippy::unwrap_used
)]

//! Types for the JSON image repositories and utilities to download and validate them.

use types::ImageRepo;
use url::ParseError;

/// Types and traits for the crate.
pub mod types;

pub use reqwest::Url;

pub use encoding::RegexError;

/// Possible errors while fetching data from URLs.
#[derive(Debug)]
pub enum Error {
    /// Invalid URL provided
    UrlParse(ParseError),
    /// HTTP error
    Request(reqwest::Error),
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::UrlParse(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Request(value)
    }
}

/// Download a repository JSON file from a URL and parse it to an [`types::ImageRepo`]
///
/// # Errors
///
/// [`Error`]
pub async fn download_repo_manifest<T>(url: T) -> Result<ImageRepo, Error>
where
    T: AsRef<str>,
{
    let url = Url::parse(url.as_ref())?;
    let repo = reqwest::get(url).await?.json::<ImageRepo>().await?;
    Ok(repo)
}
