use std::path::PathBuf;

use url::Url;
use viewmodel_api::{error::Error, viewmodels::RepositoryViewModel, ResourceType};

trait TauriResult<T> {
    fn serialize_err(self) -> Result<T, String>;
}

impl<T> TauriResult<T> for Result<T, Error> {
    fn serialize_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

impl<T> TauriResult<T> for std::io::Result<T> {
    fn serialize_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_repositories_view_model() -> Result<Vec<RepositoryViewModel>, String> {
    viewmodel_api::list_repositories().await.serialize_err()
}

#[tauri::command]
pub async fn add_repository(url: Url) -> Result<RepositoryViewModel, String> {
    let (repo, file_path) = viewmodel_api::download_resource_to_file(url, false)
        .await
        .serialize_err()?;
    RepositoryViewModel::from_resource(repo, file_path)
        .await
        .serialize_err()
}

#[tauri::command]
pub async fn delete_resource(path: PathBuf, resource_type: ResourceType) -> Result<(), String> {
    let storage_root = viewmodel_api::storage_root(resource_type).serialize_err()?;
    if !path.starts_with(storage_root) {
        return Err("Attempted to delete file that is outside application storage.".into());
    }

    tokio::fs::remove_file(path).await.serialize_err()
}

#[tauri::command]
pub async fn update_repo(repo: RepositoryViewModel) -> Result<(), String> {
    if let Some(url) = repo.update_url {
        let url = Url::parse(&url).map_err(|_| "Invalid URL".to_string())?;
        let _ = viewmodel_api::download_resource_to_file(url, true)
            .await
            .serialize_err()?;
        Ok(())
    } else {
        Err("Repo has no update URL.".into())
    }
}
