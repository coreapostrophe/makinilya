use std::path::PathBuf;

use crate::builder::ManuscriptBuilderLayout;

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

impl Default for Config {
    fn default() -> Self {
        Self {
            project: Default::default(),
            builder: Default::default(),
        }
    }
}
