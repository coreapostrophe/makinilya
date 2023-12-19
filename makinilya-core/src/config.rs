use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

use crate::builder::ContactInformation;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    ParsingError(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct StoryConfig {
    pub title: Option<String>,
    pub pen_name: Option<String>,
}

impl Default for StoryConfig {
    fn default() -> Self {
        Self {
            title: None,
            pen_name: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub base_directory: Option<PathBuf>,
    pub draft_directory: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub context_path: Option<PathBuf>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            base_directory: None,
            draft_directory: None,
            config_path: None,
            output_path: None,
            context_path: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub mobile_number: Option<String>,
    pub email_address: Option<String>,
}

impl Default for AuthorConfig {
    fn default() -> Self {
        Self {
            name: None,
            address_1: None,
            address_2: None,
            mobile_number: None,
            email_address: None,
        }
    }
}

impl Into<ContactInformation> for AuthorConfig {
    fn into(self) -> ContactInformation {
        ContactInformation {
            name: self.name,
            address_1: self.address_1,
            address_2: self.address_2,
            mobile_number: self.mobile_number,
            email_address: self.email_address,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentConfig {
    pub name: Option<String>,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub mobile_number: Option<String>,
    pub email_address: Option<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: None,
            address_1: None,
            address_2: None,
            mobile_number: None,
            email_address: None,
        }
    }
}

impl Into<ContactInformation> for AgentConfig {
    fn into(self) -> ContactInformation {
        ContactInformation {
            name: self.name,
            address_1: self.address_1,
            address_2: self.address_2,
            mobile_number: self.mobile_number,
            email_address: self.email_address,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub story: Option<StoryConfig>,
    pub project: Option<ProjectConfig>,
    pub author: Option<AuthorConfig>,
    pub agent: Option<AuthorConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            story: None,
            project: None,
            author: None,
            agent: None,
        }
    }
}

impl Config {
    pub fn parse(source: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(source)?)
    }
}
