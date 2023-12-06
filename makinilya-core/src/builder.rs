use docx_rs::{
    AlignmentType, Docx, LineSpacing, LineSpacingType, PageMargin, Paragraph, Run, RunFonts, Table,
    TableCell, TableRow, VAlignType, WidthType,
};
use thiserror::Error;

use crate::story::Story;

#[derive(Error, Debug)]
pub enum Error {}

pub struct ContactInformation {
    pub name: String,
    pub address_1: String,
    pub address_2: String,
    pub mobile_number: String,
    pub email_address: String,
}

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
                address_2: "Address 2".into(),
                mobile_number: "01234567890".into(),
                email_address: "unknown@mail.com".into(),
            },
            agent_information: None,
        }
    }
}

pub struct ManuscriptBuilder {
    pub word_count: u32,
    pub layout: ManuscriptBuilderLayout,
}

impl ManuscriptBuilder {
    pub fn new(layout: ManuscriptBuilderLayout) -> Self {
        Self {
            layout,
            word_count: 0,
        }
    }

    fn build_document(&self) -> Docx {
        Docx::new().page_size(12240, 15840).page_margin(
            PageMargin::new()
                .top(1440)
                .bottom(1440)
                .left(1440)
                .right(1440),
        )
    }

    fn separate_with_commas(num: u32) -> String {
        num.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }

    fn paragraph(text: &str) -> Paragraph {
        Paragraph::new()
            .size(24)
            .add_run(Run::new().add_text(text).size(24))
            .fonts(RunFonts::new().ascii("Times New Roman"))
    }

    fn build_title_page(&self, doc: Docx) -> Docx {
        let top_section_paragraph = |text: &str| {
            Self::paragraph(text)
                .line_spacing(
                    LineSpacing::new()
                        .line_rule(LineSpacingType::Auto)
                        .line(240)
                        .after(0),
                )
                .align(AlignmentType::Left)
        };
        let middle_section_paragraph = |text: &str| {
            Self::paragraph(text)
                .line_spacing(
                    LineSpacing::new()
                        .line_rule(LineSpacingType::Auto)
                        .line(480)
                        .after(0),
                )
                .align(AlignmentType::Center)
        };
        let bottom_section_paragraph = |text: &str| {
            Self::paragraph(text)
                .line_spacing(
                    LineSpacing::new()
                        .line_rule(LineSpacingType::Auto)
                        .line(240)
                        .after(0),
                )
                .align(AlignmentType::Right)
        };

        let mut table_rows = vec![
            TableRow::new(vec![TableCell::new()
                .clear_all_border()
                .vertical_align(VAlignType::Top)
                .add_paragraph(top_section_paragraph(&self.layout.contact_information.name))
                .add_paragraph(top_section_paragraph(
                    &self.layout.contact_information.address_1,
                ))
                .add_paragraph(top_section_paragraph(
                    &self.layout.contact_information.address_2,
                ))
                .add_paragraph(top_section_paragraph(
                    &self.layout.contact_information.mobile_number,
                ))
                .add_paragraph(top_section_paragraph(
                    &self.layout.contact_information.email_address,
                ))])
            .row_height(4320.0),
            TableRow::new(vec![TableCell::new()
                .clear_all_border()
                .vertical_align(VAlignType::Center)
                .add_paragraph(middle_section_paragraph(&self.layout.title))
                .add_paragraph(middle_section_paragraph(
                    &self.layout.contact_information.name,
                ))
                .add_paragraph(middle_section_paragraph(&format!(
                    "Approx. {} words",
                    Self::separate_with_commas(self.word_count)
                )))])
            .row_height(4320.0),
        ];

        if let Some(agent_information) = &self.layout.agent_information {
            table_rows.push(
                TableRow::new(vec![TableCell::new()
                    .clear_all_border()
                    .vertical_align(VAlignType::Bottom)
                    .add_paragraph(bottom_section_paragraph(&agent_information.name))
                    .add_paragraph(bottom_section_paragraph(&agent_information.address_1))
                    .add_paragraph(bottom_section_paragraph(&agent_information.address_2))
                    .add_paragraph(bottom_section_paragraph(&agent_information.mobile_number))
                    .add_paragraph(bottom_section_paragraph(
                        &agent_information.email_address,
                    ))])
                .row_height(4320.0),
            )
        }

        doc.add_table(Table::new(table_rows).width(9360, WidthType::Auto))
    }

    pub fn build(&self, _story: &Story) -> Result<(), Error> {
        let path = std::path::Path::new("./hello.docx");
        let file = std::fs::File::create(&path).unwrap();

        let mut doc = self.build_document();
        doc = self.build_title_page(doc);

        doc.build().pack(file).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn builds_pdf() {
        let builder = ManuscriptBuilder::new(ManuscriptBuilderLayout::default());
        let result = builder.build(&Story::new_part("Root"));
        assert!(result.is_ok())
    }
}
