use std::{
    collections::HashMap,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use toml::{Table, Value};

use crate::{
    context::{Context, Data},
    story::Story,
};

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

    #[error("`DateTime` and `Array` are not supported context values")]
    UnsupportedContextValue,
}

#[derive(Debug)]
pub struct FileHandler;

impl FileHandler {
    pub fn build_story(base_directory: impl Into<PathBuf>) -> Result<Story, FileHandlerError> {
        let mut story_model = Story::new_part("root");

        Self::build_story_from_dir(base_directory.into().as_path(), &mut story_model)?;
        Ok(story_model)
    }

    fn build_story_from_dir(path: &Path, partition: &mut Story) -> Result<(), FileHandlerError> {
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
                    let mut nested_story_model = Story::new_part(object_name);
                    Self::build_story_from_dir(entry_path, &mut nested_story_model)?;
                    partition.push(nested_story_model);
                } else if let Some(extension) = entry_path.extension() {
                    if extension == "mt" {
                        let file_string = fs::read_to_string(entry_path)
                            .map_err(|error| FileHandlerError::IoException(error))?;

                        partition.push(Story::new_content(object_name, &file_string))
                    }
                }
            }
        }
        Ok(())
    }

    pub fn build_context(base_directory: impl Into<PathBuf>) -> Result<Context, FileHandlerError> {
        let file_string = fs::read_to_string(base_directory.into().as_path())
            .map_err(|error| FileHandlerError::IoException(error))?;
        let table = file_string
            .parse::<Table>()
            .or(Err(FileHandlerError::UnableToParseContext))?;

        let variables = Self::build_context_variables(table)?;

        Ok(Context { variables })
    }

    fn build_context_variables(table: Table) -> Result<HashMap<String, Data>, FileHandlerError> {
        let mut variables = HashMap::new();

        for (key, value) in table.iter() {
            match value {
                Value::String(string_value) => {
                    variables.insert(key.to_owned(), Data::String(string_value.to_owned()));
                }
                Value::Integer(integer_value) => {
                    variables.insert(key.to_owned(), Data::Number(*integer_value as f64));
                }
                Value::Float(float_value) => {
                    variables.insert(key.to_owned(), Data::Number(*float_value));
                }
                Value::Boolean(boolean_value) => {
                    variables.insert(key.to_owned(), Data::Boolean(*boolean_value));
                }
                Value::Table(table_value) => {
                    let object_value = Self::build_context_variables(table_value.to_owned())?;
                    variables.insert(key.to_owned(), Data::Object(object_value));
                }
                _ => return Err(FileHandlerError::UnsupportedContextValue),
            }
        }

        Ok(variables)
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn builds_story_model() {
        let result = FileHandler::build_story("./mock/draft");
        assert!(result.is_ok());
    }

    #[test]
    fn fetches_context() {
        let result = FileHandler::build_context("./mock/Context.toml");
        assert!(result.is_ok());
    }
}
