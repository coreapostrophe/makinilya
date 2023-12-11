//! Structs and implementations for building the manuscript

use docx_rs::{
    AlignmentType, Docx, LineSpacing, LineSpacingType, PageMargin, Paragraph, Run, RunFonts,
    SpecialIndentType, Table, TableCell, TableRow, VAlignType, WidthType,
};
use thiserror::Error;

use crate::{story::Story, units::{HalfPoint, Twip}, extensions::{OptionalParagraph, WithThousandsSeparator}};

#[derive(Error, Debug)]
pub enum Error {}

#[derive(Debug)]
pub struct ContactInformation {
    pub name: String,
    pub address_1: String,
    pub address_2: Option<String>,
    pub mobile_number: Option<String>,
    pub email_address: String,
}

#[derive(Debug)]
pub struct ParagraphLayout {
    pub font_size_point: f32,
    pub line_spacing_point: f32,
    pub after_line_spacing_point: f32,
    pub first_line_indention_inch: f32,
    pub alignment: AlignmentType,
}

impl Default for ParagraphLayout {
    fn default() -> Self {
        Self {
            font_size_point: 12.0,
            line_spacing_point: 24.0,
            after_line_spacing_point: 0.0,
            first_line_indention_inch: 0.0,
            alignment: AlignmentType::Left,
        }
    }
}

#[derive(Debug)]
pub struct ManuscriptBuilderLayout {
    pub title: String,
    pub pen_name: String,
    pub contact_information: ContactInformation,
    pub agent_information: Option<ContactInformation>,
}

impl Default for ManuscriptBuilderLayout {
    fn default() -> Self {
        Self {
            title: "Untitled".into(),
            pen_name: "Unknown Author".into(),
            contact_information: ContactInformation {
                name: "Unknown Author".into(),
                address_1: "Address 1".into(),
                address_2: None,
                mobile_number: None,
                email_address: "unknown@mail.com".into(),
            },
            agent_information: None,
        }
    }
}

/// Builds the manuscript.
/// 
/// Stores a `layout` field that contains all of the title page information,
/// and builds the a `manuscript.docx` file from a provided `Story` struct.
/// 
/// # Example
/// ```
/// use makinilya_core::{builder::ManuscriptBuilder, files::FileHandler};
/// 
/// let builder = ManuscriptBuilder::new(Default::default());
/// let story = FileHandler::build_story("./mock").unwrap();
/// let result = builder.build(&story);
/// 
/// assert!(result.is_ok());
/// ```
#[derive(Debug)]
pub struct ManuscriptBuilder {
    pub layout: ManuscriptBuilderLayout,
}

impl ManuscriptBuilder {
    pub fn new(layout: ManuscriptBuilderLayout) -> Self {
        Self { layout }
    }

    fn paragraph(text: &str, layout: ParagraphLayout) -> Paragraph {
        Paragraph::new()
            .align(layout.alignment)
            .fonts(RunFonts::new().ascii("Times New Roman"))
            .size(HalfPoint::from_point(layout.font_size_point).into())
            .add_run(
                Run::new()
                    .add_text(text)
                    .size(HalfPoint::from_point(layout.font_size_point).into()),
            )
            .line_spacing(
                LineSpacing::new()
                    .line_rule(LineSpacingType::Auto)
                    .line(Twip::from_point(layout.line_spacing_point).into())
                    .after(Twip::from_point(layout.after_line_spacing_point).into()),
            )
            .indent(
                None,
                Some(SpecialIndentType::FirstLine(
                    Twip::from_inch(layout.first_line_indention_inch).into(),
                )),
                None,
                None,
            )
    }

    fn word_count(story: &Story) -> u32 {
        let mut count = 0;

        for content in story.contents() {
            let words: Vec<&str> = content
                .split(|c: char| c.is_whitespace())
                .filter(|item| !item.is_empty())
                .collect();
            count += words.len() as u32;
        }

        for part in story.parts() {
            count += Self::word_count(part);
        }

        count
    }

    fn build_document(&self) -> Docx {
        Docx::new()
            .page_size(Twip::from_inch(8.5).into(), Twip::from_inch(11.0).into())
            .page_margin(
                PageMargin::new()
                    .top(Twip::from_inch(1.0).into())
                    .bottom(Twip::from_inch(1.0).into())
                    .left(Twip::from_inch(1.0).into())
                    .right(Twip::from_inch(1.0).into()),
            )
    }

