pub mod error;
pub mod types;
pub mod viewmodels;

use error::Error;
use error::Result;
use std::path::PathBuf;
use strum::EnumIter;
use strum::IntoEnumIterator;
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

#[derive(Debug, EnumIter)]
pub enum ResourceType {
    Repo,
    Image,
}

/// Get the toplevel storage root directory for the given storage type.
pub fn storage_root(resource_type: ResourceType) -> Result<PathBuf> {
    match resource_type {
        ResourceType::Repo => {
            dirs_next::config_dir().map(|config| config.join(STORAGE_ROOT).join("repositories"))
        }
        ResourceType::Image => {
            dirs_next::cache_dir().map(|cache| cache.join(STORAGE_ROOT).join("images"))
        }
    }
    .ok_or(Error::FailedToGetStorageDir)
}

pub async fn init_storage() -> Result<()> {
    for root in ResourceType::iter() {
        let root = storage_root(root)?;
        if !root.exists() {
            fs::create_dir_all(root).await?;
        }
    }

    Ok(())
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
        .create(true)
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
/// // (ImageRepo, PathBuf)
/// let (img_repo, json_file_path) = viewmodel_api::download_resource_to_file(Url::parse("").unwrap())
///     .await
///     .unwrap();
/// // (ImageData, PathBuf)
/// let (img_data, img_file_path) = viewmodel_api::download_resource_to_file(img_repo.images[0].clone()).await.unwrap();
/// # }
/// ```
pub async fn download_resource_to_file<T, V>(
    downloadable: T,
    overwrite: bool,
) -> Result<(V, PathBuf)>
where
    V: TryIntoStoragePath,
    T: DownloadableResource<V>,
{
    let (resource, bytes) = downloadable.download_resource().await?;
    let path = store_resource(&resource, bytes.as_slice(), overwrite).await?;
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
    let storage_root = storage_root(ResourceType::Repo)?;
    let mut repos = vec![];
    let mut dir_stream = ReadDirStream::new(fs::read_dir(storage_root).await?);
    while let Some(file) = dir_stream.next().await {
        let path = file?.path();
        let view = RepositoryViewModel::from_path(path).await?;
        repos.push(view);
    }

    Ok(repos)
}
