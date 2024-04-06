#![doc(hidden)]

use std::{
    fs::{self},
    io::Read,
    path::PathBuf,
};

use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("Failed to read file name ({file_path})")]
    FileName { file_path: PathBuf },

    #[error("Failed to read entry ({dir_path})")]
    Entry { dir_path: PathBuf },

    #[error("Failed to read directory ({dir_path})")]
    Directory { dir_path: PathBuf },

    #[error("Failed to open file ({file_name})")]
    OpenFile { file_name: String },

    #[error("Failed to read file ({file_name})")]
    ReadFile { file_name: String },
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub extension: Option<String>,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum PathItem {
    File(File),
    Directory(Box<Directory>),
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<PathItem>,
}

impl Directory {
    #[allow(missing_docs)]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            name: title.into(),
            contents: Vec::new(),
        }
    }

    #[allow(missing_docs)]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[allow(missing_docs)]
    pub fn contents(&self) -> &Vec<PathItem> {
        &self.contents
    }

    #[allow(missing_docs)]
    pub fn push_item(&mut self, path_item: PathItem) {
        self.contents.push(path_item);
    }

    pub fn read(path: impl Into<PathBuf>) -> Result<Self, ReaderError> {
        let path: PathBuf = path.into();
        let read_dir = fs::read_dir(&path).map_err(|_error| ReaderError::Directory {
            dir_path: path.clone(),
        })?;

        let mut directory: Directory = {
            let name = path
                .clone()
                .file_name()
                .ok_or(ReaderError::FileName {
                    file_path: path.clone(),
                })?
                .to_string_lossy()
                .to_string();

            Self::new(name)
        };

        for entry in read_dir {
            let entry = entry.map_err(|_error| ReaderError::Entry {
                dir_path: path.clone(),
            })?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let nested_directory = Self::read(entry_path)?;
                directory.push_item(PathItem::Directory(Box::new(nested_directory)))
            } else {
                let name = entry.file_name().to_string_lossy().to_string();
                let mut file =
                    fs::File::open(&entry_path).map_err(|_error| ReaderError::OpenFile {
                        file_name: name.clone(),
                    })?;

                let mut content: Vec<u8> = vec![];
                file.read_to_end(&mut content)
                    .map_err(|_error| ReaderError::ReadFile {
                        file_name: name.clone(),
                    })?;

                let extension = entry
                    .path()
                    .extension()
                    .map(|os_string| os_string.to_string_lossy().to_string());
                let nested_file = File {
                    content,
                    name,
                    extension,
                };

                directory.push_item(PathItem::File(nested_file));
            }
        }

        Ok(directory)
    }
}
