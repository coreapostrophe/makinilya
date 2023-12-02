use std::{
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use toml::Table;

use crate::story::StoryModel;

#[derive(Error, Debug)]
pub enum FileHandlerError {
    #[error("path `{0}` is not a valid directory.")]
    InvalidDirectory(String),

    #[error("{0}")]
    IoException(io::Error),

    #[error("stripped prefix not found")]
    PrefixNotFound,

    #[error("context cannot be parsed")]
    UnableToParseContext,
}

#[derive(Debug)]
pub struct FileHandler;

impl FileHandler {
    pub fn build_story_model(
        base_directory: impl Into<PathBuf>,
    ) -> Result<StoryModel, FileHandlerError> {
        let mut story_model = StoryModel::new_part("root");

        Self::handle_dir(base_directory.into().as_path(), &mut story_model)?;
        Ok(story_model)
    }

    fn handle_dir(path: &Path, partition: &mut StoryModel) -> Result<(), FileHandlerError> {
        if !path.is_dir() {
            return Err(FileHandlerError::InvalidDirectory(
                path.to_string_lossy().into_owned(),
            ));
        }

        let read_dir = fs::read_dir(path).map_err(|error| FileHandlerError::IoException(error))?;

        for entry in read_dir {
            let entry = entry.map_err(|error| FileHandlerError::IoException(error))?;

            let entry_pathbuf = entry.path();
            let entry_path = entry_pathbuf.as_path();

            let stripped_path = entry_path
                .strip_prefix(path)
                .or(Err(FileHandlerError::PrefixNotFound))?;

            if let Some(object_name) = stripped_path.to_str() {
                if entry_path.is_dir() {
                    let mut nested_story_model = StoryModel::new_part(object_name);
                    Self::handle_dir(entry_path, &mut nested_story_model)?;
                    partition.push(nested_story_model);
                } else if let Some(extension) = entry_path.extension() {
                    if extension == "mt" {
                        let file_string = fs::read_to_string(entry_path)
                            .map_err(|error| FileHandlerError::IoException(error))?;

                        partition.push(StoryModel::new_content(object_name, &file_string))
                    }
                }
            }
        }
        Ok(())
    }

    pub fn fetch_context(base_directory: impl Into<PathBuf>) -> Result<Table, FileHandlerError> {
        let file_string = fs::read_to_string(base_directory.into().as_path())
            .map_err(|error| FileHandlerError::IoException(error))?;
        let table = file_string
            .parse::<Table>()
            .or(Err(FileHandlerError::UnableToParseContext))?;
        Ok(table)
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn builds_story_model() {
        let result = FileHandler::build_story_model("./mock/draft");
        assert!(result.is_ok());
    }

    #[test]
    fn fetches_context() {
        let result = FileHandler::fetch_context("./mock/Context.toml");
        assert!(result.is_ok());
    }
}
