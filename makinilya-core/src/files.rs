//! Structs that handle the project files

use std::{
    collections::HashMap,
    fs::{self},
    io,
    path::{PathBuf, StripPrefixError},
};

use thiserror::Error;
use toml::{Table, Value};

use crate::{
    config::Config,
    context::{Context, Data},
    story::Story,
};

#[derive(Error, Debug)]
pub enum FileHandlerError {
    #[error("`{path}` is not a valid directory.")]
    InvalidDirectory { path: String },

    #[error("Unable to obtain directory name.")]
    DirectoryNameException,

    #[error("Can't parse directory name to string.")]
    StringDirectoryName,

    #[error("Context is not a valid toml file.")]
    UnableToParseContext,

    #[error("`DateTime` and `Array` are not supported context values.")]
    UnsupportedContextValue,

    #[error(transparent)]
    IoException(#[from] io::Error),

    #[error(transparent)]
    StripPrefixException(#[from] StripPrefixError),
}

pub const MAKINILYA_TEXT_EXTENSION: &str = "mt";

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
    /// ```
    /// use makinilya_core::files::FileHandler;
    ///
    /// let story = FileHandler::build_story("./mock");
    ///
    /// assert!(story.is_ok());
    /// ```
    pub fn build_story(path: impl Into<PathBuf>) -> Result<Story, FileHandlerError> {
        let story = Self::build_story_from_dir(path.into())?;
        Ok(story)
    }

    fn build_story_from_dir(path: PathBuf) -> Result<Story, FileHandlerError> {
        if !path.exists() || !path.is_dir() {
            let path = path.to_string_lossy().into_owned();
            return Err(FileHandlerError::InvalidDirectory { path });
        }

        let directory_name = path
            .file_name()
            .ok_or(FileHandlerError::DirectoryNameException)?
            .to_str()
            .ok_or(FileHandlerError::StringDirectoryName)?;

        let mut story = Story::new(directory_name);
        let read_dir = fs::read_dir(&path).map_err(|error| FileHandlerError::IoException(error))?;

        for entry in read_dir {
            let entry = entry.map_err(|error| FileHandlerError::IoException(error))?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let nested_story = Self::build_story_from_dir(entry_path)?;
                story.push_part(nested_story);
            } else if let Some(extension) = entry_path.extension() {
                if extension == MAKINILYA_TEXT_EXTENSION {
                    let file_content_string = fs::read_to_string(&entry_path)
                        .map_err(|error| FileHandlerError::IoException(error))?;
                    story.push_content(file_content_string)
                }
            }
        }

        Ok(story)
    }

    /// Builds the story context from provided path argument.
    ///
    /// This static function reads the context file path and parses
    /// all of its values into a `Context` struct.
    ///
    /// # Examples
    /// ```
    /// use makinilya_core::files::FileHandler;
    ///
    /// let story = FileHandler::build_context("./Context.toml");
    /// ```
    pub fn build_context(path: impl Into<PathBuf>) -> Result<Context, FileHandlerError> {
        let file_string = fs::read_to_string(path.into().as_path())
            .map_err(|error| FileHandlerError::IoException(error))?;

        let table = file_string
            .parse::<Table>()
            .or(Err(FileHandlerError::UnableToParseContext))?;

        let variables = Self::build_context_variables(table)?;
        let context = Context::from(variables);

        Ok(context)
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

    fn set_value<'a, T: From<&'a String>>(field: &mut T, value: Option<&'a Value>) {
        if let Some(value) = value {
            match value {
                Value::String(string_value) => {
                    *field = string_value.into();
                }
                _ => (),
            }
        }
    }

    fn set_option_value<'a, T: From<&'a String>>(field: &mut Option<T>, value: Option<&'a Value>) {
        if let Some(value) = value {
            match value {
                Value::String(string_value) => {
                    *field = Some(string_value.into());
                }
                _ => (),
            }
        }
    }

    pub fn build_config(path: impl Into<PathBuf>) -> Result<Config, FileHandlerError> {
        let file_string = fs::read_to_string(path.into().as_path())
            .map_err(|error| FileHandlerError::IoException(error))?;

        let table = file_string
            .parse::<Table>()
            .or(Err(FileHandlerError::UnableToParseContext))?;

        let mut config = Config::default();

        if let Some(story_table) = table.get("story") {
            Self::set_value(&mut config.builder_layout.title, story_table.get("title"));
            Self::set_value(
                &mut config.builder_layout.pen_name,
                story_table.get("pen_name"),
            );
        }

        if let Some(author_table) = table.get("author") {
            Self::set_value(
                &mut config.builder_layout.contact_information.name,
                author_table.get("name"),
            );
            Self::set_option_value(
                &mut config.builder_layout.contact_information.mobile_number,
                author_table.get("mobile_number"),
            );
            Self::set_value(
                &mut config.builder_layout.contact_information.address_1,
                author_table.get("address_1"),
            );
            Self::set_option_value(
                &mut config.builder_layout.contact_information.address_2,
                author_table.get("address_2"),
            );
            Self::set_value(
                &mut config.builder_layout.contact_information.email_address,
                author_table.get("email_address"),
            );
        }

        if let Some(agent_table) = table.get("agent") {
            Self::set_value(
                &mut config.builder_layout.contact_information.name,
                agent_table.get("name"),
            );
            Self::set_option_value(
                &mut config.builder_layout.contact_information.mobile_number,
                agent_table.get("mobile_number"),
            );
            Self::set_value(
                &mut config.builder_layout.contact_information.address_1,
                agent_table.get("address_1"),
            );
            Self::set_option_value(
                &mut config.builder_layout.contact_information.address_2,
                agent_table.get("address_2"),
            );
            Self::set_value(
                &mut config.builder_layout.contact_information.email_address,
                agent_table.get("email_address"),
            );
        }

        if let Some(project_table) = table.get("project") {
            Self::set_value(
                &mut config.project_config.base_directory,
                project_table.get("base_directory"),
            );
            Self::set_value(
                &mut config.project_config.draft_directory,
                project_table.get("draft_directory"),
            );
            Self::set_value(
                &mut config.project_config.config_path,
                project_table.get("config_path"),
            );
            Self::set_value(
                &mut config.project_config.output_path,
                project_table.get("output_path"),
            );
            Self::set_value(
                &mut config.project_config.context_path,
                project_table.get("context_path"),
            );
        }

        Ok(config)
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use std::env;

    #[test]
    fn builds_story_model() {
        let mock_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/mock/draft";
        let result = FileHandler::build_story(mock_path);
        assert!(result.is_ok());
    }

    #[test]
    fn fetches_context() {
        let mock_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/mock/Context.toml";
        let result = FileHandler::build_context(mock_path);
        assert!(result.is_ok());
    }
}
