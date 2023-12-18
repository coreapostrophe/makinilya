use std::path::PathBuf;

use thiserror::Error;
use toml::{Table, Value};

use crate::builder::ManuscriptBuilderLayout;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    ParsingError(#[from] toml::de::Error),
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub base_directory: PathBuf,
    pub draft_directory: PathBuf,
    pub config_path: PathBuf,
    pub output_path: PathBuf,
    pub context_path: PathBuf,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            base_directory: "./".into(),
            draft_directory: "draft".into(),
            config_path: "Config.toml".into(),
            output_path: "./out/manuscript.docx".into(),
            context_path: "Context.toml".into(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub project: ProjectConfig,
    pub builder: ManuscriptBuilderLayout,
}

impl Config {
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

    pub fn parse(source: &str) -> Result<Self, ConfigError> {
        let table = source.parse::<Table>()?;
        let mut config = Config::default();

        if let Some(story_table) = table.get("story") {
            Self::set_value(&mut config.builder.title, story_table.get("title"));
            Self::set_value(&mut config.builder.pen_name, story_table.get("pen_name"));
        }

        if let Some(author_table) = table.get("author") {
            Self::set_value(
                &mut config.builder.contact_information.name,
                author_table.get("name"),
            );
            Self::set_option_value(
                &mut config.builder.contact_information.mobile_number,
                author_table.get("mobile_number"),
            );
            Self::set_value(
                &mut config.builder.contact_information.address_1,
                author_table.get("address_1"),
            );
            Self::set_option_value(
                &mut config.builder.contact_information.address_2,
                author_table.get("address_2"),
            );
            Self::set_value(
                &mut config.builder.contact_information.email_address,
                author_table.get("email_address"),
            );
        }

        if let Some(agent_table) = table.get("agent") {
            Self::set_value(
                &mut config.builder.contact_information.name,
                agent_table.get("name"),
            );
            Self::set_option_value(
                &mut config.builder.contact_information.mobile_number,
                agent_table.get("mobile_number"),
            );
            Self::set_value(
                &mut config.builder.contact_information.address_1,
                agent_table.get("address_1"),
            );
            Self::set_option_value(
                &mut config.builder.contact_information.address_2,
                agent_table.get("address_2"),
            );
            Self::set_value(
                &mut config.builder.contact_information.email_address,
                agent_table.get("email_address"),
            );
        }

        if let Some(project_table) = table.get("project") {
            Self::set_value(
                &mut config.project.base_directory,
                project_table.get("base_directory"),
            );
            Self::set_value(
                &mut config.project.draft_directory,
                project_table.get("draft_directory"),
            );
            Self::set_value(
                &mut config.project.config_path,
                project_table.get("config_path"),
            );
            Self::set_value(
                &mut config.project.output_path,
                project_table.get("output_path"),
            );
            Self::set_value(
                &mut config.project.context_path,
                project_table.get("context_path"),
            );
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: Default::default(),
            builder: Default::default(),
        }
    }
}
