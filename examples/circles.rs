//! Example program drawing circles on a page.
#[macro_use]
extern crate simple_pdf;

use simple_pdf::graphicsstate::Color;
use simple_pdf::units::{Points, UserSpace};
use simple_pdf::Pdf;
use std::f32::consts::PI;
use std::io;

/// Create a `circles.pdf` file, with a single page containg a circle
/// stroked in black, overwritten with a circle in a finer yellow
/// stroke.
/// The black circle is drawn using the `Canvas.circle` method,
/// which approximates a circle with four bezier curves.
/// The yellow circle is drawn as a 200-sided polygon.
fn main() -> io::Result<()> {
    // Open our pdf document.
    let mut document =
        Pdf::create("circles.pdf").expect("Could not create file.");

    // Add a 400x400 pt page.

    // Render-page writes the pdf file structure for a page and
    // creates a Canvas which is sent to the function that is the last
    // argument of the render_page method.
    // That function then puts content on the page by calling methods
    // on the canvas.
    document.render_page(pt!(400), pt!(400), |c| {
        let (x, y) = (pt!(200), pt!(200));
        let r = pt!(190);

        // Set a wide black pen and stroke a circle
        c.set_stroke_color(Color::rgb(0, 0, 0))?;
        c.set_line_width(pt!(2))?;
        c.circle(x, y, r)?;
        c.stroke()?;

        // Set a finer yellow pen and stroke a 200-sided polygon
        c.set_stroke_color(Color::rgb(255, 230, 150))?;
        c.set_line_width(pt!(1))?;
        c.move_to(x + r, y)?;
        let sides: u8 = 200;
        for n in 1..sides {
            let phi = f32::from(n) * 2.0 * PI / f32::from(sides);
            c.line_to(x + r * phi.cos(), y + r * phi.sin())?;
        }
        c.close_and_stroke()
    })?;
    document.finish()
}
