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
