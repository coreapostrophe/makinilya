use makinilya_core::parser::{MarkdownParser, Rule};
use pest::Parser;

#[test]
fn parses_string_interpolation() {
    let file = MarkdownParser::parse(Rule::markdown, "{{ name }}");
    assert!(file.is_ok());
    let file = MarkdownParser::parse(Rule::markdown, "{{ }}");
    assert!(file.is_err());
    let file = MarkdownParser::parse(Rule::markdown, "{{ ?#@# }}");
    assert!(file.is_err());
}
