//! Structs for units used in the Office Open XML (OOXML) specification.
//!
//! The conversion values were taken from the [Open XML SDK] from Microsoft.
//!
//! [Open XML SDK]: https://learn.microsoft.com/en-us/office/open-xml/open-xml-sdk

use std::fmt::Debug;

/// A unit that equates to one-twentieth of an imperial point, hence "twip". It is 1/1440
/// of an inch.
///
/// # Examples
/// ```
/// use makinilya_core::units::Twip;
///
/// let twip_8_inch = Twip::from_inch(8.0);
/// let twip_12_point = Twip::from_point(12.0);
///
/// let float_twip: f32 = twip_8_inch.into();
///
/// assert_eq!(Twip::from_inch(8.0), 8.0 * 1440.0);
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

    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

impl Debug for Twip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Twip> for usize {
    fn from(value: Twip) -> Self {
        value.0 as usize
    }
}

impl From<Twip> for u32 {
    fn from(value: Twip) -> Self {
        value.0 as u32
    }
}

impl From<Twip> for i32 {
    fn from(value: Twip) -> Self {
        value.0 as i32
    }
}

impl From<Twip> for f32 {
    fn from(value: Twip) -> Self {
        value.0 as f32
    }
}

impl PartialEq<f32> for Twip {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
    fn ne(&self, other: &f32) -> bool {
        self.0 != *other
    }
}

impl PartialEq for Twip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl PartialEq<Twip> for f32 {
    fn eq(&self, other: &Twip) -> bool {
        *self == other.0
    }
    fn ne(&self, other: &Twip) -> bool {
        *self != other.0
    }
}

/// As the name suggests, this is half of a point. It is 1/144 of an inch.
///
/// # Examples
/// ```
/// use makinilya_core::units::HalfPoint;
///
/// let half_point_12_point = HalfPoint::from_point(12.0);
/// let usize_half_point: usize = half_point_12_point.into();
///
/// assert_eq!(HalfPoint::from_point(12.0), 12.0 * 2.0);
/// ```
pub struct HalfPoint(f32);

impl HalfPoint {
    pub fn from_point(value: f32) -> Self {
        Self(value * 2.0)
    }
}

impl From<HalfPoint> for usize {
    fn from(value: HalfPoint) -> Self {
        value.0 as usize
    }
}

impl Debug for HalfPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for HalfPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl PartialEq<f32> for HalfPoint {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
    fn ne(&self, other: &f32) -> bool {
        self.0 != *other
    }
}

impl PartialEq<HalfPoint> for f32 {
    fn eq(&self, other: &HalfPoint) -> bool {
        *self == other.0
    }
    fn ne(&self, other: &HalfPoint) -> bool {
        *self != other.0
    }
}
