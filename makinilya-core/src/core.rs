use std::path::PathBuf;

use thiserror::Error;

use crate::{
    context::Context,
    files::{FileHandler, FileHandlerError},
    story::Story,
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
    story: Story,
    context: Context,
}

impl MakinilyaCore {
    pub fn init(config: MakinilyaConfig) -> Result<Self, MakinilyaError> {
        let mut context_path = config.base_directory.clone();
        context_path.push("Context.toml");
        let mut story_directory = config.base_directory.clone();
        story_directory.push("draft");

        let context = FileHandler::build_context(context_path)
            .map_err(|error| MakinilyaError::FileHandlerException(error))?;
        let story = FileHandler::build_story(story_directory)
            .map_err(|error| MakinilyaError::FileHandlerException(error))?;

        Ok(Self { story, context })
    }

    pub fn story_model(&self) -> &Story {
        &self.story
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
