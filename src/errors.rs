use thiserror::Error;

use crate::models;

#[derive(Error, Debug)]
pub enum AssetOpsError {
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error("File name has no extension")]
    NoFileExtension,
    #[error("Not a valid file name")]
    InvalidFileName,
    #[error("No valid base file")]
    InvalidBaseFile,
    #[error("An OS string is not valid utf-8")]
    OsStringNotUtf8,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    ParseAssetFileError(#[from] models::file_kind::ParseAssetFileError),
}
