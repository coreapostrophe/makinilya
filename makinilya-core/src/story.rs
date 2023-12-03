#[derive(Debug)]
pub enum Story {
    Content {
        title: String,
        source: String,
    },
    Part {
        title: String,
        children: Vec<Box<Story>>,
    },
}

impl Story {
    pub fn new_part(title: &str) -> Self {
        Self::Part {
            title: title.to_owned(),
            children: vec![],
        }
    }

    pub fn new_content(title: &str, content: &str) -> Self {
        Self::Content {
            title: title.to_owned(),
            source: content.to_owned(),
        }
    }

    pub fn push(&mut self, story_model: Story) {
        match self {
            Self::Part { children, .. } => children.push(Box::new(story_model)),
            _ => panic!("Tried to push to story model content."),
        }
    }
}
