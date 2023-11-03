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
    pub description: String,
    pub images: Vec<ImageData>,
}
