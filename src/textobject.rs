use encoding::{get_base_enc, Encoding};
use fontref::FontRef;
use graphicsstate::Color;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use units::{LengthUnit, UserSpace};

/// A text object is where text is put on the canvas.
///
/// A TextObject should never be created directly by the user. Instead, the
/// [Canvas.text](struct.Canvas.html#method.text) method should be called.
/// It will create a TextObject and call a callback, before terminating the
/// text object properly.
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate simple_pdf;
/// # use simple_pdf::units::{Points, UserSpace, LengthUnit};
/// # use simple_pdf::{Pdf, BuiltinFont, FontSource};
/// # use simple_pdf::graphicsstate::Matrix;
/// # use std::io;
/// # fn main() -> io::Result<()> {
/// # let mut document = Pdf::create("foo.pdf")?;
/// # document.render_page(pt!(180), pt!(240), |canvas| {
///     let serif = canvas.get_font(&BuiltinFont::Times_Roman);
///     // t will be a TextObject
///     canvas.text(|t| {
///         t.set_font(&serif, pt!(14))?;
///         t.set_leading(pt!(18))?;
///         t.pos(pt!(10), pt!(300))?;
///         t.show("Some lines of text in what might look like a")?;
///         t.show_line("paragraph of three lines. Lorem ipsum dolor")?;
///         t.show_line("sit amet. Blahonga.")
///     })?;
/// # Ok(())
/// # })?;
/// # document.finish()
/// # }
/// ```
pub struct TextObject<'a> {
    output: &'a mut BufWriter<File>,
    encoding: Encoding,
}

impl<'a> TextObject<'a> {
    // Should not be called by user code.
    pub(crate) fn new(output: &'a mut BufWriter<File>) -> Self {
        TextObject {
            output,
            encoding: get_base_enc().to_encoding().clone(),
        }
    }
    /// Set the font and font-size to be used by the following text operations.
    pub fn set_font<T: LengthUnit>(
        &mut self,
        font: &FontRef,
        size: UserSpace<T>,
    ) -> Result<()> {
        self.encoding = font.encoding().clone();
        writeln!(self.output, "{} {} Tf", font, size)
    }
    /// Set text render mode, which enables rendering text filled, stroked or
    /// as clipping boundary.
    pub fn set_render_mode(&mut self, mode: RenderMode) -> Result<()> {
        writeln!(self.output, "{} Tr", mode)
    }
    /// Set leading, the vertical distance from a line of text to the next.
    /// This is important for the [show_line](#method.show_line) method.
    pub fn set_leading<T: LengthUnit>(
        &mut self,
        leading: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} TL", leading)
    }
    /// Set the rise above the baseline for coming text. Calling set_rise again
    /// with a zero argument will get back to the old baseline.
    pub fn set_rise<T: LengthUnit>(
        &mut self,
        rise: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} Ts", rise)
    }
    /// Set the amount of extra space between characters, in 1/1000 text unit.
    pub fn set_char_spacing<T: LengthUnit>(
        &mut self,
        c_space: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} Tc", c_space)
    }
    /// Set the amount of extra space between words, in 1/1000 text unit.
    pub fn set_word_spacing<T: LengthUnit>(
        &mut self,
        w_space: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} Tw", w_space)
    }

    /// Set color for stroking operations.
    pub fn set_stroke_color(&mut self, color: Color) -> Result<()> {
        match color {
            Color::RGB { .. } => writeln!(self.output, "{} SC", color),
            Color::Gray { .. } => writeln!(self.output, "{} G", color),
        }
    }
    /// Set color for non-stroking operations.
    pub fn set_fill_color(&mut self, color: Color) -> Result<()> {
        match color {
            Color::RGB { .. } => writeln!(self.output, "{} sc", color),
            Color::Gray { .. } => writeln!(self.output, "{} g", color),
        }
    }

    /// Move text position.
    ///
    /// The first time `pos` is called in a TextObject, (x, y) refers to the
    /// same point as for [Canvas::move_to](struct.Canvas.html#method.move_to),
    /// after that, the point is relative to the earlier pos.
    pub fn pos<T: LengthUnit>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} {} Td", x, y)
    }
    /// Show a text.
    pub fn show(&mut self, text: &str) -> Result<()> {
        write!(self.output, "(")?;
        self.output.write_all(&self.encoding.encode_string(text))?;
        writeln!(self.output, ") Tj")
    }

    /// Show one or more text strings, allowing individual glyph positioning.
    ///
    /// Each item in param should contain a string to show and a number to
    /// adjust the position. The adjustment is measured in thousands of unit of
    /// text space. Positive adjustment brings letters closer, negative widens
    /// the gap.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate simple_pdf;
    /// # use simple_pdf::units::{Points, UserSpace, LengthUnit};
    /// # use simple_pdf::{Pdf, BuiltinFont, FontSource};
    /// # use simple_pdf::graphicsstate::Matrix;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// # let mut document = Pdf::create("foo.pdf")?;
    /// # document.render_page(pt!(180), pt!(240), |canvas| {
    /// # let serif = canvas.get_font(&BuiltinFont::Times_Roman);
    /// # canvas.text(|t| {
    /// # t.set_font(&serif, pt!(14))?;
    /// t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)])
    /// # })
    /// # })?;
    /// # document.finish()
    /// # }
    /// ```
    pub fn show_adjusted(&mut self, param: &[(&str, i32)]) -> Result<()> {
        write!(self.output, "[")?;
        for &(text, offset) in param {
            write!(self.output, "(")?;
            self.output.write_all(&self.encoding.encode_string(text))?;
            write!(self.output, ") {} ", offset)?;
        }
        writeln!(self.output, "] TJ")
    }
    /// Show a text as a line.  See also [set_leading](#method.set_leading).
    pub fn show_line(&mut self, text: &str) -> Result<()> {
        write!(self.output, "(")?;
        self.output.write_all(&self.encoding.encode_string(text))?;
        writeln!(self.output, ") '")
    }
    /// Push the graphics state on a stack.
    pub fn gsave(&mut self) -> Result<()> {
        // TODO Push current encoding in self?
        writeln!(self.output, "q")
    }
    /// Pop a graphics state from the [gsave](#method.gsave) stack and restore
    /// it.
    pub fn grestore(&mut self) -> Result<()> {
        // TODO Pop current encoding in self?
        writeln!(self.output, "Q")
    }
}

/// Text rendering modes for the method 
/// [TextObject.set_render_mode](struct.TextObject.html#method.set_render_mode).
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum RenderMode {
    /// Fills text glyphs with nonstroking color.
    Fill,
    /// Draws outline of text glyphs with stroking color.
    Stroke,
    /// Fills, then strokes text.
    FillAndStroke,
    /// Renders the text invisible.
    Invisible,
    /// Adds the filled text glyphs to clipping path.
    FillAndClipping,
    /// Adds the stroked text glyphs to clipping path.
    StrokeAndClipping,
    /// Adds the filled, then stroked text glyphs to clipping path.
    FillAndStrokeAndClipping,
    /// Adds text to clipping path
    Clipping,
}

impl fmt::Display for RenderMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                RenderMode::Fill => 0,
                RenderMode::Stroke => 1,
                RenderMode::FillAndStroke => 2,
                RenderMode::Invisible => 3,
                RenderMode::FillAndClipping => 4,
                RenderMode::StrokeAndClipping => 5,
                RenderMode::FillAndStrokeAndClipping => 6,
                RenderMode::Clipping => 7,
            }
        )
    }
}
