use crate::{
    error::{Error, Result},
    storage_root, ResourceType,
};
use async_trait::async_trait;
use image_repo::types::{ImageData, ImageRepo};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};
use typeshare::typeshare;

const ONE_DAY: u64 = 86_400;
const ONE_WEEK: u64 = 604_800;

/// Download a resource from the internet and return the response body bytes.
async fn download_bytes<T>(url: T) -> Result<Vec<u8>>
where
    T: AsRef<str>,
{
    let url = url.as_ref();
    let http_resp = reqwest::get(url).await?;
    let status = http_resp.status();
    if status != StatusCode::OK {
        return Err(Error::HttpStatus(status));
    }
    Ok(http_resp.bytes().await?.to_vec())
}

/// Trait to allow getting a full filepath from
/// something that should be stored to disk.
pub trait TryIntoStoragePath {
    /// Get the full, absolute filepath that the resource should be stored at.
    fn try_into_storage_path(&self) -> Result<PathBuf>;
}

impl TryIntoStoragePath for ImageRepo {
    fn try_into_storage_path(&self) -> Result<PathBuf> {
        let file_name = self.to_file_name();
        storage_root(ResourceType::Repo).map(|root| root.join(file_name))
    }
}

impl TryIntoStoragePath for ImageData {
    fn try_into_storage_path(&self) -> Result<PathBuf> {
        storage_root(ResourceType::Image).map(|root| root.join(self.to_file_name()))
    }
}

#[async_trait]
pub trait DownloadableResource<T>
where
    T: TryIntoStoragePath,
{
    /// Download the data from the URL and store it locally as a resource.
    async fn download_resource(&self) -> Result<(T, Vec<u8>)>;
}

#[async_trait]
impl DownloadableResource<ImageRepo> for Url {
    async fn download_resource(&self) -> Result<(ImageRepo, Vec<u8>)> {
        let bytes = download_bytes(self).await?;
        // verify the contents are proper JSON schema
        let mut repo = serde_json::from_slice::<ImageRepo>(&bytes)?;
        // verify the update URL
        match &repo.update_url {
            // Check update URL matches
            Some(url) if url != self => {
                return Err(Error::InvalidUpdateUrl((url.to_string(), self.to_string())))
            }
            // Inject update URL it was downloaded from if there is none
            None => repo.update_url = Some(self.clone()),
            _ => {}
        };
        Ok((repo, bytes))
    }
}

#[async_trait]
impl DownloadableResource<ImageData> for ImageData {
    async fn download_resource(&self) -> Result<(ImageData, Vec<u8>)> {
        let bytes = download_bytes(self.url.clone()).await?;
        self.verify_checksum(&bytes)?;
        Ok((self.clone(), bytes))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
#[typeshare]
pub enum UpdateInterval {
    Days(u8),
    Weeks(u8),
}

impl From<UpdateInterval> for Duration {
    fn from(val: UpdateInterval) -> Self {
        match val {
            UpdateInterval::Days(n) => Duration::from_secs(ONE_DAY * (n as u64)),
            UpdateInterval::Weeks(n) => Duration::from_secs(ONE_WEEK * (n as u64)),
        }
    }
}
