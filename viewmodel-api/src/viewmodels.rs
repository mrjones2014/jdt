use std::path::PathBuf;

use crate::error::Result;
use chrono::{DateTime, Utc};
use image_repo::types::ImageRepo;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use typeshare::typeshare;

/// A view-friendly representation of an [`image_repo::types::ImageRepo`]
#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryViewModel {
    /// Timestamp formatted as an ISO 8601 timestamp. Should be able to parse for formatting
    /// on the frontend using a library like Luxon.
    pub last_updated: String,
    /// Repo name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Repo JSON file update URL
    pub update_url: Option<String>,
    /// Local disk path of repo JSON file
    pub path: PathBuf,
}

async fn metadata_last_updated(file: &File) -> Result<String> {
    let last_updated: DateTime<Utc> = file.metadata().await?.modified()?.into();
    Ok(last_updated.to_rfc3339())
}

async fn file_bytes(path: &PathBuf) -> Result<(File, Vec<u8>)> {
    let mut file = File::open(&path).await?;
    let mut file_bytes = vec![];
    file.read_to_end(&mut file_bytes).await?;
    Ok((file, file_bytes))
}

impl RepositoryViewModel {
    /// Read the file specified by the [`PathBuf`] and parse to a [`RepositoryViewModel`]
    ///
    /// # Errors
    ///
    /// [`crate::Error`]
    pub async fn from_path(path: PathBuf) -> Result<RepositoryViewModel> {
        let (file, file_bytes) = file_bytes(&path).await?;
        let repo = serde_json::from_slice::<ImageRepo>(file_bytes.as_slice())?;
        Ok(Self {
            last_updated: metadata_last_updated(&file).await?,
            name: repo.name,
            description: repo.description,
            update_url: repo.update_url.map(|url| url.to_string()),
            path,
        })
    }

    /// Convert an [`ImageRepo`] to a [`RepositoryViewModel`]
    ///
    /// # Errors
    ///
    /// [`crate::Error`]
    pub async fn from_resource(repo: ImageRepo, path: PathBuf) -> Result<RepositoryViewModel> {
        let (file, _) = file_bytes(&path).await?;
        Ok(Self {
            last_updated: metadata_last_updated(&file).await?,
            name: repo.name,
            description: repo.description,
            update_url: repo.update_url.map(|url| url.to_string()),
            path,
        })
    }
}
