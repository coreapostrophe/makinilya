//! Structs that handle the project files

use std::{
    fs::{self},
    io::{self, Read},
    path::{PathBuf, StripPrefixError},
};

use thiserror::Error;

use crate::{
    config::{Config, ConfigError},
    context::{Context, ContextError},
    story::Story,
};

#[derive(Error, Debug)]
pub enum FileHandlerError {
    #[error("`{path}` cannot end in `..`")]
    FileNameException { path: PathBuf },

    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    ContextError(#[from] ContextError),

    #[error(transparent)]
    IoException(#[from] io::Error),

    #[error(transparent)]
    StripPrefixException(#[from] StripPrefixError),
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub extension: Option<String>,
}

#[derive(Debug)]
pub enum PathItem {
    File(File),
    Directory(Box<Directory>),
}

impl PathItem {
    pub fn new_file(file_content: File) -> Self {
        Self::File(file_content)
    }

    pub fn new_directory(directory: Directory) -> Self {
        Self::Directory(Box::new(directory))
    }
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    contents: Vec<PathItem>,
}

impl Directory {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            name: title.into(),
            contents: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn contents(&self) -> &Vec<PathItem> {
        &self.contents
    }

    pub fn push_item(&mut self, path_item: PathItem) {
        self.contents.push(path_item);
    }

    pub fn read(path: impl Into<PathBuf>) -> Result<Self, FileHandlerError> {
        let path: PathBuf = path.into();
        let read_dir = fs::read_dir(&path)?;

        let mut directory: Directory = {
            let name = path
                .file_name()
                .ok_or(FileHandlerError::FileNameException { path: path.clone() })?
                .to_string_lossy()
                .to_string();

            Self::new(name)
        };

        for entry in read_dir {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let nested_directory = Self::read(entry_path)?;
                directory.push_item(PathItem::Directory(Box::new(nested_directory)))
            } else {
                let name = entry.file_name().to_string_lossy().to_string();
                let mut file = fs::File::open(&entry_path)?;

                let mut content: Vec<u8> = vec![];
                file.read_to_end(&mut content)?;

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

/// Unit struct that holds static functions that reads file
/// paths.
///
/// The `FileHandler` has two main use cases. Building a story
/// structure from the project draft, and fetching all external
/// configurations.
#[derive(Debug)]
pub struct FileHandler;

impl FileHandler {
    /// Builds a story from a provided path argument.
    ///
    /// This static function extracts all makinilya text files and
    /// stores them inside a `Story` struct.
    ///
    /// # Examples
    /// ```no_run
    /// use makinilya_core::files::FileHandler;
    ///
    /// let story = FileHandler::build_story("./draft");
    ///
    /// assert!(story.is_ok());
    /// ```
    pub fn build_story(path: impl Into<PathBuf>) -> Result<Story, FileHandlerError> {
        let directory = Directory::read(path)?;
        let story = Story::parse(&directory);
        Ok(story)
    }

    /// Builds the story context from provided path argument.
    ///
    /// This static function reads the context file path and parses
    /// all of its values into a `Context` struct.
    ///
    /// # Examples
    /// ```no_run
    /// use makinilya_core::files::FileHandler;
    ///
    /// let story = FileHandler::build_context("./Context.toml");
    /// ```
    pub fn build_context(path: impl Into<PathBuf>) -> Result<Context, FileHandlerError> {
        let file_string = fs::read_to_string(path.into().as_path())?;
        let context = Context::parse(&file_string)?;
        Ok(context)
    }

    /// Builds the story config from provided path argument.
    ///
    /// This static function reads the config file path and parses
    /// all of its values into a `Config` struct.
    ///
    /// # Examples
    /// ```no_run
    /// use makinilya_core::files::FileHandler;
    ///
    /// let story = FileHandler::build_config("./Config.toml");
    /// ```
    pub fn build_config(path: impl Into<PathBuf>) -> Result<Config, FileHandlerError> {
        let file_string = fs::read_to_string(path.into().as_path())?;
        let config = Config::parse(&file_string)?;
        Ok(config)
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use std::env;

    #[test]
    fn builds_story_model() {
        let mock_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/mock/01-standard-project/draft";
        let result = FileHandler::build_story(mock_path);
        assert!(result.is_ok());
    }

    #[test]
    fn fetches_context() {
        let mock_path =
            env::var("CARGO_MANIFEST_DIR").unwrap() + "/mock/01-standard-project/Context.toml";
        let result = FileHandler::build_context(mock_path);
        assert!(result.is_ok());
    }

    #[test]
    fn builds_config() {
        let mock_path =
            env::var("CARGO_MANIFEST_DIR").unwrap() + "/mock/01-standard-project/Config.toml";
        let result = FileHandler::build_config(mock_path);
        assert!(result.is_ok());
    }
}
