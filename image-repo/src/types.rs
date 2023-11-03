use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SupportedFiletype {
    Jpg,
    Png,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub url: Url,
    pub width: i16,
    pub height: i16,
    pub filetype: SupportedFiletype,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRepo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub images: Vec<ImageData>,
}

#[cfg(test)]
mod tests {
    use super::ImageRepo;

    #[test]
    fn deserializes_from_test_repo() {
        let repo_json = include_str!("../examples/example_repo.json");
        let result = serde_json::from_str::<ImageRepo>(repo_json);
        assert!(result.is_ok());
        let repo = result.unwrap();
        assert!(!repo.images.is_empty());
    }
}
