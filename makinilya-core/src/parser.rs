use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar/markdown.pest"]
pub struct MarkdownParser;
