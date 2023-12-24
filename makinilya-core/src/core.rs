//! Handles all mainline operations of the application

use std::{fs, path::PathBuf};

use thiserror::Error;

use crate::{
    builder::ManuscriptBuilder,
    config::Config,
    context::Context,
    extensions::CloneOnSome,
    files::{FileHandler, FileHandlerError},
    interpolator::StoryInterpolator,
    story::Story,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("[FileHandler Error]: {0}")]
    FileHandlerError(#[from] FileHandlerError),

    #[error("[Parser Error]: {0}")]
    ParserError(#[from] makinilya_text::Error),
}

/// A struct for initializing the script.
///
/// Consumes a configuration and contains static functions and
/// methods that not only builds the `Story` and `Context` from
/// provided paths, but also the docx manuscript.
///
/// # Examples
/// ```
/// use makinilya_core::{
///     core::MakinilyaCore,
///     config::{Config, ProjectConfig}
/// };
/// use std::path::PathBuf;
///
/// let story = MakinilyaCore::init("./mock");
///
/// assert!(story.is_ok());
/// ```
#[derive(Debug)]
pub struct MakinilyaCore {
    story: Story,
    context: Context,
    config: Config,
}

impl MakinilyaCore {
    pub const DEFAULT_BASE_DIRECTORY: &str = "./";
    pub const DEFAULT_DRAFT_DIRECTORY: &str = "draft";
    pub const DEFAULT_CONTEXT_PATH: &str = "Context.toml";
    pub const DEFAULT_CONFIG_PATH: &str = "Config.toml";
    pub const DEFAULT_OUTPUT_PATH: &str = "./out/manuscript.docx";

    /// Initializes the story and context of the project.
    ///
    /// This function consumes the path provided by the configuration
    /// and builds a `Story` and `Context` struct out of them.
    pub fn init(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let config_path = {
            let mut path: PathBuf = path.into();
            path.push(Self::DEFAULT_CONFIG_PATH);
            path
        };

        let config = FileHandler::build_config(config_path)?;

        let base_directory = match &config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(Self::DEFAULT_BASE_DIRECTORY.into()),
            None => Self::DEFAULT_BASE_DIRECTORY.into(),
        };
        let context_path = {
            let context_path = match &config.project {
                Some(project_config) => project_config
                    .context_path
                    .as_ref()
                    .clone_on_some(Self::DEFAULT_CONTEXT_PATH.into()),
                None => Self::DEFAULT_CONTEXT_PATH.into(),
            };

            let mut path = base_directory.clone();
            path.push(context_path);

            path
        };
        let draft_directory = {
            let draft_directory = match &config.project {
                Some(project_config) => project_config
                    .draft_directory
                    .as_ref()
                    .clone_on_some(Self::DEFAULT_DRAFT_DIRECTORY.into()),
                None => Self::DEFAULT_DRAFT_DIRECTORY.into(),
            };

            let mut path = base_directory.clone();
            path.push(draft_directory);

            path
        };

        Self::handle_directory(&draft_directory);

        let context = FileHandler::build_context(context_path)?;
        let story = FileHandler::build_story(draft_directory)?;

        Ok(Self {
            story,
            context,
            config,
        })
    }

    /// Interpolates the story and builds the manuscript
    ///
    /// The story of the project is interpolated with the context
    /// variables to create its final draft. The core then passes the
    /// interpolated story to the builder which then creates the
    /// docx file. Afterwards, the document is written to a system
    /// file based on the path provided from the configuration.
    pub fn build(&self) -> Result<(), Error> {
        let interpolated_story = StoryInterpolator::interpolate(&self.story, &self.context)?;

        let builder = ManuscriptBuilder::new(&self.config);
        let manuscript_document = builder.build(&interpolated_story).unwrap();

        let base_directory = match &self.config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(Self::DEFAULT_BASE_DIRECTORY.into()),
            None => Self::DEFAULT_BASE_DIRECTORY.into(),
        };
        let output_path = {
            let output_path = match &self.config.project {
                Some(project_config) => project_config
                    .output_path
                    .as_ref()
                    .clone_on_some(Self::DEFAULT_OUTPUT_PATH.into()),
                None => Self::DEFAULT_OUTPUT_PATH.into(),
            };

            let mut path = base_directory.clone();
            path.push(output_path);

            path
        };

        let mut output_directory = output_path.clone();
        output_directory.pop();

        Self::handle_directory(&output_directory);

        let file = fs::File::create(&output_path).unwrap();
        manuscript_document.build().pack(file).unwrap();

        Ok(())
    }

    fn handle_directory(path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
    }

    pub fn story(&self) -> &Story {
        &self.story
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;

    #[test]
    fn extracts_story_and_context() {
        let result = MakinilyaCore::init("./mock");
        assert!(result.is_ok());
    }

    #[test]
    fn builds_manuscript() {
        let result = MakinilyaCore::init("./mock");
        assert!(result.unwrap().build().is_ok());
    }
}
