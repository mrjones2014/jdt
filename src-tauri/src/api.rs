use url::Url;
use viewmodel_api::{error::Error, viewmodels::RepositoryViewModel};

trait TauriResult<T> {
    fn serialize_err(self) -> Result<T, String>;
}

impl<T> TauriResult<T> for Result<T, Error> {
    fn serialize_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_repositories_viewmodel() -> Result<Vec<RepositoryViewModel>, String> {
    viewmodel_api::list_repositories().await.serialize_err()
}

#[tauri::command(rename_all = "camelCase")]
pub async fn add_repository(url: Url) -> Result<RepositoryViewModel, String> {
    let (repo, file_path) = viewmodel_api::download_resource_to_file(url)
        .await
        .serialize_err()?;
    RepositoryViewModel::from_resource(repo, file_path)
        .await
        .serialize_err()
}
