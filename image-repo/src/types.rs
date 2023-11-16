use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Supported image formats
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SupportedFormat {
    /// JPG/JPEG images
    Jpg,
    /// PNG images
    Png,
}

impl std::fmt::Display for SupportedFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SupportedFormat::Jpg => "jpg",
                SupportedFormat::Png => "png",
            }
        )
    }
}

/// Metadata about the image, as well as the URL to download it from.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    /// Download URL for the image
    pub url: Url,
    /// SHA256 hash of the image bytes
    pub hash: String,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Image format
    pub format: SupportedFormat,
}

/// Error checking the checksum of downloaded file vs. expected checksum
#[derive(Debug)]
pub enum ChecksumError {
    /// Checksums do not match. The contained value
    /// is the internal checksum first, then the checksum
    /// of the data passed by the caller.
    NoMatch((String, String)),
}

impl std::fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (expected, received) = match self {
            ChecksumError::NoMatch((expected, received)) => (expected, received),
        };
        write!(
            f,
            "Checksums do not match. Expected {expected} but got {received}",
        )
    }
}

impl std::error::Error for ChecksumError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ImageData {
    /// Given a byte slice, get a SHA256 checksum of it and
    /// verify that it matches the [`ImageData::hash`] property stored
    /// in this [`ImageData`].
    ///
    /// # Errors
    ///
    /// [`ChecksumError`]
    pub fn verify_checksum(&self, img_bytes: &[u8]) -> Result<(), ChecksumError> {
        let hash = encoding::checksum_string(img_bytes);
        if hash == self.hash {
            Ok(())
        } else {
            Err(ChecksumError::NoMatch((self.hash.clone(), hash)))
        }
    }

    /// Deterministically generate the filepath tail for the given
    /// [`ImageData`]. This path should be appended to the storage
    /// root under `$XDG_CACHE_HOME` before storing.
    #[must_use]
    pub fn to_file_name(&self) -> String {
        format!("{}.{}", self.hash, self.format)
    }
}

/// Repository metadata, including a list of [`image_repo::types::ImageData`]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageRepo {
    /// Repository name
    pub name: String,
    /// Repository description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Repository update URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_url: Option<Url>,
    /// The list of [`image_repo::types::ImageData`] the repository provides
    pub images: Vec<ImageData>,
}

impl ImageRepo {
    /// Get a safe file name string.
    ///
    /// # Errors
    ///
    /// Errors if regex fails to compile (should never happen).
    pub fn to_file_name(&self) -> Result<String, encoding::RegexError> {
        let raw_filename = [
            self.name.to_string(),
            self.update_url
                .clone()
                .map_or_else(String::new, |url| url.to_string()),
        ]
        .join(",");
        Ok(format!("{}.json", encoding::safe_filename(raw_filename)?))
    }
}

/// Errors that can occur while processing the image
#[cfg(feature = "decoding")]
#[derive(Debug)]
pub enum ImgError {
    /// Failed to detect image format.
    CouldntDetectFormat,
    /// Unsupported image format. Associated value is the format name.
    UnsupportedFormat(String),
    /// Failed to decode the image data.
    DecodingFailed(image::ImageError),
}

#[cfg(feature = "decoding")]
impl From<image::ImageError> for ImgError {
    fn from(value: image::ImageError) -> Self {
        Self::DecodingFailed(value)
    }
}

#[cfg(feature = "decoding")]
impl std::fmt::Display for ImgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImgError::CouldntDetectFormat => "Unable to detect image format from bytes.".into(),
                ImgError::UnsupportedFormat(fmt) => format!("Unsupported image format: {fmt}"),
                ImgError::DecodingFailed(e) => format!("Failed to decode image: {e}"),
            }
        )
    }
}

#[cfg(feature = "decoding")]
impl std::error::Error for ImgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(feature = "decoding")]
impl TryFrom<(Url, Vec<u8>)> for ImageData {
    type Error = ImgError;

    fn try_from((url, img_bytes): (Url, Vec<u8>)) -> Result<Self, Self::Error> {
        Self::try_from((url, img_bytes.as_slice()))
    }
}

#[cfg(feature = "decoding")]
impl TryFrom<(Url, &[u8])> for ImageData {
    type Error = ImgError;

    fn try_from((url, img_bytes): (Url, &[u8])) -> Result<Self, Self::Error> {
        use image::GenericImageView;
        use std::io::Cursor;

        let Some(format) = imghdr::from_bytes(img_bytes) else {
            return Err(ImgError::CouldntDetectFormat);
        };
        let format = match format {
            imghdr::Type::Jpeg => SupportedFormat::Jpg,
            imghdr::Type::Png => SupportedFormat::Png,
            _ => return Err(ImgError::UnsupportedFormat(format!("{format:?}"))),
        };

        let img_reader = image::io::Reader::with_format(
            Cursor::new(&img_bytes),
            match format {
                SupportedFormat::Jpg => image::ImageFormat::Jpeg,
                SupportedFormat::Png => image::ImageFormat::Png,
            },
        )
        .decode()?;
        let (width, height) = img_reader.dimensions();
        let hash = encoding::checksum_string(img_bytes);

        Ok(ImageData {
            url,
            hash,
            width,
            height,
            format,
        })
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::{ImageData, ImageRepo};

    #[test]
    fn deserializes_from_test_repo() {
        let repo_json = include_str!("../examples/example_repo.json");
        let result = serde_json::from_str::<ImageRepo>(repo_json);
        assert!(result.is_ok());

        let repo = result.expect("image_repo");
        assert!(!repo.images.is_empty());
    }

    #[test]
    #[cfg(feature = "decoding")]
    fn decodes_from_btye_vec() {
        let img_bytes = include_bytes!("../ferris.png").to_vec();
        let url =
            Url::parse("https://rustacean.net/assets/rustacean-flat-noshadow.png").expect("url");
        let result = ImageData::try_from((url, img_bytes));
        assert!(result.is_ok());
        let data = result.expect("imagedata");
        assert_eq!(
            serde_json::to_string_pretty(&data).expect("json string"),
            r#"{
  "url": "https://rustacean.net/assets/rustacean-flat-noshadow.png",
  "hash": "b64500c829882b4abed9d768dbb396569ff1d5e6baf7d274460ab372fe53aadb",
  "width": 460,
  "height": 307,
  "format": "png"
}"#
        );
    }
}
