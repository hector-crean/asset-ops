pub mod video;

use std::{
    ffi::OsStr,
    fs::{self, File},
    path::Path,
};

use image::{
    codecs::{
        jpeg::{self, JpegEncoder},
        png::{self, PngEncoder},
    },
    ImageError, ImageResult,
};

use crate::{
    errors::{self, AssetOpsError},
    models::file_kind::AssetFile,
};

impl AssetFile {
    pub fn write<Q: AsRef<Path>>(&self, to: Q) -> Result<(), errors::AssetOpsError> {
        match self {
            AssetFile::PNG(path) => match image::open(Path::new(&path)) {
                Ok(dyn_image) => {
                    let mut wtr = File::create(&to)?;

                    let encoder = PngEncoder::new_with_quality(
                        &mut wtr,
                        png::CompressionType::Best,
                        png::FilterType::NoFilter,
                    );
                    dyn_image.write_with_encoder(encoder)?;

                    Ok(())
                }
                Err(err) => Err(AssetOpsError::ImageError(err)),
            },
            AssetFile::JPG(path) => match image::open(Path::new(&path)) {
                Ok(dyn_image) => {
                    let mut wtr = File::create(&to)?;

                    let encoder = JpegEncoder::new_with_quality(&mut wtr, 72);

                    dyn_image.write_with_encoder(encoder)?;

                    Ok(())
                }
                Err(err) => Err(AssetOpsError::ImageError(err)),
            },
            AssetFile::MP3(path)
            | AssetFile::MP4(path)
            | AssetFile::PDF(path)
            | AssetFile::WAV(path) => match fs::copy(&path, &to) {
                Ok(_) => {
                    // tracing::info!("✔️, {:?}", &from_str);
                    Ok(())
                }
                Err(err) => Err(AssetOpsError::IoError(err)),
            },
        }
    }
}
