use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoryModelError {
    #[error("Source does not exist")]
    SourceIsNone,
}

pub struct StoryModel {
    source: String,
    children: Vec<Box<StoryModel>>,
}

impl StoryModel {
    pub fn source(&self) -> &String {
        &self.source
    }
    pub fn children(&self) -> &Vec<Box<StoryModel>> {
        &self.children
    }
}

pub struct StoryModelBuilder {
    source: Option<String>,
    children: Vec<Box<StoryModel>>,
}

impl StoryModelBuilder {
    pub fn new() -> Self {
        Self {
            source: None,
            children: vec![],
        }
    }

    pub fn set_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn add_child(mut self, child: StoryModel) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn build(self) -> Result<StoryModel, StoryModelError> {
        Ok(StoryModel {
            source: self.source.ok_or(StoryModelError::SourceIsNone)?,
            children: self.children,
        })
    }
}
