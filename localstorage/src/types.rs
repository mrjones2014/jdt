use crate::{download_bytes, error::Result, storage_root, StorageType};
use async_trait::async_trait;
use image_repo::types::{ImageData, ImageRepo};
use reqwest::Url;
use std::path::PathBuf;

/// Trait to allow getting a full filepath from
/// something that should be stored to disk.
pub trait TryIntoStoragePath {
    /// Get the full, absolute filepath that the resource should be stored at.
    fn try_into_storage_path(&self) -> Result<PathBuf>;
}

impl TryIntoStoragePath for ImageRepo {
    fn try_into_storage_path(&self) -> Result<PathBuf> {
        let file_name = self.to_file_name();
        storage_root(StorageType::Repo).map(|root| root.join(file_name))
    }
}

impl TryIntoStoragePath for ImageData {
    fn try_into_storage_path(&self) -> Result<PathBuf> {
        storage_root(StorageType::Image).map(|root| root.join(self.to_file_name()))
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
        let repo = serde_json::from_slice::<ImageRepo>(&bytes)?;
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
