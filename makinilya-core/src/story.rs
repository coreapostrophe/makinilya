//! Structs for representing the written narrative.

/// Data structure that represents the story.
/// 
/// Parts are organizational sections of the stories,
/// also depicted as Chapters, or Acts. The content are
/// the actual scenes within such parts. They contain the
/// actual narrative. The whole story is built from a 
/// combination of parts and content. 
/// 
/// 
/// # Examples
/// ```
/// use makinilya_core::story::Story;
/// 
/// let mut story = Story::new("Root");
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
}
