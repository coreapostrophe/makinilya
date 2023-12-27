//! Handles all mainline operations of the application

use std::{fs, io::Write, path::PathBuf};

use thiserror::Error;

use crate::{
    builder::ManuscriptBuilder,
    config::Config,
    context::Context,
    defaults,
    extensions::CloneOnSome,
    files::{FileHandler, FileHandlerError},
    interpolator::StoryInterpolator,
    story::Story,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("[FileHandler Error]: {0}")]
    IoError(#[from] std::io::Error),

    #[error("[FileHandler Error]: {0}")]
    FileHandlerError(#[from] FileHandlerError),

    #[error("[Parser Error]: {0}")]
    ParserError(#[from] makinilya_text::Error),
}

/// A struct that contains all static functionality for the crate
///
/// # Examples
/// ```
/// use makinilya_core::{
///     core::MakinilyaCore,
///     config::{Config, ProjectConfig}
/// };
/// use std::path::PathBuf;
///
/// let story = MakinilyaCore::build("./mock");
///
/// assert!(story.is_ok());
/// ```
#[derive(Debug)]
pub struct MakinilyaCore;

impl MakinilyaCore {
    fn handle_directory(directory: impl Into<PathBuf>) {
        let directory: PathBuf = directory.into();
        if !directory.exists() {
            fs::create_dir_all(&directory).unwrap();
        }
    }

    fn init_config(path: impl Into<PathBuf>) -> Result<Config, Error> {
        let config_path = {
            let mut path: PathBuf = path.into();
            path.push(defaults::CONFIG_PATH);
            path
        };
        Ok(FileHandler::build_config(config_path)?)
    }

    fn init_context(config: &Config) -> Result<Context, Error> {
        let base_directory = match &config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(defaults::BASE_DIRECTORY.into()),
            None => defaults::BASE_DIRECTORY.into(),
        };
        let context_path = {
            let context_path = match &config.project {
                Some(project_config) => project_config
                    .context_path
                    .as_ref()
                    .clone_on_some(defaults::CONTEXT_PATH.into()),
                None => defaults::CONTEXT_PATH.into(),
            };

            let mut path = base_directory.clone();
            path.push(context_path);

            path
        };

        Ok(FileHandler::build_context(context_path)?)
    }

    fn init_story(config: &Config) -> Result<Story, Error> {
        let base_directory = match &config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(defaults::BASE_DIRECTORY.into()),
            None => defaults::BASE_DIRECTORY.into(),
        };
        let draft_directory = {
            let draft_directory = match &config.project {
                Some(project_config) => project_config
                    .draft_directory
                    .as_ref()
                    .clone_on_some(defaults::DRAFT_DIRECTORY.into()),
                None => defaults::DRAFT_DIRECTORY.into(),
            };

            let mut path = base_directory.clone();
            path.push(draft_directory);

            path
        };

        Self::handle_directory(&draft_directory);
        Ok(FileHandler::build_story(draft_directory)?)
    }

    /// Interpolates the story and builds the manuscript
    ///
    /// The story of the project is interpolated with the context
    /// variables to create its final draft. The core then passes the
    /// interpolated story to the builder which then creates the
    /// docx file. Afterwards, the document is written to a system
    /// file based on the path provided from the configuration.
    pub fn build(path: impl Into<PathBuf>) -> Result<(), Error> {
        let config = Self::init_config(path)?;
        let story = Self::init_story(&config)?;
        let context = Self::init_context(&config)?;

        let interpolated_story = StoryInterpolator::interpolate(&story, &context)?;

        let builder = ManuscriptBuilder::new(&config);
        let manuscript_document = builder.build(&interpolated_story).unwrap();

        let base_directory = match &config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(defaults::BASE_DIRECTORY.into()),
            None => defaults::BASE_DIRECTORY.into(),
        };
        let output_path = {
            let output_path = match &config.project {
                Some(project_config) => project_config
                    .output_path
                    .as_ref()
                    .clone_on_some(defaults::OUTPUT_PATH.into()),
                None => defaults::OUTPUT_PATH.into(),
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

    pub fn new(path: impl Into<PathBuf>) -> Result<(), Error> {
        let base_directory: PathBuf = path.into();

        let chapter_directory = {
            let mut directory = base_directory.clone();
            directory.push(defaults::DRAFT_DIRECTORY);
            directory.push("Chapter 1");
            directory
        };
        let scene_path = {
            let mut path = chapter_directory.clone();
            path.push("Scene 1.mt");
            path
        };
        let context_path = {
            let mut path = base_directory.clone();
            path.push("Context.toml");
            path
        };
        let config_path = {
            let mut path = base_directory;
            path.push("Config.toml");
            path
        };

        Self::handle_directory(chapter_directory);

        let mut scene_file = fs::File::create(scene_path)?;
        scene_file.write_all(defaults::EXAMPLE_SCENE.as_bytes())?;

        let mut context_file = fs::File::create(context_path)?;
        context_file.write_all(defaults::EXAMPLE_CONTEXT.as_bytes())?;

        let mut config_file = fs::File::create(config_path)?;
        config_file.write_all(defaults::EXAMPLE_CONFIG.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;

    #[test]
    fn builds_manuscript() {
        let result = MakinilyaCore::build("./mock");
        assert!(result.is_ok());
    }

    #[test]
    fn new_manuscript() {
        let result = MakinilyaCore::new("./sample");
        assert!(result.is_ok());
    }
}
