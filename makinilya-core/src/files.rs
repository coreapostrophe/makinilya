use std::{
    fs::{self, DirEntry},
    path::Path,
};

#[derive(Debug)]
pub struct FileHandler<'a> {
    base_directory: &'a Path,
    files: Vec<DirEntry>,
}

impl<'a> FileHandler<'a> {
    pub fn new() -> Self {
        Self {
            base_directory: Path::new("./src"),
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
                    self.fetch_directory(entry.path().as_path())?;
                } else if let Some(extension) = entry.path().extension() {
                    if extension == "md" {
                        self.files.push(entry);
                    }
                }
            }
        }
        Ok(())
    }
}
