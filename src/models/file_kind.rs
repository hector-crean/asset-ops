use std::{str::FromStr, string::ParseError};
use strum::EnumVariantNames;

#[derive(Debug, EnumVariantNames)]
pub enum AssetFile {
    PDF(String),
    PNG(String),
    JPG(String),
    MP4(String),
    WAV(String),
    MP3(String),
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ParseAssetFileError(String);

impl FromStr for AssetFile {
    type Err = ParseAssetFileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let extension = s
            .split('.')
            .last()
            .ok_or_else(|| ParseAssetFileError("Invalid file path".to_owned()))?;

        match extension {
            "png" => Ok(AssetFile::PNG(s.to_owned())),
            "jpeg" | "jpg" => Ok(AssetFile::JPG(s.to_owned())),
            "wav" => Ok(AssetFile::WAV(s.to_owned())),
            "mp3" => Ok(AssetFile::MP3(s.to_owned())),
            "mp4" => Ok(AssetFile::MP4(s.to_owned())),
            _ => Err(ParseAssetFileError(format!(
                "Unsupported file type: {}",
                extension
            ))),
        }
    }
}
