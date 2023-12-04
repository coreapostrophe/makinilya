use std::path::PathBuf;

use makinilya_text::{MakinilyaText, Rule};
use thiserror::Error;

use crate::{
    context::{Context, Data},
    files::FileHandler,
    story::Story,
};

#[derive(Error, Debug)]
pub enum MakinilyaError {
    #[error("[FileHandler Error]: {0}")]
    FileHandlerException(String),

    #[error("[Parser Error]: {0}")]
    ParserError(String),
}

pub struct MakinilyaConfig {
    base_directory: PathBuf,
}

#[derive(Debug)]
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
            .map_err(|error| MakinilyaError::FileHandlerException(error.to_string()))?;
        let story = FileHandler::build_story(story_directory)
            .map_err(|error| MakinilyaError::FileHandlerException(error.to_string()))?;

        Ok(Self { story, context })
    }

    pub fn interpolate(&mut self) -> Result<(), MakinilyaError> {
        Self::interpolate_content(&mut self.story, &self.context)?;
        Ok(())
    }

    fn interpolate_content(story: &mut Story, context: &Context) -> Result<(), MakinilyaError> {
        let mut interpolated_source = String::new();

        match story {
            Story::Part { children, .. } => {
                for mut child in children {
                    Self::interpolate_content(&mut child, context)?;
                }
            }
            Story::Content { source, .. } => {
                let parsed_source = MakinilyaText::parse(&source)
                    .map_err(|error| MakinilyaError::ParserError(error.to_string()))?
                    .next()
                    .unwrap();

                for expression in parsed_source.into_inner() {
                    if let Some(expression_value) = expression.into_inner().next() {
                        match expression_value.as_rule() {
                            Rule::string_interpolation => {
                                let identifier_array: Vec<String> = expression_value
                                    .into_inner()
                                    .next()
                                    .unwrap()
                                    .into_inner()
                                    .map(|pair| pair.as_str().to_string())
                                    .collect();

                                let mut data = context.variables().get(&identifier_array[0]);

                                for identifier in &identifier_array[1..] {
                                    if let Some(unwrapped_data) = data {
                                        match unwrapped_data {
                                            Data::Object(object_value) => {
                                                data = object_value.get(identifier);
                                            }
                                            _ => (),
                                        }
                                    }
                                }

                                if let Some(unwrapped_data) = data {
                                    interpolated_source.push_str(&unwrapped_data.to_string());
                                }
                            }
                            Rule::text_content => {
                                interpolated_source.push_str(expression_value.as_str());
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        match story {
            Story::Content { source, .. } => *source = interpolated_source,
            _ => (),
        }

        Ok(())
    }

    pub fn story_model(&self) -> &Story {
        &self.story
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
        let result = MakinilyaCore::init(MakinilyaConfig {
            base_directory: PathBuf::from("./mock"),
        });
        assert!(result.is_ok());
    }

    #[test]
    fn interpolates_story() {
        let mut core = MakinilyaCore::init(MakinilyaConfig {
            base_directory: PathBuf::from("./mock"),
        })
        .unwrap();
        assert!(core.interpolate().is_ok());
    }
}
