use std::{
    env,
    error::Error,
    fmt::Display,
    io::{Cursor, Read},
};

use colored_json::ToColoredJson;
use image::{io::Reader, GenericImageView, ImageFormat};
use image_repo::types::{ImageData, SupportedFormat};
use reqwest::StatusCode;
use url::Url;

#[derive(Debug)]
enum Errors {
    HttpFailed,
    ImgFormatDetectionFailed,
    UnsupportedImgFormat,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Errors::HttpFailed => "Failed to download image.",
                Errors::ImgFormatDetectionFailed => "Failed to detect image format.",
                Errors::UnsupportedImgFormat => "Unsupported image format.",
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

        //
        // Detect format
        //
        let format = imghdr::from_bytes(&img_bytes);
        if format.is_none() {
            errors.push(Box::new(Errors::ImgFormatDetectionFailed));
            continue;
        }
        let format = match format.unwrap() {
            imghdr::Type::Jpeg => Some(SupportedFormat::Jpg),
            imghdr::Type::Png => Some(SupportedFormat::Png),
            _ => None,
        };
        if format.is_none() {
            errors.push(Box::new(Errors::UnsupportedImgFormat));
            continue;
        }
        let format = format.unwrap();

        //
        // Detect dimensions
        //
        let img_reader = Reader::with_format(
            Cursor::new(img_bytes),
            match format {
                SupportedFormat::Jpg => ImageFormat::Jpeg,
                SupportedFormat::Png => ImageFormat::Png,
            },
        )
        .decode();
        if let Err(e) = img_reader {
            errors.push(Box::new(e));
            continue;
        }
        let (width, height) = img_reader.unwrap().dimensions();

        let img_data = ImageData {
            url: Url::parse(url.as_str()).unwrap(),
            width,
            height,
            format,
        };
        images.push(img_data);
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
