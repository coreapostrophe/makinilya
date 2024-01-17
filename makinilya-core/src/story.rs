//! Structs for representing the written narrative.

use crate::files::{Directory, PathItem};

pub const MAKINILYA_TEXT_EXTENSION: &str = "mt";

/// Data structure that represents the story.
///
/// - `Parts` are organizational sections of the story, they
/// can also be called as Chapters, or Acts.
/// - `Contents` are the actual scenes within such parts, and
/// contains the actual narrative.
///
/// The whole story, itself, is a part that we conventionally
/// entitle "root". It comprises a combination of parts and
/// contents.
///
///
/// # Examples
/// ```
/// use makinilya_core::story::Story;
///
/// let mut story = Story::new("root");
///
/// let mut part = Story::new("Chapter 1");
/// part.push_content("I'm the first scene.");
/// part.push_content("I'm the second scene.");
///
/// story.push_part(part);
/// ```
#[derive(Debug, Clone)]
pub struct Story {
    title: String,
    parts: Vec<Box<Story>>,
    contents: Vec<String>,
}

impl Story {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            parts: vec![],
            contents: vec![],
        }
    }

    pub fn push_part(&mut self, part: Story) {
        self.parts.push(Box::new(part));
    }

    pub fn push_content(&mut self, source: impl Into<String>) {
        self.contents.push(source.into());
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn parts(&self) -> &Vec<Box<Story>> {
        &self.parts
    }

    pub fn mut_parts(&mut self) -> &mut Vec<Box<Story>> {
        &mut self.parts
    }

    pub fn contents(&self) -> &Vec<String> {
        &self.contents
    }

    pub fn mut_contents(&mut self) -> &mut Vec<String> {
        &mut self.contents
    }

    pub fn parse(directory: &Directory) -> Self {
        let mut story = Self::new(directory.name());

        for item in directory.contents() {
            match item {
                PathItem::Directory(directory) => {
                    let nested_story = Self::parse(directory);
                    story.push_part(nested_story);
                }
                PathItem::File(file) => {
                    if let Some(extension) = &file.extension {
                        if extension == MAKINILYA_TEXT_EXTENSION {
                            let string_content = String::from_utf8_lossy(&file.content);
                            story.push_content(string_content);
                        }
                    }
                }
            }
        }

        story
    }
}
