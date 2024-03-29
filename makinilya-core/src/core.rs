//! Handles all mainline operations of the application

use std::{fs, io::Write, path::PathBuf};

use colored::Colorize;
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
    IoError(#[from] std::io::Error),

    #[error("[FileHandler Error]: {0}")]
    FileHandlerError(#[from] FileHandlerError),

    #[error("[Parser Error]: {0}")]
    ParserError(#[from] makinilya_text::Error),
}

/// A struct that contains all static functions for the core commands of the crate.
///
/// # Examples
/// ```no_run
/// use makinilya_core::{
///     core::MakinilyaCore,
///     config::{Config, ProjectConfig}
/// };
/// use std::path::PathBuf;
///
/// let story = MakinilyaCore::build("./sample-project");
///
/// assert!(story.is_ok());
/// ```
#[derive(Debug)]
pub struct MakinilyaCore;

impl MakinilyaCore {
    pub const DEFAULT_BASE_DIRECTORY: &'static str = "./";
    pub const DEFAULT_DRAFT_DIRECTORY: &'static str = "draft";
    pub const DEFAULT_CONTEXT_PATH: &'static str = "Context.toml";
    pub const DEFAULT_CONFIG_PATH: &'static str = "Config.toml";
    pub const DEFAULT_OUTPUT_PATH: &'static str = "./out/manuscript.docx";
    pub const DEFAULT_SCENE: &'static str = r#"Hi, my name is {{ names.mc }}.
"#;
    pub const DEFAULT_CONTEXT: &'static str = r#"[names]
mc = "Core"
"#;
    pub const DEFAULT_CONFIG: &'static str = r#"[project]
base_directory = "./"
draft_directory = "draft"
output_path = "./out/manuscript.docx"
context_path = "Context.toml"

[story]
title = "Untitled"
pen_name = "Brutus Ellis"

[author]
name = "Brutus Ellis"
address_1 = "2688 South Avenue"
address_2 = "Barangay Olympia, Makati City"
mobile_number = "+63 895 053 4757"
email_address = "brutusellis@email.com"

[agent]
name = "Cymone Sabina"
address_1 = "755 Maria Clara Street"
address_2 = "Mandaluyong City"
mobile_number = "+63 908 524 4125"
email_address = "cymonesabina.@email.com"
"#;

    fn handle_directory(directory: impl Into<PathBuf>) {
        let directory: PathBuf = directory.into();
        if !directory.exists() {
            fs::create_dir_all(&directory).unwrap();
        }
    }

    fn init_config(path: impl Into<PathBuf>) -> Result<Config, Error> {
        let config_path = {
            let mut path: PathBuf = path.into();
            path.push(Self::DEFAULT_CONFIG_PATH);
            path
        };
        Ok(FileHandler::build_config(config_path)?)
    }

    fn init_context(config: &Config) -> Result<Context, Error> {
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

        Ok(FileHandler::build_context(context_path)?)
    }

    fn init_story(config: &Config) -> Result<Story, Error> {
        let base_directory = match &config.project {
            Some(project_config) => project_config
                .base_directory
                .as_ref()
                .clone_on_some(Self::DEFAULT_BASE_DIRECTORY.into()),
            None => Self::DEFAULT_BASE_DIRECTORY.into(),
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
        Ok(FileHandler::build_story(draft_directory)?)
    }

    /// Interpolates the story and builds the manuscript
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
                .clone_on_some(Self::DEFAULT_BASE_DIRECTORY.into()),
            None => Self::DEFAULT_BASE_DIRECTORY.into(),
        };
        let output_path = {
            let output_path = match &config.project {
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

        println!(
            "{}{} final manuscript ({})\n",
            " ".repeat(3),
            "Built".green().bold(),
            output_path.canonicalize()?.to_string_lossy()
        );

        Ok(())
    }

    /// Creates a new project
    pub fn new(path: impl Into<PathBuf>) -> Result<(), Error> {
        let base_directory: PathBuf = path.into();

        let chapter_directory = {
            let mut directory = base_directory.clone();
            directory.push(Self::DEFAULT_DRAFT_DIRECTORY);
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
            let mut path = base_directory.clone();
            path.push("Config.toml");
            path
        };

        Self::handle_directory(chapter_directory);

        let mut scene_file = fs::File::create(scene_path)?;
        scene_file.write_all(Self::DEFAULT_SCENE.as_bytes())?;

        let mut context_file = fs::File::create(context_path)?;
        context_file.write_all(Self::DEFAULT_CONTEXT.as_bytes())?;

        let mut config_file = fs::File::create(config_path)?;
        config_file.write_all(Self::DEFAULT_CONFIG.as_bytes())?;

        println!(
            "{}{} makinilya project ({})\n",
            " ".repeat(3),
            "Created".green().bold(),
            base_directory.canonicalize()?.to_string_lossy()
        );

        Ok(())
    }

    /// Lists all existing identifiers in project
    pub fn check(path: impl Into<PathBuf>) -> Result<(), Error> {
        let config = Self::init_config(path)?;
        let story = Self::init_story(&config)?;

        let checked_story = StoryInterpolator::check(&story)?;

        println!("{}{}", " ".repeat(3), "Identifiers".green().bold());

        for identifier in checked_story {
            println!("{}{}", " ".repeat(6), identifier);
        }

        println!("");

        Ok(())
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;

    #[test]
    fn builds_manuscript() {
        let result = MakinilyaCore::build("./mock/01-standard-project");
        assert!(result.is_ok());
    }

    #[test]
    fn new_project() {
        let result = MakinilyaCore::new("./mock/02-new-project");
        assert!(result.is_ok());
    }

    #[test]
    fn check_project() {
        let result = MakinilyaCore::check("./mock/01-standard-project");
        assert!(result.is_ok());
    }
}
