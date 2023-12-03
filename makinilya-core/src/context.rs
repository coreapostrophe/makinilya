use std::collections::HashMap;

pub enum Data {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, Data>),
}

pub struct Context {
    pub variables: HashMap<String, Data>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn variables(&self) -> &HashMap<String, Data> {
        &self.variables
    }
}
