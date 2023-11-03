use types::ImageRepo;
use url::{ParseError, Url};

pub mod types;

#[derive(Debug)]
pub enum Error {
    UrlParse(ParseError),
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

pub async fn download_repo_manifest<T>(url: T) -> Result<ImageRepo, Error>
where
    T: AsRef<str>,
{
    let url = Url::parse(url.as_ref())?;
    let repo = reqwest::get(url).await?.json::<ImageRepo>().await?;
    Ok(repo)
}
