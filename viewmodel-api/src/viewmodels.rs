use std::path::PathBuf;

use crate::error::Result;
use chrono::{DateTime, Utc};
use image_repo::types::ImageRepo;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryViewModel {
    /// Timestamp formatted as an ISO 8601 timestamp. Should be able to parse for formatting
    /// on the frontend using a library like Luxon.
    pub last_updated: String,
    /// Repo name
    pub name: String,
    /// Repo JSON file update URL
    pub update_url: Option<String>,
    /// Local disk path of repo JSON file
    pub path: PathBuf,
}

impl RepositoryViewModel {
    pub async fn new(path: PathBuf) -> Result<RepositoryViewModel> {
        let mut file = File::open(&path).await?;
        let mut file_bytes = vec![];
        file.read_to_end(&mut file_bytes).await?;
        let repo = serde_json::from_slice::<ImageRepo>(file_bytes.as_slice())?;
        let last_updated: DateTime<Utc> = file.metadata().await?.modified()?.into();
        let last_updated = last_updated.to_rfc3339();
        Ok(Self {
            last_updated,
            name: repo.name,
            update_url: repo.update_url.map(|url| url.to_string()),
            path,
        })
    }
}
