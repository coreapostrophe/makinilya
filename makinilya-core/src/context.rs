use std::collections::HashMap;

#[derive(Debug)]
pub enum Data {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, Data>),
}

impl ToString for Data {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(boolean_value) => boolean_value.to_string(),
            Self::Number(numeric_value) => numeric_value.to_string(),
            Self::String(string_value) => string_value.to_owned(),
            Self::Object(object_value) => format!("{:?}", object_value),
        }
    }
}

#[derive(Debug)]
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
