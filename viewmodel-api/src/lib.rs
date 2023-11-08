pub mod error;
pub mod types;
pub mod viewmodels;

use error::Error;
use error::Result;
use image_repo::types::ImageRepo;
use reqwest::StatusCode;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;
use types::DownloadableResource;
use types::TryIntoStoragePath;
use types::UpdateInterval;
use viewmodels::RepositoryViewModel;

#[cfg(debug_assertions)]
const STORAGE_ROOT: &str = "jdt-debug";
#[cfg(not(debug_assertions))]
const STORAGE_ROOT: &str = "jdt";

#[derive(Debug)]
pub enum StorageType {
    Repo,
    Image,
}

/// Download a resource from the internet and return the response body bytes.
pub(crate) async fn download_bytes<T>(url: T) -> Result<Vec<u8>>
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

/// Get the toplevel storage root directory for the given storage type.
pub fn storage_root(storage_type: StorageType) -> Result<PathBuf> {
    match storage_type {
        StorageType::Repo => {
            dirs_next::config_dir().map(|config| config.join(STORAGE_ROOT).join("repositories"))
        }
        StorageType::Image => {
            dirs_next::cache_dir().map(|cache| cache.join(STORAGE_ROOT).join("images"))
        }
    }
    .ok_or(Error::FailedToGetStorageDir)
}

/// Store the given resource.
pub async fn store_resource<T>(resource: &T, bytes: &[u8], overwrite: bool) -> Result<PathBuf>
where
    T: TryIntoStoragePath,
{
    let filepath = resource.try_into_storage_path()?;
    if !overwrite && filepath.exists() {
        return Err(Error::FileAlreadyExists(filepath));
    }

    let path = resource.try_into_storage_path()?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .await?;
    file.write_all(bytes).await?;
    file.flush().await?;
    Ok(path)
}

/// Load the resource if it exists locally. Does not connect to the internet to download
/// the resource.
pub async fn local_load_resource<T>(resource: T) -> Result<Vec<u8>>
where
    T: TryIntoStoragePath,
{
    let path = resource.try_into_storage_path()?;
    if !path.exists() {
        return Err(Error::FileNotFound(path));
    }
    let mut file = File::open(path).await?;
    let mut file_bytes = vec![];
    file.read_to_end(&mut file_bytes).await?;
    Ok(file_bytes)
}

/// Download the given resource and save it to disk in the right location.
/// Overwrites the file if it already exists.
///
/// Example:
///
/// ```no_run
/// # async fn test() {
/// # use reqwest::Url;
/// // (ImageRepo, Vec<u8>)
/// let (img_repo, json_bytes) = viewmodel_api::download_resource_to_file(Url::parse("").unwrap())
///     .await
///     .unwrap();
/// // (ImageData, Vec<u8>)
/// let (img_data, img_bytes) = viewmodel_api::download_resource_to_file(img_repo.images[0].clone()).await.unwrap();
/// # }
/// ```
pub async fn download_resource_to_file<T, V>(downloadable: T) -> Result<(V, PathBuf)>
where
    V: TryIntoStoragePath,
    T: DownloadableResource<V>,
{
    let (resource, bytes) = downloadable.download_resource().await?;
    let path = store_resource(&resource, bytes.as_slice(), true).await?;
    Ok((resource, path))
}

/// Check if a file has not been updated since longer than the specified interval.
/// This checks the file modified metadata.
pub async fn needs_update(path: &PathBuf, update_interval: UpdateInterval) -> Result<bool> {
    let metadata = fs::metadata(path).await?;
    let modified_time = metadata.modified()?;
    let elapsed = modified_time
        .elapsed()
        .map_err(|_| Error::FileMetadataFailed)?;
    Ok(elapsed > update_interval.into())
}

/// List all insatlled image repositories.
pub async fn list_repositories() -> Result<Vec<RepositoryViewModel>> {
    let storage_root = storage_root(StorageType::Repo)?;
    let mut repos = vec![];
    let mut dir_stream = ReadDirStream::new(fs::read_dir(storage_root).await?);
    while let Some(file) = dir_stream.next().await {
        let path = file?.path();
        let view = RepositoryViewModel::new(path).await?;
        repos.push(view);
    }

    Ok(repos)
}
