//! Handles the context of the narrative.
//!
//! The variables that the context stores are extracted from the project's `Context.toml`.
//! They are free-form and do not require a specific structure. The context's main function
//! is to serve these variables to the project's scenes, allowing for some dynamic writing
//! using features like string interpolation.
//!
//! # Examples
//! ## Adding variables to context
//! This is an example of content within the `Context.toml` of the project.
//! ```toml
//! [names]
//! author = { first = "Mark", last = "Lopez", full = "Mark Lopez" }
//! ```
//!
//! ## Using the variables from context
//! This is an example scene `Scene.mt` that would be located within the project's draft directory.
//! (The draft is usually inside `/draft`)
//! ```plaintext
//! Hello, my name is {{ names.author.full }}, short for {{ names.author.short }}.
//! ```

use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;
use toml::{Table, Value};

#[doc(hidden)]
#[derive(Error, Debug)]
pub enum ContextError {
    #[error(transparent)]
    Parsing(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("`DateTime` and `Array` are not supported context values.")]
    UnsupportedValue,
}

/// Enum of all valid values that the [`Context`] could store.
///
/// They are a subset of the native types supported in the [`TOML`] language spec. More complex
/// types such as `Arrays` and `DateTimes` are not supported as there's currently no apparent
/// use-case for them. Though, they might be supported in the future.
///
/// [`TOML`]: https://toml.io/en/v1.0.0
#[allow(missing_docs)]
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

/// Stores all context values for project use.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct Context {
    pub variables: HashMap<String, Data>,
}

#[doc(hidden)]
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

    pub fn read(path: impl Into<PathBuf>) -> Result<Context, ContextError> {
        let file_string = std::fs::read_to_string(path.into().as_path())?;
        let context = Self::parse(&file_string)?;
        Ok(context)
    }
}

#[doc(hidden)]
impl From<HashMap<String, Data>> for Context {
    fn from(variables: HashMap<String, Data>) -> Self {
        Self { variables }
    }
}
