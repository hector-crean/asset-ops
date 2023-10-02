pub mod asset;
pub mod errors;
pub mod models;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::{fs, io, ops::Deref, path::Path, str::FromStr};

use crate::models::file_kind::AssetFile;
/// 1. Parse json file:
/// 2. find all strs which match a Z// drive regex
/// 3. find them in the folder specified
/// 4. if found, copy them to target directory, and replace the matched str with new path
/// 5. report on finding etc.
///
///
///

pub fn read_json<P: AsRef<Path>>(path: P) -> Value {
    let file = fs::File::open(path).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    json
}

pub struct JsonStrVisitor {
    pub collected: Vec<String>,
}

impl JsonStrVisitor {
    pub fn new() -> Self {
        Self {
            collected: Vec::new(),
        }
    }
    pub fn collected(self) -> Vec<String> {
        self.collected
    }
    pub fn visit<Pred, Op>(&mut self, value: &Value, predicate: &Pred, op: &Op) -> &Self
    where
        Pred: Fn(&str) -> bool,
        Op: Fn(&str) -> &str,
    {
        match value {
            Value::Object(map) => {
                for (_, v) in map {
                    self.visit(v, predicate, op);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    self.visit(v, predicate, op);
                }
            }
            Value::String(s) => match predicate(op(s)) {
                true => self.collected.push(s.to_string()),
                false => {}
            },
            _ => {}
        }

        self
    }
}

struct Slug(String);
impl Slug {
    pub fn new(s: &str) -> Self {
        let slug = s
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric() || c == ' ' || c == '.')
            .collect::<String>()
            .replace(' ', "-");

        Self(slug)
    }
}
impl Deref for Slug {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Define an enum to represent a File or a Folder
pub enum FileSystemEntity {
    File,
    Folder,
}

impl FileSystemEntity {
    // Function to classify a Path as either a File or a Folder
    pub fn classify(path: &Path) -> io::Result<Self> {
        let metadata = path.metadata()?;
        Ok(if metadata.is_file() {
            FileSystemEntity::File
        } else if metadata.is_dir() {
            FileSystemEntity::Folder
        } else {
            // You can add more cases here for other types of file system entities if needed
            // For now, we'll just treat unknown entities as files
            FileSystemEntity::File
        })
    }
}

pub enum TargetFile<'src> {
    // RelativePath(&'target str),
    AbsolutePath(&'src Path),
}

pub struct FileSearcher<'dest, Q: AsRef<Path>> {
    dest_folder_path: &'dest Q,
}

impl<'target, 'dest: 'target, Q: AsRef<Path>> FileSearcher<'target, Q> {
    pub fn new(dest_folder_path: &'dest Q) -> Self {
        Self { dest_folder_path }
    }
    pub fn search_and_copy<'src, R: AsRef<Path>>(
        &self,
        target_file: TargetFile,
        search_directory: R,
    ) -> Result<(), errors::AssetOpsError> {
        match target_file {
            // TargetFile::RelativePath(p) => {
            //     for entry in fs::read_dir(search_directory)? {
            //         let entry = entry?;
            //         let path = entry.path();
            //         tracing::info!("searching: {:?}", path.display());

            //         match FileSystemEntity::classify(&path)? {
            //             FileSystemEntity::File => {
            //                 let canon_path = path.canonicalize()?;

            //                 if canon_path.ends_with(relative_path) {
            //                     let file_name = path
            //                         .file_name()
            //                         .and_then(|f| f.to_str())
            //                         .unwrap_or("fucked_it");

            //                     let slug = Slug::new(file_name);
            //                     let dest_path = self.dest_folder_path.as_ref().join(&*slug);

            //                     let file_type = AssetFile::from_str(relative_path)?;
            //                     file_type.write(dest_path)?;
            //                 } else {
            //                 }
            //             }
            //             FileSystemEntity::Folder => self.search_and_copy(target_file, path)?,
            //         }
            //     }
            // }
            TargetFile::AbsolutePath(p) => {
                let file_name = p.file_name().and_then(|f| f.to_str()).unwrap();

                let slug = Slug::new(file_name);

                let dest_path = self.dest_folder_path.as_ref().join(&*slug);
                fs::copy(p, dest_path)?;
            }
        }
        Ok(())
    }
}

lazy_static! {
    static ref FILE_NAME_REGEX: Regex =
        Regex::new(r#"^(?x)[^\\/:*?<>|\r\n]+\.(pdf|png|jpeg|jpg|mp4|wav|mp3)$"#).unwrap();
    static ref FILE_PATH_REGEX: Regex =
        Regex::new(r#"^(?x)[^:*?<>|\r\n]+\.(pdf|png|jpeg|jpg|mp4|wav|mp3)$"#).unwrap();
}

pub fn filename_predicate(haystack: &str) -> bool {
    FILE_PATH_REGEX.is_match(haystack)
}

#[cfg(test)]
mod tests {
    use super::FILE_PATH_REGEX;

    #[test]
    fn test_valid_filenames() {
        let valid_filenames = vec![
            "valid.pdf",
            "valid.png",
            "valid.jpeg",
            "valid.jpg",
            "valid.mp4",
            "valid.wav",
            "valid.mp3",
            "valid/valid.pdf",
            r"filename\valid.pdf",
        ];

        for filename in valid_filenames {
            assert!(
                FILE_PATH_REGEX.is_match(filename),
                "Expected {} to be a valid filename",
                filename
            );
        }
    }

    #[test]
    fn test_invalid_filenames() {
        let invalid_filenames = vec![
            "filename.txt",
            "filename.docx",
            "filename.exe",
            "filename:invalid.pdf",
            "filename*invalid.pdf",
            "filename?invalid.pdf",
            "filename<invalid.pdf",
            "filename>invalid.pdf",
            "filename|invalid.pdf",
        ];

        for filename in invalid_filenames {
            assert!(
                !FILE_PATH_REGEX.is_match(filename),
                "Expected {} to be an invalid filename",
                filename
            );
        }
    }
}
