use std::{env, error::Error, fmt::Display, io::Read};

use colored_json::ToColoredJson;
use image_repo::types::ImageData;
use reqwest::StatusCode;
use url::Url;

#[derive(Debug)]
enum Errors {
    HttpFailed,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Errors::HttpFailed => "Failed to download image.",
            }
        )
    }
}

impl Error for Errors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn main() {
    let mut errors: Vec<Box<dyn Error>> = vec![];
    let mut images: Vec<ImageData> = vec![];
    for url in env::args().skip(1) {
        eprintln!("Downloading image {url} ...");
        //
        // Download image
        //
        let http_resp = reqwest::blocking::get(url.clone());
        if let Err(e) = http_resp {
            errors.push(Box::new(e));
            continue;
        }

        let mut http_resp = http_resp.unwrap();
        if http_resp.status() != StatusCode::OK {
            errors.push(Box::new(Errors::HttpFailed));
            continue;
        }
        let mut img_bytes = vec![];
        if let Err(e) = http_resp.read_to_end(&mut img_bytes) {
            errors.push(Box::new(e));
            continue;
        }

        eprintln!("Decoding image...");

        let img_data = ImageData::try_from((Url::parse(&url).unwrap(), img_bytes));
        if let Err(e) = img_data {
            errors.push(Box::new(e));
            continue;
        }
        let img_data = img_data.unwrap();
        images.push(img_data);

        eprintln!("Successfully processed image!");
    }

    let json = serde_json::to_string_pretty(&images).unwrap();

    #[cfg(target_os = "windows")]
    let _ = colored_json::enable_ansi_support();

    println!("{}", json.to_colored_json_auto().unwrap());

    if !errors.is_empty() {
        eprintln!("{} images failed to process:", errors.len());
        for error in errors.iter() {
            eprintln!("    {error}");
        }
    }
}
