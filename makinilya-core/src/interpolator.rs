use makinilya_text::{Error, MakinilyaText, Rule};
use pest::iterators::Pair;

use crate::{
    context::{Context, Data},
    story::Story,
};

pub struct StoryInterpolator;

impl StoryInterpolator {
    pub fn interpolate(story: &Story, context: &Context) -> Result<Story, Error> {
        let mut interpolated_story = Story::new(story.title());

        for content in story.contents() {
            let parsed_source = MakinilyaText::parse(&content)?.next().unwrap();
            let expressions = parsed_source.into_inner();

            let interpolated_expressions: Vec<String> = expressions
                .map(|expression| Self::interpolate_expression(expression, context))
                .collect();

            interpolated_story.push_content(interpolated_expressions.join(""));
        }

        for part in story.parts() {
            let interpolated_part = Self::interpolate(part, context)?;
            interpolated_story.push_part(interpolated_part);
        }

        Ok(interpolated_story)
    }

    fn interpolate_expression(expression: Pair<'_, Rule>, context: &Context) -> String {
        let mut result = String::new();

        if let Some(expression_value) = expression.into_inner().next() {
            match expression_value.as_rule() {
                Rule::string_interpolation => {
                    let mut identifier_array = expression_value
                        .into_inner()
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|pair| pair.as_str());

                    let first_identifier = identifier_array.next().unwrap();
                    let mut data = context.variables().get(first_identifier);

                    while let Some(identifier) = identifier_array.next() {
                        if let Some(unwrapped_data) = data {
                            match unwrapped_data {
                                Data::Object(object_value) => {
                                    data = object_value.get(identifier);
                                }
                                _ => (),
                            }
                        }
                    }

                    if let Some(unwrapped_data) = data {
                        result.push_str(&unwrapped_data.to_string());
                    }
                }
                Rule::text_content => {
                    result.push_str(expression_value.as_str());
                }
                _ => (),
            }
        }

        result
    }
}
