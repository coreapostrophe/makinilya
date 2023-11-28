use std::{
    fs::{self, DirEntry},
    path::Path,
};

use regex::Regex;

#[derive(Debug)]
pub struct FileHandler<'a> {
    base_directory: &'a Path,
    file_regex: Regex,
    files: Vec<DirEntry>,
}

impl<'a> FileHandler<'a> {
    pub fn new() -> Self {
        Self {
            base_directory: Path::new("./src"),
            file_regex: Regex::new(r"^\S+\.tw$").unwrap(),
            files: vec![],
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        self.fetch_directory(self.base_directory)?;
        Ok(())
    }

    pub fn set_base_directory(&mut self, base_directory: &'a str) {
        self.base_directory = Path::new(base_directory);
    }

    fn fetch_directory(&mut self, path: &Path) -> std::io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let entry_path = entry.path();
                    self.fetch_directory(entry_path.as_path())?;
                } else if let Some(file_name) = entry.file_name().to_str() {
                    if self.file_regex.is_match(file_name) {
                        self.files.push(entry);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod files_tests {
    use super::*;

    #[test]
    fn fetches_files() {
        let mut file_handler = FileHandler::new();
        assert!(file_handler.init().is_ok());
    }
}
