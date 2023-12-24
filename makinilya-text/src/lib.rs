use pest::{error::LineColLocation, iterators::Pairs, Parser, RuleType};
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "./grammar/makinilya.pest"]
struct GrammarParser;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[line {0}:{1}] {2}")]
    ParsingError(usize, usize, String),
}

pub struct MakinilyaText;

impl MakinilyaText {
    pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, Error> {
        GrammarParser::parse(Rule::makinilya, source).map_err(|error| Self::map_parser_error(error))
    }

    fn map_parser_error<R>(error: pest::error::Error<R>) -> Error
    where
        R: RuleType,
    {
        let message = error.variant.message();
        let (line, col) = match error.line_col {
            LineColLocation::Pos(line_col) => line_col,
            _ => (0, 0),
        };
        Error::ParsingError(line, col, message.into())
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn parses_string_interpolation() {
        let file = MakinilyaText::parse("{{ name }}");
        assert!(file.is_ok());
        let file = MakinilyaText::parse("{{ }}");
        assert!(file.is_err());
        let file = MakinilyaText::parse("{{ 32 }}");
        assert!(file.is_err());
        let file = MakinilyaText::parse("{{ name32 }}");
        assert!(file.is_ok());
        let file = MakinilyaText::parse("{{ name_32 }}");
        assert!(file.is_ok());
        let file = MakinilyaText::parse("{{ name_32.long }}");
        assert!(file.is_ok());
        let file = MakinilyaText::parse("{{ name_32..long }}");
        assert!(file.is_err());
    }

    #[test]
    fn parses_content() {
        let file = GrammarParser::parse(Rule::makinilya, "Hello. My name is {{ name }}.");
        assert!(file.is_ok());
        let file = GrammarParser::parse(Rule::makinilya, "Hello. My name is {{ name.long }}.");
        assert!(file.is_ok());
    }
}
