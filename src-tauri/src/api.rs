use serde::{Deserialize, Serialize};
use typeshare::typeshare;
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

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum CommandRequest {
    GetRepositories,
    AddRepository(Url),
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum CommandResponse {
    GetRepositories(Vec<RepositoryViewModel>),
    AddRepository(RepositoryViewModel),
}

#[tauri::command]
pub async fn invoke(request: CommandRequest) -> Result<CommandResponse, String> {
    match request {
        CommandRequest::GetRepositories => viewmodel_api::list_repositories()
            .await
            .serialize_err()
            .map(CommandResponse::GetRepositories),
        CommandRequest::AddRepository(url) => {
            let (repo, file_path) = viewmodel_api::download_resource_to_file(url)
                .await
                .serialize_err()?;
            RepositoryViewModel::from_resource(repo, file_path)
                .await
                .serialize_err()
                .map(CommandResponse::AddRepository)
        }
    }
}
