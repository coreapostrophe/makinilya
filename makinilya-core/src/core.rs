use std::path::PathBuf;

use thiserror::Error;
use toml::Table;

use crate::{
    files::{FileHandler, FileHandlerError},
    story::StoryModel,
};

#[derive(Error, Debug)]
pub enum MakinilyaError {
    #[error("[FileHandler Error]: {0}")]
    FileHandlerException(FileHandlerError),
}

pub struct MakinilyaConfig {
    base_directory: PathBuf,
}

pub struct MakinilyaCore {
    story_model: StoryModel,
    context: Table,
}

impl MakinilyaCore {
    pub fn init(config: MakinilyaConfig) -> Result<Self, MakinilyaError> {
        let mut context_path = config.base_directory.clone();
        context_path.push("Context.toml");
        let mut story_directory = config.base_directory.clone();
        story_directory.push("draft");

        let context = FileHandler::fetch_context(context_path)
            .map_err(|error| MakinilyaError::FileHandlerException(error))?;
        let story_model = FileHandler::build_story_model(story_directory)
            .map_err(|error| MakinilyaError::FileHandlerException(error))?;

        Ok(Self {
            story_model,
            context,
        })
    }

    pub fn story_model(&self) -> &StoryModel {
        &self.story_model
    }

    pub fn context(&self) -> &Table {
        &self.context
    }
}
