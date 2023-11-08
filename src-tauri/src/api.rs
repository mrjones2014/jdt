use viewmodel_api::{error::Error, viewmodels::RepositoryViewModel};

trait TauriResult<T> {
    fn serialize_err(self) -> Result<T, String>;
}

impl<T> TauriResult<T> for Result<T, Error> {
    fn serialize_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_repositories_viewmodel() -> Result<Vec<RepositoryViewModel>, String> {
    viewmodel_api::list_repositories().await.serialize_err()
}
