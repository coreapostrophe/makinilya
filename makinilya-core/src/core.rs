//! Handles all mainline operations of the application

use std::{fs, path::PathBuf};

use makinilya_text::{MakinilyaText, Rule};
use pest::iterators::Pair;
use thiserror::Error;

use crate::{
    builder::{ManuscriptBuilder, ManuscriptBuilderLayout},
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

#[derive(Debug)]
pub struct Config {
    pub base_directory: PathBuf,
    pub draft_directory: PathBuf,
    pub output_path: PathBuf,
    pub context_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_directory: "./".into(),
            draft_directory: "draft".into(),
            output_path: "./out/manuscript.docx".into(),
            context_path: "Context.toml".into(),
        }
    }
}

/// A struct for initializing the script.
///
/// Consumes a configuration and contains static functions and
/// methods that not only builds the `Story` and `Context` from
/// provided paths, but also the docx manuscript.
///
/// # Examples
/// ```
/// use makinilya_core::core::MakinilyaCore;
/// 
/// let story = MakinilyaCore::init(Config {
///    base_directory: PathBuf::from("./mock"),
///    ..Default::default()
/// });
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
    /// Initializes the story and context of the project.
    /// 
    /// This function consumes the path provided by the configuration
    /// and builds a `Story` and `Context` struct out of them. 
    pub fn init(config: Config) -> Result<Self, MakinilyaError> {
        let mut context_path = config.base_directory.clone();
        context_path.push(&config.context_path);
        let mut story_directory = config.base_directory.clone();
        story_directory.push(&config.draft_directory);

        Self::handle_directory(&context_path);
        Self::handle_directory(&story_directory);

        let context = FileHandler::build_context(context_path)
            .map_err(|error| MakinilyaError::FileHandlerException(error.to_string()))?;
        let story = FileHandler::build_story(story_directory)
            .map_err(|error| MakinilyaError::FileHandlerException(error.to_string()))?;

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
    pub fn build(&mut self, builder_layout: ManuscriptBuilderLayout) -> Result<(), MakinilyaError> {
        let interpolated_story = Self::interpolate_story(&mut self.story, &self.context)?;
        let builder = ManuscriptBuilder::new(builder_layout);
        let manuscript_document = builder.build(&interpolated_story).unwrap();

        let mut output_path = self.config.base_directory.clone();
        output_path.push(&self.config.output_path);

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

    fn interpolate_story(story: &mut Story, context: &Context) -> Result<Story, MakinilyaError> {
        let mut interpolated_story = Story::new(story.title());

        for content in story.mut_contents() {
            let parsed_source = MakinilyaText::parse(&content)
                .map_err(|error| MakinilyaError::ParserError(error.to_string()))?
                .next()
                .unwrap();
            let expressions = parsed_source.into_inner();

            let interpolated_expressions: Vec<String> = expressions
                .map(|expression| Self::interpolate_expression(expression, context))
                .collect();

            interpolated_story.push_content(interpolated_expressions.join(""));
        }

        for part in story.mut_parts() {
            let interpolated_part = Self::interpolate_story(part, context)?;
            interpolated_story.push_part(interpolated_part);
        }

        Ok(interpolated_story)
    }

    fn interpolate_expression(expression: Pair<'_, Rule>, context: &Context) -> String {
        let mut result = String::new();

        if let Some(expression_value) = expression.into_inner().next() {
            match expression_value.as_rule() {
                Rule::string_interpolation => {
                    let mut identifier_array = expression_value
                        .into_inner()
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|pair| pair.as_str());

                    let first_identifier = identifier_array.next().unwrap();
                    let mut data = context.variables().get(first_identifier);

                    while let Some(identifier) = identifier_array.next() {
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
                        result.push_str(&unwrapped_data.to_string());
                    }
                }
                Rule::text_content => {
                    result.push_str(expression_value.as_str());
                }
                _ => (),
            }
        }

        result
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
        let result = MakinilyaCore::init(Config {
            base_directory: PathBuf::from("./mock"),
            ..Default::default()
        });

        assert!(result.is_ok());
    }

    #[test]
    fn builds_manuscript() {
        let result = MakinilyaCore::init(Config {
            base_directory: PathBuf::from("./mock"),
            ..Default::default()
        });
        assert!(result
            .unwrap()
            .build(ManuscriptBuilderLayout::default())
            .is_ok());
    }
}
