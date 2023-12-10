//! Structs for units used in the Office Open XML (OOXML)
//! specification.
//!
//! The conversion values were taken from the [Open XML SDK]
//! from Microsoft.
//! 
//! [Open XML SDK]: https://learn.microsoft.com/en-us/office/open-xml/open-xml-sdk

/// A unit that equates to one-twentieth of an imperial point,
/// hence "twip". It is 1/1440 of an inch.
///
/// # Examples
/// ```
/// use makinilya_core::units::Twip;
///
/// let twip_8_inch = Twip::from_inch(8.0);
/// let twip_12_point = Twip::from_point(12.0);
///
/// let float_twip: f32 = twip_8_inch.into();
/// ```
pub struct Twip(f32);

impl Twip {
    const POINT_TO_TWIP: f32 = 20.0;
    const INCH_TO_TWIP: f32 = 1440.0;

    pub fn from_inch(value: f32) -> Self {
        Self(value * Self::INCH_TO_TWIP)
    }

    pub fn from_point(value: f32) -> Self {
        Self(value * Self::POINT_TO_TWIP)
    }
}

impl Into<usize> for Twip {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Into<u32> for Twip {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

impl Into<i32> for Twip {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<f32> for Twip {
    fn into(self) -> f32 {
        self.0 as f32
    }
}

/// As the name suggests. This is half of a point. It is 1/144
/// of an inch.
///
/// # Examples
/// ```
/// use makinilya_core::units::HalfPoint;
///
/// let half_point_12_point = HalfPoint::from_point(12.0);
///
/// let usize_half_point: usize = half_point_12_point.into();
/// ```
pub struct HalfPoint(f32);

impl HalfPoint {
    pub fn from_point(value: f32) -> Self {
        Self(value * 2.0)
    }
}

impl Into<usize> for HalfPoint {
    fn into(self) -> usize {
        self.0 as usize
    }
}