    fn build_title_page(&self, doc: Docx, word_count: u32) -> Docx {
        let top_paragraph = |text: Option<&String>| {
            text.map(|text| {
                Self::paragraph(
                    text,
                    ParagraphLayout {
                        line_spacing_point: 12.0,
                        ..Default::default()
                    },
                )
            })
        };
        let middle_paragraph = |text: Option<&String>| {
            text.map(|text| {
                Self::paragraph(
                    text,
                    ParagraphLayout {
                        alignment: AlignmentType::Center,
                        ..Default::default()
                    },
                )
            })
        };
        let bottom_paragraph = |text: Option<&String>| {
            text.map(|text| {
                Self::paragraph(
                    text,
                    ParagraphLayout {
                        line_spacing_point: 12.0,
                        alignment: AlignmentType::Right,
                        ..Default::default()
                    },
                )
            })
        };

        let title = &self.layout.title;
        let contact_information = &self.layout.contact_information;
        let word_count = format!(
            "{} words",
            word_count.to_string().with_thousands_separator()
        );

        let mut table_rows = vec![
            TableRow::new(vec![TableCell::new()
                .clear_all_border()
                .vertical_align(VAlignType::Top)
                .add_opt_paragraph(top_paragraph(Some(&contact_information.name)))
                .add_opt_paragraph(top_paragraph(Some(&contact_information.address_1)))
                .add_opt_paragraph(top_paragraph(contact_information.address_2.as_ref()))
                .add_opt_paragraph(top_paragraph(contact_information.mobile_number.as_ref()))
                .add_opt_paragraph(top_paragraph(Some(
                    &contact_information.email_address,
                )))])
            .row_height(Twip::from_inch(9.0 / 3.0).into()),
            TableRow::new(vec![TableCell::new()
                .clear_all_border()
                .vertical_align(VAlignType::Center)
                .add_opt_paragraph(middle_paragraph(Some(&title)))
                .add_opt_paragraph(middle_paragraph(Some(&contact_information.name)))
                .add_opt_paragraph(middle_paragraph(Some(&word_count)))])
            .row_height(Twip::from_inch(9.0 / 3.0).into()),
        ];

        if let Some(agent_information) = &self.layout.agent_information {
            table_rows.push(
                TableRow::new(vec![TableCell::new()
                    .clear_all_border()
                    .vertical_align(VAlignType::Bottom)
                    .add_opt_paragraph(bottom_paragraph(Some(&agent_information.name)))
                    .add_opt_paragraph(bottom_paragraph(Some(&agent_information.address_1)))
                    .add_opt_paragraph(bottom_paragraph(agent_information.address_2.as_ref()))
                    .add_opt_paragraph(bottom_paragraph(agent_information.mobile_number.as_ref()))
                    .add_opt_paragraph(bottom_paragraph(Some(
                        &agent_information.email_address,
                    )))])
                .row_height(Twip::from_inch(9.0 / 3.0).into()),
            )
        }

        doc.add_table(Table::new(table_rows).width(Twip::from_inch(6.5).into(), WidthType::Auto))
    }

    fn build_chapter(&self, mut doc: Docx, story: &Story) -> Docx {
        if !story.contents().is_empty() {
            doc = doc
                .add_paragraph(
                    Paragraph::new().add_run(Run::new().add_break(docx_rs::BreakType::Page)),
                )
                .add_table(
                    Table::new(vec![TableRow::new(vec![TableCell::new()])
                        .row_height(Twip::from_inch(9.0 / 3.0).into())])
                    .clear_all_border(),
                )
                .add_paragraph(Self::paragraph(
                    &story.title(),
                    ParagraphLayout {
                        line_spacing_point: 24.0,
                        after_line_spacing_point: 24.0,
                        alignment: AlignmentType::Center,
                        ..Default::default()
                    },
                ));

            let mut peekable_contents = story.contents().iter().peekable();

            while let Some(content) = peekable_contents.next() {
                let splitted_source = content.split("\n");

                for paragraph in splitted_source {
                    doc = doc.add_paragraph(Self::paragraph(
                        paragraph,
                        ParagraphLayout {
                            first_line_indention_inch: 0.5,
                            ..Default::default()
                        },
                    ));
                }

                if peekable_contents.peek().is_some() {
                    doc = doc.add_paragraph(Self::paragraph(
                        "#",
                        ParagraphLayout {
                            alignment: AlignmentType::Center,
                            ..Default::default()
                        },
                    ));
                }
            }
        }

        for part in story.parts() {
            doc = self.build_chapter(doc, part);
        }

        doc
    }

    pub fn build(&self, story: &Story) -> Result<Docx, Error> {
        let word_count = Self::word_count(story);

        let mut doc = self.build_document();
        doc = self.build_title_page(doc, word_count);
        doc = self.build_chapter(doc, story);

        Ok(doc)
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn builds_pdf() {
        let mock_story = {
            let mut story = Story::new("Root");

            let mut chapter_1 = Story::new("Chapter 1");
            chapter_1.push_content("I am Scene #1.");
            chapter_1.push_content("I am Scene #2.");

            story.push_part(chapter_1);

            story
        };

        let builder = ManuscriptBuilder::new(ManuscriptBuilderLayout::default());
        let result = builder.build(&mock_story);
        assert!(result.is_ok());
    }
}
