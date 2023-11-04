pub mod error;
pub mod types;

use error::Error;
use error::Result;
use reqwest::StatusCode;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use types::DownloadableResource;
use types::TryIntoStoragePath;

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
pub fn store_resource<T>(resource: &T, bytes: &[u8], overwrite: bool) -> Result<PathBuf>
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
        .open(&path)?;
    file.write_all(bytes)?;
    file.flush()?;
    Ok(path)
}

/// Load the resource if it exists locally. Does not connect to the internet to download
/// the resource.
pub fn local_load_resource<T>(resource: T) -> Result<Vec<u8>>
where
    T: TryIntoStoragePath,
{
    let path = resource.try_into_storage_path()?;
    if !path.exists() {
        return Err(Error::FileNotFound(path));
    }
    let mut file = File::open(path)?;
    let mut file_bytes = vec![];
    file.read_to_end(&mut file_bytes)?;
    Ok(file_bytes)
}

/// Download the given resource and save it to disk in the right location.
/// Overwrites the file if it already exists.
///
/// Example:
///
/// ```no_run
/// // (ImageRepo, Vec<u8>)
/// let (img_repo, json_bytes) = download_resource_to_file(Url::parse("").unwrap())
///     .await
///     .unwrap();
/// // (ImageData, Vec<u8>)
/// let (img_data, img_bytes) = download_resource_to_file(img_repo.images[0]).await.unwrap();
/// ```
pub async fn download_resource_to_file<T, V>(downloadable: T) -> Result<(V, PathBuf)>
where
    V: TryIntoStoragePath,
    T: DownloadableResource<V>,
{
    let (resource, bytes) = downloadable.download_resource().await?;
    let path = store_resource(&resource, bytes.as_slice(), true)?;
    Ok((resource, path))
}
