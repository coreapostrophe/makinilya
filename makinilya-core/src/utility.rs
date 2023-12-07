use docx_rs::{Paragraph, TableCell};

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

pub struct Twip;

impl Twip {
    const POINT_TO_TWIP: u32 = 20;
    const INCH_TO_TWIP: u32 = 1440;

    pub fn from_inch(value: f32) -> f32 {
        value * Self::INCH_TO_TWIP as f32
    }

    pub fn from_point(value: f32) -> f32 {
        value * Self::POINT_TO_TWIP as f32
    }
}

pub struct HalfPoint;

impl HalfPoint {
    pub fn from_point(value: f32) -> f32 {
        value * 2.0
    }
}
