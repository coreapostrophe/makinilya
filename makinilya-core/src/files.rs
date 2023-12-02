use std::{
    fs::{self},
    io,
    path::Path,
};

use thiserror::Error;

use crate::story::StoryModel;

#[derive(Error, Debug)]
pub enum FileHandlerError {
    #[error("path `{0}` is not a valid directory.")]
    InvalidDirectory(String),

    #[error("an io exception occurred: {0}")]
    IoException(io::Error),

    #[error("stripped prefix not found")]
    PrefixNotFound,
}

#[derive(Debug)]
pub struct FileHandler<'a> {
    base_directory: &'a Path,
    story_model: Option<StoryModel>,
}

impl<'a> FileHandler<'a> {
    pub fn new() -> Self {
        Self {
            base_directory: Path::new("./"),
            story_model: None,
        }
    }

    pub fn base_directory(&self) -> &'a Path {
        self.base_directory
    }

    pub fn story_model(&self) -> Option<&StoryModel> {
        self.story_model.as_ref()
    }

    pub fn init(&mut self) -> Result<(), FileHandlerError> {
        let mut story_model = StoryModel::new_part("root");

        let mut base_directory_buf = self.base_directory.to_path_buf();
        base_directory_buf.push("draft");

        self.build_story(base_directory_buf.as_path(), &mut story_model)?;
        self.story_model = Some(story_model);
        Ok(())
    }

    pub fn set_base_directory(&mut self, base_directory: &'a str) {
        self.base_directory = Path::new(base_directory);
    }

    fn build_story(
        &mut self,
        path: &Path,
        partition: &mut StoryModel,
    ) -> Result<(), FileHandlerError> {
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
                    self.build_story(entry_path, &mut nested_story_model)?;
                    partition.push(nested_story_model);
                } else if let Some(extension) = entry_path.extension() {
                    if extension == "mt" {
                        let source = fs::read_to_string(entry_path)
                            .map_err(|error| FileHandlerError::IoException(error))?;

                        partition.push(StoryModel::new_content(object_name, &source))
                    }
                }
            }
        }
        Ok(())
    }
}
