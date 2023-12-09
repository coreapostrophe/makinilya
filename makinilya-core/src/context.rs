//! Structs for creating the context of the narrative.

use std::collections::HashMap;

/// Enum representation of the possible values that the `Context` could
/// store.
///
/// They are a subset of the native types supported in the [`TOML`](https://toml.io/en/v1.0.0)
/// language spec. More complex types such as `Arrays` and `DateTimes` are not included as there's
/// currently not an apparent use case for them. Though, it's possible that they'll be included
/// in the future.
///
/// # Examples
/// ```
/// use makinilya_core::context::Data;
/// use std::collections::HashMap;
///
/// let string_data = Data::String("I'm a string".into());
/// let number_data = Data::Number(1.0);
/// let boolean_data = Data::Boolean(false);
/// let object_data = Data::Object(HashMap::new());
/// ```
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

/// Struct abstraction of the narrative context.
///
/// The crate will use this module to interpolate strings based on those variables for
/// manuscript building.
/// 
/// # Examples
/// ```
/// use makinilya_core::context::{Data, Context};
/// use std::collections::HashMap;
/// 
/// let mut variables: HashMap<String, Data> = HashMap::new();
/// variables.insert("main_character_name".into(), Data::String("Alyssa".into()));
/// 
/// let context = Context::from(variables);
/// ```
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

impl From<HashMap<String, Data>> for Context {
    fn from(variables: HashMap<String, Data>) -> Self {
        Self { variables }
    }
}
