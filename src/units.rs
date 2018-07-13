//! Types for units in PDF files in points, millimeters and pixels (wrappers 
//! around floating points).
//! By default points are used but also offers the possibilty 
//! to use other metrics.

use std::fmt;

const MM_TO_PT: f32 = 2.834_646;
const PT_TO_MM: f32 = 0.352_778;
const PX_TO_PT: f32 = 72.0;

/// Represents points in a Pdf. Points can be converted from
/// and into millimeters and to pixels.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pt(pub f32);

impl Pt {
    /// Returns the value in pixels with given dpi.
    pub fn to_px(self, dpi: f32) -> Px {
        Px(f32::round(self.0 / PX_TO_PT * dpi) as usize)
    }
}

impl From<Mm> for Pt {
    fn from(value: Mm) -> Pt {
        Pt(value.0 * MM_TO_PT)
    }
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents millimeters in a Pdf. Millimeters can be converted from
/// and into points and to pixels.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mm(pub f32);

impl From<Pt> for Mm {
    fn from(value: Pt) -> Mm {
        Mm(value.0 * PT_TO_MM)
    }
}

impl Mm {
    /// Returns the value in pixels with given dpi.
    pub fn to_px(self, dpi: f32) -> Px {
        Px(f32::round(Pt::from(self).0 / PX_TO_PT * dpi) as usize)
    }
}

/// Pixels are measured in a different way. For pixels dpi 
/// (dots per inches) have to be defined.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Px(pub usize);

impl Px {
    /// Returns the value in points with given dpi.
    pub fn to_pt(self, dpi: f32) -> Pt {
        Pt(self.0 as f32 / dpi * PX_TO_PT)
    }
    /// Returns the value in millimeters with given dpi.
    pub fn to_mm(self, dpi: f32) -> Mm {
        Mm::from(self.to_pt(dpi))
    }
}
