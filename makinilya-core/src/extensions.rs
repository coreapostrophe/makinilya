//! Traits for extending implementations from external crates, making
//! them more attuned to the crate's use cases.

use docx_rs::{Paragraph, TableCell};

/// Optionally renders a paragraph to a document structure.
///
/// Consumes an `Option<Paragraph>` argument that determines whether
/// or not a paragraph is rendered or not. If the value is `Some`,
/// the paragraph will be added to the structure, otherwise, the
/// operation will be ignored.
pub trait OptionalParagraph {
    fn add_opt_paragraph(self, p: Option<Paragraph>) -> Self;
}

impl OptionalParagraph for TableCell {
    fn add_opt_paragraph(self, p: Option<Paragraph>) -> Self {
        match p {
            Some(p) => self.add_paragraph(p),
            None => self,
        }
    }
}

/// Separarates a string with commas (,) at every 3rd character.
///
/// Mainly a utility for numeric strings, this allows the separation
/// of each digit on a relative thousand place for added readability.
pub trait WithThousandsSeparator {
    fn with_thousands_separator(self) -> Self;
}

impl WithThousandsSeparator for String {
    fn with_thousands_separator(self) -> Self {
        self.as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }
}

/// Clones content of `Option` if it exists, otherwise it returns
/// a default value.
pub trait CloneOnSome<T: Clone> {
    fn clone_on_some(&self, default: T) -> T;
}

impl<T: Clone> CloneOnSome<T> for Option<&T> {
    fn clone_on_some(&self, default: T) -> T {
        self.map_or(default, |some| some.clone())
    }
}
