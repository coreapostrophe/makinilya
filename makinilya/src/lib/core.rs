//! Handles all mainline operations of the application.
//!
//! # Operations
//! - [`MakinilyaCore::build()`] - Builds the output manuscript from the project.
//! - [`MakinilyaCore::new()`] - Creates a new project.
//! - [`MakinilyaCore::check()`] - Checks all identifiers accessible within the project.

use std::{fs, io::Write, path::PathBuf};

use colored::Colorize;
use thiserror::Error;

#[allow(unused_imports)]
use crate::{
    builder::{BuilderError, ManuscriptBuilder},
    config::{Config, ConfigError, ProjectConfig},
    context::{Context, ContextError},
    extensions::CloneOnSome,
    files::ReaderError,
    interpolator::StoryInterpolator,
    story::Story,
};

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("[Io Error]: {0}")]
    Io(#[from] std::io::Error),

    #[error("[FileHandler Error]: {0}")]
    Reader(#[from] ReaderError),

    #[error("[Parser Error]: {0}")]
    Parser(#[from] makinilya_text::Error),

    #[error("[Config Error]: {0}")]
    Config(#[from] ConfigError),

    #[error("[Context Error]: {0}")]
    Context(#[from] ContextError),

    #[error("[Builder Error]: {0}")]
    Builder(#[from] BuilderError),

    #[error("[Packing Error]: {0}")]
    Zipper(#[from] zip::result::ZipError),
}

/// Encapsulates all static functions of the application's core commands.
#[derive(Debug)]
pub struct MakinilyaCore;

impl MakinilyaCore {
    const CONFIG_FILE_NAME: &'static str = "Config.toml";
    const CONTEXT_FILE_NAME: &'static str = "Context.toml";
    const DEFAULT_DRAFT_DIRECTORY: &'static str = "draft";
    const DEFAULT_OUTPUT_PATH: &'static str = "out/manuscript.docx";
    const DEFAULT_SCENE: &'static str = r#"Hi, my name is {{ names.mc }}."#;
    const DEFAULT_CONTEXT: &'static str = r#"[names]
mc = "Core"
"#;
    const DEFAULT_CONFIG: &'static str = r#"[project]
draft_directory = "draft"
output_path = "out/manuscript.docx"

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

    fn handle_directory(directory: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        let directory: PathBuf = directory.into();
        if !directory.exists() {
            fs::create_dir_all(&directory)?;
        }
        Ok(())
    }

    fn init_config(path: impl Into<PathBuf>) -> Result<Config, Error> {
        let mut config_path = path.into();
        config_path.push(Self::CONFIG_FILE_NAME);
        Ok(Config::read(config_path)?)
    }

    fn init_context(path: impl Into<PathBuf>) -> Result<Context, Error> {
        let mut context_path = path.into();
        context_path.push(Self::CONTEXT_FILE_NAME);
        Ok(Context::read(context_path)?)
    }

    fn init_story(path: impl Into<PathBuf>, config: &Config) -> Result<Story, Error> {
        let mut draft_directory = path.into();

        draft_directory.push(match &config.project {
            Some(project_config) => project_config
                .draft_directory
                .as_ref()
                .clone_on_some(Self::DEFAULT_DRAFT_DIRECTORY.into()),
            None => Self::DEFAULT_DRAFT_DIRECTORY.into(),
        });

        Self::handle_directory(&draft_directory)?;

        Ok(Story::read(draft_directory)?)
    }

    /// The manuscript will be built within the path provided in the `output_path` of the
    /// `Config.toml`. Refer to [`ProjectConfig`] for more information.
    pub fn build(path: impl Into<PathBuf>) -> Result<(), Error> {
        let path_buf: PathBuf = path.into();

        let config = Self::init_config(path_buf.clone())?;
        let story = Self::init_story(path_buf.clone(), &config)?;
        let context = Self::init_context(path_buf.clone())?;

        let interpolated_story = StoryInterpolator::interpolate(&story, &context)?;

        let builder = ManuscriptBuilder::new(&config);
        let manuscript_document = builder.build_docx(&interpolated_story)?;

        let mut output_path = path_buf;

        output_path.push(match &config.project {
            Some(project_config) => project_config
                .output_path
                .as_ref()
                .clone_on_some(Self::DEFAULT_OUTPUT_PATH.into()),
            None => Self::DEFAULT_OUTPUT_PATH.into(),
        });

        let mut output_directory = output_path.clone();
        output_directory.pop();

        Self::handle_directory(&output_directory)?;

        let file = fs::File::create(&output_path)?;
        manuscript_document.build().pack(file)?;

        println!(
            "{}{} final manuscript ({})\n",
            " ".repeat(3),
            "Built".green().bold(),
            output_path.canonicalize()?.to_string_lossy()
        );

        Ok(())
    }

    /// Creates project files from directory path. The resulting project will have a defaulted
    /// `Config.toml` and `Context.toml` files, as well as a scene and chapter.
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

        Self::handle_directory(chapter_directory)?;

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

    /// Checks whether or not there are any missing variables within the project's `Context.toml`.
    pub fn check(path: impl Into<PathBuf>) -> Result<(), Error> {
        let path_buf: PathBuf = path.into();
        let config = Self::init_config(path_buf.clone())?;
        let story = Self::init_story(path_buf, &config)?;

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
        let path = std::env::current_dir().unwrap();
        let result = MakinilyaCore::build(path.join("mock/01-standard-project"));
        assert!(result.is_ok());
    }

    #[test]
    fn new_project() {
        let path = std::env::current_dir().unwrap();
        let result = MakinilyaCore::new(path.join("mock/02-new-project"));
        assert!(result.is_ok());
    }

    #[test]
    fn check_project() {
        let path = std::env::current_dir().unwrap();
        let result = MakinilyaCore::check(path.join("mock/01-standard-project"));
        assert!(result.is_ok());
    }
}
