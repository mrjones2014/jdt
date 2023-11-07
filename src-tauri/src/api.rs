use image_repo::types::ImageRepo;
use localstorage::error::Error;

trait TauriResult<T> {
    fn serialize_err(self) -> Result<T, String>;
}

impl<T> TauriResult<T> for Result<T, Error> {
    fn serialize_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_repositories_viewmodel() -> Result<Vec<ImageRepo>, String> {
    localstorage::list_repositories().await.serialize_err()
}
