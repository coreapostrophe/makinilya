//! Structs for creating the config of the executable.

use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    ParsingError(#[from] toml::de::Error),
}

/// General detail configurations of the manuscript.
///
/// - `title` - The title of the manuscript
/// - `pen_name` - The pseudonym of the author that's presented on the cover.
///
/// # Examples
/// ```
/// use makinilya_core::config::StoryConfig;
///
/// let story_config = StoryConfig {
///     title: Some("Untitled".into()),
///     pen_name: Some("Brutus Ellis".into())
/// };
/// ```
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

/// Project structure configurations of the manuscript
///
/// - `base_directory` - The directory that's prefixed to all other project structure paths.
/// - `draft_directory` - The directory where the narrative scenes and chapters are contained.
/// - `output_path` - The path of the file where the final manuscript is built.
/// - `context_path` - The path to the file where context of the narrative is stored.
///
/// # Examples
/// ```
/// use makinilya_core::config::ProjectConfig;
///
/// let project_config = ProjectConfig {
///     base_directory: Some("./".into()),
///     draft_directory: Some("draft".into()),
///     output_path: Some("./out/manuscript.docx".into()),
///     context_path: Some("./Context.toml".into()),
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub base_directory: Option<PathBuf>,
    pub draft_directory: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub context_path: Option<PathBuf>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            base_directory: None,
            draft_directory: None,
            output_path: None,
            context_path: None,
        }
    }
}

/// Contact information struct.
///
/// # Examples
/// ```
/// use makinilya_core::config::ContactInformation;
///
/// let contact_information = ContactInformation {
///     name: Some("Brutus Ellis".into()),
///     address_1: Some("2688 South Avenue".into()),
///     address_2: Some("Barangay Olympia, Makati City".into()),
///     mobile_number: Some("+63 895 053 4757".into()),
///     email_address: Some("brutusellis@email.com".into()),
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct ContactInformation {
    pub name: Option<String>,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub mobile_number: Option<String>,
    pub email_address: Option<String>,
}

impl Default for ContactInformation {
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

/// Collective configuration of the executable
///
/// # Examples
/// ```
/// use makinilya_core::config::{StoryConfig, ProjectConfig, ContactInformation, Config};
///
/// let story = StoryConfig {
///     title: Some("Untitled".into()),
///     pen_name: Some("Brutus Ellis".into())
/// };
///
/// let project = ProjectConfig {
///     base_directory: Some("./".into()),
///     draft_directory: Some("draft".into()),
///     output_path: Some("./out/manuscript.docx".into()),
///     context_path: Some("./Context.toml".into()),
/// };
///
/// let author = ContactInformation {
///     name: Some("Brutus Ellis".into()),
///     address_1: Some("2688 South Avenue".into()),
///     address_2: Some("Barangay Olympia, Makati City".into()),
///     mobile_number: Some("+63 895 053 4757".into()),
///     email_address: Some("brutusellis@email.com".into()),
/// };
///
/// let agent = ContactInformation {
///     name: Some("Cymone Sabina".into()),
///     address_1: Some("755 Maria Clara Street".into()),
///     address_2: Some("Mandaluyong City".into()),
///     mobile_number: Some("+63 908 524 4125".into()),
///     email_address: Some("cymonesabina.@email.com".into()),
/// };
///
/// let config = Config {
///     story: Some(story),
///     project: Some(project),
///     author: Some(author),
///     agent: Some(agent),
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub story: Option<StoryConfig>,
    pub project: Option<ProjectConfig>,
    pub author: Option<ContactInformation>,
    pub agent: Option<ContactInformation>,
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
