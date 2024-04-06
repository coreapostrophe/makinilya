//! Handles the config of the project.
//!
//! The configuration of the project is stored within the `Config.toml` located at the root of the
//! project directory.
//!
//! # Examples
//! This is an example of content within the `Config.toml` of the project.
//! ```toml
//! [project]
//! draft_directory = "draft"
//! output_path = "out/manuscript.docx"
//! [story]
//! title = "Untitled"
//! pen_name = "Brutus Ellis"
//!
//! [author]
//! name = "Brutus Ellis"
//! address_1 = "2688 South Avenue"
//! address_2 = "Barangay Olympia, Makati City"
//! mobile_number = "+63 895 053 4757"
//! email_address = "brutusellis@email.com"
//!
//! [agent]
//! name = "Cymone Sabina"
//! address_1 = "755 Maria Clara Street"
//! address_2 = "Mandaluyong City"
//! mobile_number = "+63 908 524 4125"
//! email_address = "cymonesabina.@email.com"
//! ```

use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

#[doc(hidden)]
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    Parsing(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// General detail configurations of the manuscript.
#[derive(Debug, Deserialize, Clone)]
pub struct StoryConfig {
    /// The title of the manuscript.
    pub title: Option<String>,
    /// The pseudonym of the author that's presented on the cover.
    pub pen_name: Option<String>,
}

/// Project structure configurations of the manuscript. The paths should all be relative and must
/// not have a starting slash `/`.
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    /// The directory where the narrative scenes and chapters are contained.
    pub draft_directory: Option<PathBuf>,
    /// The path of the file where the final manuscript is built.
    pub output_path: Option<PathBuf>,
}

/// Struct representation of a person's contact information.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone)]
pub struct ContactInformation {
    pub name: Option<String>,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub mobile_number: Option<String>,
    pub email_address: Option<String>,
}

/// Collective configuration of the crate's executable.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub story: Option<StoryConfig>,
    pub project: Option<ProjectConfig>,
    pub author: Option<ContactInformation>,
    pub agent: Option<ContactInformation>,
}

#[doc(hidden)]
impl Config {
    pub fn parse(source: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(source)?)
    }

    pub fn read(path: impl Into<PathBuf>) -> Result<Self, ConfigError> {
        let file_string = std::fs::read_to_string(path.into().as_path())?;
        let config = Config::parse(&file_string)?;
        Ok(config)
    }
}
