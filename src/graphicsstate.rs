//! Types for representing details in the graphics state.
use std::f32::consts::PI;
use std::fmt;
use std::ops::Mul;
use units::{LengthUnit, UserSpace};

/// Line join styles, as described in section 8.4.3.4 of the PDF specification.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum JoinStyle {
    /// The outer edges continues until they meet.
    Miter,
    /// The lines are joined by a circle of line-width diameter.
    Round,
    /// End the lines as with `CapStyle::Butt` and fill the resulting gap with
    /// a triangle.
    Bevel,
}

impl fmt::Display for JoinStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                JoinStyle::Miter => 0,
                JoinStyle::Round => 1,
                JoinStyle::Bevel => 2,
            }
        )
    }
}

/// Line cap styles, as described in section 8.4.3.4 of the PDF specification.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum CapStyle {
    /// Truncate the line squarely through the endpoint.
    Butt,
    /// Include a circle of line-width diameter around the endpoint.
    Round,
    /// Include a square around the endpoint, so the line continues for half a
    /// line-width through the endpoint.
    ProjectingSquare,
}

impl fmt::Display for CapStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                CapStyle::Butt => 0,
                CapStyle::Round => 1,
                CapStyle::ProjectingSquare => 2,
            }
        )
    }
}

/// Any color (or grayscale) value that this library can make PDF represent.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    #[doc(hidden)]
    RGB { red: u8, green: u8, blue: u8 },
    #[doc(hidden)]
    Gray { gray: u8 },
}

impl Color {
    /// Return a color from a RGB colorspace.

    /// # Example
    /// ````
    /// # use simple_pdf::graphicsstate::Color;
    /// let white = Color::rgb(255, 255, 255);
    /// let black = Color::rgb(0, 0, 0);
    /// let red = Color::rgb(255, 0, 0);
    /// let yellow = Color::rgb(255, 255, 0);
    /// ````
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color::RGB { red, green, blue }
    }

    /// Return a grayscale color value.

    /// # Example
    /// ````
    /// # use simple_pdf::graphicsstate::Color;
    /// let white = Color::gray(255);
    /// let gray = Color::gray(128);
    /// ````
    pub fn gray(gray: u8) -> Self {
        Color::Gray { gray }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let norm = |c: u8| f32::from(c) / 255.0;
        match *self {
            Color::RGB { red, green, blue } => {
                write!(f, "{} {} {}", norm(red), norm(green), norm(blue))
            }
            Color::Gray { gray } => write!(f, "{}", norm(gray)),
        }
    }
}

/// A transformation matrix for the pdf graphics state.
///
/// Matrixes can be created with numerous named constructors and combined by
/// multiplication.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate simple_pdf;
///
/// # use simple_pdf::units::{Points, UserSpace, LengthUnit};
/// # use simple_pdf::{Pdf, BuiltinFont, FontSource};
/// # use simple_pdf::graphicsstate::Matrix;
/// # use std::io;
///
/// # fn main() -> io::Result<()> {
/// # let mut document: Pdf = Pdf::create("foo.pdf")?;
/// # document.render_page(pt!(180), pt!(240), |canvas| {
///     canvas.concat(Matrix::translate(pt!(10), pt!(24)))?;
///
///     // Matrixes can be combined by multiplication:
///     canvas.concat(
///         (Matrix::translate(pt!(7), pt!(0)) * Matrix::rotate_deg(45.0))
///     )?;
///     // will be visualy identical to:
///     canvas.concat(Matrix::translate(pt!(7), pt!(0)))?;
///     canvas.concat(Matrix::rotate_deg(45.0))?;
/// # Ok(())
/// # })?;
/// # document.finish()
/// }
/// ```
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix {
    v: [f32; 6],
}

impl Matrix {
    /// Construct a matrix for translation
    pub fn translate<T: LengthUnit>(
        dx: UserSpace<T>,
        dy: UserSpace<T>,
    ) -> Self {
        Matrix {
            v: [1., 0., 0., 1., dx.pt as f32, dy.pt as f32],
        }
    }
    /// Construct a matrix for rotating by `a` radians.
    pub fn rotate(alpha: f32) -> Self {
        Matrix {
            v: [alpha.cos(), alpha.sin(), -alpha.sin(), alpha.cos(), 0., 0.],
        }
    }
    /// Construct a matrix for rotating by `a` degrees.
    pub fn rotate_deg(alpha: f32) -> Self {
        Matrix::rotate(alpha * PI / 180.)
    }
    /// Construct a matrix for scaling by factor `sx` in x-direction and by
    /// `sy` in y-direction.
    pub fn scale(sx: f32, sy: f32) -> Self {
        Matrix {
            v: [sx, 0., 0., sy, 0., 0.],
        }
    }
    /// Construct a matrix for scaling by the same factor, `s` in both
    /// directions.
    pub fn uniform_scale(scale: f32) -> Self {
        Matrix::scale(scale, scale)
    }
    /// Construct a matrix for skewing.
    pub fn skew(alpha: f32, beta: f32) -> Self {
        Matrix {
            v: [1., alpha.tan(), beta.tan(), 1., 0., 0.],
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.v;
        write!(f, "{} {} {} {} {} {}", v[0], v[1], v[2], v[3], v[4], v[5])
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, b: Matrix) -> Self::Output {
        let a = self.v;
        let b = b.v;
        Matrix {
            v: [
                a[0] * b[0] + a[1] * b[2],
                a[0] * b[1] + a[1] * b[3],
                a[2] * b[0] + a[3] * b[2],
                a[2] * b[1] + a[3] * b[3],
                a[4] * b[0] + a[5] * b[2] + b[4],
                a[4] * b[1] + a[5] * b[3] + b[5],
            ],
        }
    }
}

#[test]
fn test_matrix_mul_a() {
    assert_unit(Matrix::rotate_deg(45.) * Matrix::rotate_deg(-45.));
}
#[test]
fn test_matrix_mul_b() {
    assert_unit(Matrix::uniform_scale(2.) * Matrix::uniform_scale(0.5));
}
#[test]
fn test_matrix_mul_c() {
    assert_unit(Matrix::rotate(2. * PI));
}
#[test]
fn test_matrix_mul_d() {
    assert_unit(Matrix::rotate(PI) * Matrix::uniform_scale(-1.));
}

#[allow(dead_code)]
fn assert_unit(m: Matrix) {
    assert_eq!(None, diff(&[1., 0., 0., 1., 0., 0.], &m.v));
}

#[allow(dead_code)]
fn diff(a: &[f32; 6], b: &[f32; 6]) -> Option<String> {
    let large_a = a.iter().fold(0f32, |x, &y| x.max(y));
    let large_b = b.iter().fold(0f32, |x, &y| x.max(y));
    let epsilon = 1e-6 * large_a.max(large_b);
    for i in 0..6 {
        if (a[i] - b[i]).abs() > epsilon {
            return Some(format!("{:?} != {:?}", a, b));
        }
    }
    None
}
