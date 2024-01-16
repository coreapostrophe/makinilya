//! Structs for creating the context of the narrative.

use std::collections::HashMap;

use thiserror::Error;
use toml::{Table, Value};

#[derive(Error, Debug)]
pub enum ContextError {
    #[error(transparent)]
    ParsingError(#[from] toml::de::Error),

    #[error("`DateTime` and `Array` are not supported context values.")]
    UnsupportedValue,
}

/// Enum representation of all valid values that the `Context` could store.
///
/// They are a subset of the native types supported in the [`TOML`] language spec. More complex
/// types such as `Arrays` and `DateTimes` are not included as there's currently no apparent
/// use-case for them. Though, it's possible that they'll be included in the future.
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
///
/// [`TOML`]: https://toml.io/en/v1.0.0
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
/// This struct stores indexable values that is used for interpolation in the final manuscript.
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

    fn parse_variables(table: Table) -> Result<HashMap<String, Data>, ContextError> {
        let mut variables = HashMap::new();

        for (key, value) in table.iter() {
            match value {
                Value::String(string_value) => {
                    variables.insert(key.to_owned(), Data::String(string_value.to_owned()));
                }
                Value::Integer(integer_value) => {
                    variables.insert(key.to_owned(), Data::Number(*integer_value as f64));
                }
                Value::Float(float_value) => {
                    variables.insert(key.to_owned(), Data::Number(*float_value));
                }
                Value::Boolean(boolean_value) => {
                    variables.insert(key.to_owned(), Data::Boolean(*boolean_value));
                }
                Value::Table(table_value) => {
                    let object_value = Self::parse_variables(table_value.to_owned())?;
                    variables.insert(key.to_owned(), Data::Object(object_value));
                }
                _ => return Err(ContextError::UnsupportedValue),
            }
        }

        Ok(variables)
    }

    pub fn parse(source: &str) -> Result<Self, ContextError> {
        let table = source.parse::<Table>()?;

        let variables = Self::parse_variables(table)?;
        let context = Self::from(variables);

        Ok(context)
    }
}

impl From<HashMap<String, Data>> for Context {
    fn from(variables: HashMap<String, Data>) -> Self {
        Self { variables }
    }
}
