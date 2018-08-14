use fontref::FontRef;
use fontsource::{Font, FontSource};
use graphicsstate::{CapStyle, Color, JoinStyle, Matrix};
use outline::OutlineItem;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::sync::Arc;
use textobject::TextObject;
use units::{LengthUnit, Points, UserSpace};

/// An visual area where content can be drawn (a page).
///
/// Provides methods for defining and stroking or filling paths, as well as
/// placing text objects.
pub struct Canvas<'a> {
    output: &'a mut BufWriter<File>,
    fonts: &'a mut HashMap<Font, FontRef>,
    outline_items: &'a mut Vec<OutlineItem>,
}

impl<'a> Canvas<'a> {
    // Should not be called by user code.
    pub(crate) fn new(
        output: &'a mut BufWriter<File>,
        fonts: &'a mut HashMap<Font, FontRef>,
        outline_items: &'a mut Vec<OutlineItem>,
    ) -> Canvas<'a> {
        Canvas {
            output,
            fonts,
            outline_items,
        }
    }
    /// Append a closed rectangle with a corner at (x, y) and extending width ×
    /// height to the to the current path.
    pub fn rectangle<T: LengthUnit>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
        width: UserSpace<T>,
        height: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} {} {} {} re", x, y, width, height)
    }
    /// Set the line join style in the graphics state.
    pub fn set_line_join_style(&mut self, style: JoinStyle) -> Result<()> {
        writeln!(self.output, "{} j", style)
    }
    /// Set the line join style in the graphics state.
    pub fn set_line_cap_style(&mut self, style: CapStyle) -> Result<()> {
        writeln!(self.output, "{} J", style)
    }
    /// Set the line width in the graphics state.
    pub fn set_line_width<T: LengthUnit>(
        &mut self,
        width: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} w", width)
    }
    /// Set the line dash pattern in the graphics state. Values must not be
    /// negative, or all array values must not be 0. If this happens the
    /// operation is not executed.
    pub fn set_dash<T: LengthUnit>(
        &mut self,
        pattern: &[UserSpace<T>],
        phase: UserSpace<T>,
    ) -> Result<()> {
        // a valid array must not be empty and have non-negative values plus
        // not all values are allowed to be 0
        let mut valid_array = false;
        for dash in pattern {
            if dash < &pt!(0) {
                return Ok(());
            } else if dash > &pt!(0) {
                valid_array = true;
            }
        }
        if !valid_array {
            return Ok(());
        }
        write!(self.output, "[{}", pattern[0])?;
        for dash in pattern.iter().skip(1) {
            write!(self.output, " {}", dash)?;
        }
        writeln!(self.output, "] {} d", phase)
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

    /// Modify the current transformation matrix for coordinates by
    /// concatenating the specified matrix.
    pub fn concat(&mut self, matrix: Matrix) -> Result<()> {
        writeln!(self.output, "{} cm", matrix)
    }

    /// Append a straight line from (x1, y1) to (x2, y2) to the current path.
    pub fn line<T: LengthUnit>(
        &mut self,
        x1: UserSpace<T>,
        y1: UserSpace<T>,
        x2: UserSpace<T>,
        y2: UserSpace<T>,
    ) -> Result<()> {
        self.move_to(x1, y1)?;
        self.line_to(x2, y2)
    }
    /// Add a straight line from the current point to (x, y) to the current
    /// path.
    pub fn line_to<T: LengthUnit>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
    ) -> Result<()> {
        write!(self.output, "{} {} l ", x, y)
    }
    /// Begin a new subpath at the point (x, y).
    pub fn move_to<T: LengthUnit>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
    ) -> Result<()> {
        write!(self.output, "{} {} m ", x, y)
    }
    /// Add an Bézier curve from the current point to (x3, y3) with (x1, y1)
    /// and (x2, y2) as Bézier control points.
    pub fn curve_to<T: LengthUnit>(
        &mut self,
        x1: UserSpace<T>,
        y1: UserSpace<T>,
        x2: UserSpace<T>,
        y2: UserSpace<T>,
        x3: UserSpace<T>,
        y3: UserSpace<T>,
    ) -> Result<()> {
        writeln!(self.output, "{} {} {} {} {} {} c", x1, y1, x2, y2, x3, y3)
    }
    /// Add a circle approximated by four cubic Bézier curves to the current
    /// path. Based on http://spencermortensen.com/articles/bezier-circle/.
    pub fn circle<T: LengthUnit>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
        r: UserSpace<T>,
    ) -> Result<()> {
        let top = y - r;
        let bottom = y + r;
        let left = x - r;
        let right = x + r;
        // actual value 0.551_915_024_494;
        // f32 truncates value
        let c = 0.551_915_05;
        let dist = r * c;
        let up = y - dist;
        let down = y + dist;
        let leftp = x - dist;
        let rightp = x + dist;
        self.move_to(x, top)?;
        self.curve_to(leftp, top, left, up, left, y)?;
        self.curve_to(left, down, leftp, bottom, x, bottom)?;
        self.curve_to(rightp, bottom, right, down, right, y)?;
        self.curve_to(right, up, rightp, top, x, top)?;
        Ok(())
    }
    /// Stroke the current path.
    pub fn stroke(&mut self) -> Result<()> {
        writeln!(self.output, "S")
    }
    /// Close and stroke the current path.
    pub fn close_and_stroke(&mut self) -> Result<()> {
        writeln!(self.output, "s")
    }
    /// Fill the current path.
    pub fn fill(&mut self) -> Result<()> {
        writeln!(self.output, "f")
    }
    /// Get a FontRef for a specific font.
    pub fn get_font<F: FontSource>(&mut self, font: &F) -> FontRef {
        let next_n = self.fonts.len();
        self.fonts
            .entry(Font::from_src(font))
            .or_insert_with(|| {
                FontRef::new(
                    next_n,
                    font.encoding().clone(),
                    Arc::new(font.metrics()),
                )
            })
            .clone()
    }

    /// Create a text object.
    ///
    /// The contents of the text object is defined by the function
    /// `render_text`, by applying methods to the TextObject it gets as an
    /// argument. On success, returns the value returned by `render_text`.
    pub fn text<F, T>(&mut self, render_text: F) -> Result<T>
    where
        F: FnOnce(&mut TextObject) -> Result<T>,
    {
        writeln!(self.output, "BT")?;
        let result = render_text(&mut TextObject::new(self.output))?;
        writeln!(self.output, "ET")?;
        Ok(result)
    }
    /// Utility method for placing a string of text to the left.
    pub fn left_text<F, T>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
        font: &F,
        size: UserSpace<T>,
        text: &str,
    ) -> Result<()>
    where
        F: FontSource,
        T: LengthUnit,
    {
        let font = self.get_font(font);
        self.text(|t| {
            t.set_font(&font, size)?;
            t.pos(x, y)?;
            t.show(text)
        })
    }
    /// Utility method for placing a string of text to the right.
    pub fn right_text<F, T>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
        font: &F,
        size: UserSpace<T>,
        text: &str,
    ) -> Result<()>
    where
        F: FontSource,
        T: LengthUnit,
    {
        let font = self.get_font(font);
        self.text(|t| {
            let text_width = font.text_width(size, text);
            t.set_font(&font, size)?;
            t.pos(x - text_width, y)?;
            t.show(text)
        })
    }
    /// Utility method for placing a string of text in the center.
    pub fn center_text<F, T>(
        &mut self,
        x: UserSpace<T>,
        y: UserSpace<T>,
        font: &F,
        size: UserSpace<T>,
        text: &str,
    ) -> Result<()>
    where
        F: FontSource,
        T: LengthUnit,
    {
        let font = self.get_font(font);
        self.text(|t| {
            let text_width = font.text_width(size, text);
            t.set_font(&font, size)?;
            t.pos(x - text_width / 2.0, y)?;
            t.show(text)
        })
    }

    /// Add an item for this page in the document outline.
    ///
    /// An outline item associates a name (contained in an ordered tree) with a
    /// location in the document. The PDF standard supports several ways to
    /// specify an exact location on a page, but this implementation currently
    /// only supports linking to a specific page (the page that this Canvas is
    /// for).
    pub fn add_outline(&mut self, title: &str) {
        self.outline_items.push(OutlineItem::new(title));
    }

    /// Save the current graphics state.
    /// The caller is responsible for restoring it later.
    pub fn gsave(&mut self) -> Result<()> {
        writeln!(self.output, "q")
    }
    /// Restore the current graphics state.
    /// The caller is responsible for having saved it earlier.
    pub fn grestore(&mut self) -> Result<()> {
        writeln!(self.output, "Q")
    }
}
