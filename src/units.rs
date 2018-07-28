//! Types for units in PDF files in points, millimeters and pixels (wrappers
//! around floating points).
//! By default points are used but also offers the possibilty
//! to use other metrics.

use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

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

impl From<f32> for Pt {
    fn from(value: f32) -> Pt {
        Pt(value)
    }
}

impl From<Pt> for f32 {
    fn from(value: Pt) -> f32 {
        value.0
    }
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Pt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pt(self.0 + other.0)
    }
}

impl Sub for Pt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Pt(self.0 - other.0)
    }
}

impl Mul for Pt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Pt(self.0 * other.0)
    }
}

impl Div for Pt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Pt(self.0 / other.0)
    }
}

/// Represents millimeters in a Pdf. Millimeters can be converted from
/// and into points and to pixels.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mm(pub f32);

impl Mm {
    /// Returns the value in pixels with given dpi.
    pub fn to_px(self, dpi: f32) -> Px {
        Px(f32::round(Pt::from(self).0 / PX_TO_PT * dpi) as usize)
    }
}

impl From<Pt> for Mm {
    fn from(value: Pt) -> Mm {
        Mm(value.0 * PT_TO_MM)
    }
}

impl fmt::Display for Mm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Pt::from(*self))
    }
}

impl Add for Mm {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Mm(self.0 + other.0)
    }
}

impl Sub for Mm {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Mm(self.0 - other.0)
    }
}

impl Mul for Mm {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Mm(self.0 * other.0)
    }
}

impl Div for Mm {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Mm(self.0 / other.0)
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
    /// Returns the value as floating point with given dpi.
    pub fn to_fp(self, dpi: f32) -> f32 {
        self.to_pt(dpi).0
    }
}
