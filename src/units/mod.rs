//  https://github.com/code-ape/rust_length_arithmetic_example
//  Changes to file include renaming types, methods and macros.
//    Copyright 2018 Ferris Ellis

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

//! Types for units.
#[macro_use]
mod macros;

use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Length unit inside the UserSpace used for PDF.
///
/// There different kinds of spaces (e. g. TextSpace) which define how images
/// and text are rendered on different devices with different resolutions. The
/// UserSpace is independent, meaning that on all devices the size is always
/// the same.
/// The base unit is point which equals roughly 1/72 inches.
///
/// Basic mathematical operations like addition, subtraction and division can
/// be executed with units but not multiplication. However, units can be
/// multiplied or divided by numbers.
#[derive(Debug, Clone, Copy)]
pub struct UserSpace<T: LengthUnit> {
    pub(crate) pt: f32,
    unit: PhantomData<T>,
}

impl<T: LengthUnit> UserSpace<T> {
    /// Returns absoulute value as floating point.
    pub fn abs(self) -> f32 {
        self.pt.abs()
    }
    /// Returns cosinus of the value as floating point.
    pub fn cos(self) -> f32 {
        self.pt.cos()
    }
    /// Returns sinus of the value as floating point.
    pub fn sin(self) -> f32 {
        self.pt.sin()
    }
    /// Returns tangent of the value as floating point.
    pub fn tan(self) -> f32 {
        self.pt.tan()
    }
}

/// Trait for implementing units.
pub trait LengthUnit: Copy {
    /// The conversion number from one unit to points. For example, 1mm equals
    /// circa 2.8 points.
    const PT_IN_UNIT: f32;
}

impl<T> fmt::Display for UserSpace<T>
where
    T: LengthUnit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pt)
    }
}

/// The unit can only be used with the mm! macro.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Millimeters;
/// The unit can only be used with the pt! macro. The unit points is widely
/// used in printing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Points;

/// Using this macro requires importing the Millimeters struct.
#[macro_export]
macro_rules! mm {
    ($num:expr) => {
        UserSpace::<Millimeters>::from(&$num)
    };
}

/// Using this macro requires importing the Points struct.
#[macro_export]
macro_rules! pt {
    ($num:expr) => {
        UserSpace::<Points>::from(&$num)
    };
}

impl<'a, T1, T2> From<&'a UserSpace<T1>> for UserSpace<T2>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    fn from(l: &'a UserSpace<T1>) -> Self {
        UserSpace {
            pt: l.pt,
            unit: PhantomData,
        }
    }
}

// Allow lengths to be added
impl<T1, T2> Add<UserSpace<T2>> for UserSpace<T1>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    type Output = UserSpace<T1>;

    fn add(self, other: UserSpace<T2>) -> Self::Output {
        UserSpace {
            pt: self.pt + other.pt,
            unit: PhantomData,
        }
    }
}

// Allow lengths to be subtracted
impl<T1, T2> Sub<UserSpace<T2>> for UserSpace<T1>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    type Output = UserSpace<T1>;

    fn sub(self, other: UserSpace<T2>) -> Self::Output {
        UserSpace {
            pt: self.pt - other.pt,
            unit: PhantomData,
        }
    }
}

// Allow lengths to be divided
// this yields a number as a UserSpace divided by a UserSpace is just a number
impl<T1, T2> Div<UserSpace<T2>> for UserSpace<T1>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    type Output = f32;

    fn div(self, other: UserSpace<T2>) -> Self::Output {
        self.pt / other.pt
    }
}

impl<T: LengthUnit> Neg for UserSpace<T> {
    type Output = UserSpace<T>;

    fn neg(self) -> Self::Output {
        UserSpace {
            pt: -self.pt,
            unit: PhantomData,
        }
    }
}

impl<T1, T2> PartialEq<UserSpace<T2>> for UserSpace<T1>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    fn eq(&self, other: &UserSpace<T2>) -> bool {
        self.pt == other.pt
    }
}

impl<T1, T2> PartialOrd<UserSpace<T2>> for UserSpace<T1>
where
    T1: LengthUnit,
    T2: LengthUnit,
{
    fn partial_cmp(&self, other: &UserSpace<T2>) -> Option<Ordering> {
        self.pt.partial_cmp(&other.pt)
    }
}

newUnit!(Millimeters, 2.834_646);
newUnit!(Points, 1.0);

implFromUserSpace!(f64);
implFromUserSpace!(i64);
implFromUserSpace!(f32);
implFromUserSpace!(i32);
implFromUserSpace!(isize);

implMulAndDiv!(i64);
implMulAndDiv!(f64);
implMulAndDiv!(i32);
implMulAndDiv!(f32);
implMulAndDiv!(isize);
