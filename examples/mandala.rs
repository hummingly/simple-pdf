///! Example program drawing mandalas on a page.
#[macro_use]
extern crate simple_pdf;

use simple_pdf::graphicsstate::{Color, Matrix};
use simple_pdf::units::{Points, UserSpace};
use simple_pdf::Pdf;
use std::env;
use std::f32::consts::PI;
use std::io;

/// Create a `mandala.pdf` file.
fn main() -> io::Result<()> {
    // Open our pdf document.
    let mut document =
        Pdf::create("mandala.pdf").expect("Could not create file.");
    let mut args = env::args().skip(1);
    let n: u8 = args.next().map(|s| s.parse().expect("number")).unwrap_or(7);

    // Render a page with something resembling a mandala on it.
    document.render_page(pt!(600), pt!(600), |c| {
        c.concat(Matrix::translate(pt!(300), pt!(300)))?;
        c.set_stroke_color(Color::gray(0))?;
        let segment = 2. * PI / n as f32;
        for _i in 0..n {
            c.move_to(pt!(0), pt!(33.5))?;
            c.line_to(pt!(0), pt!(250))?;
            let r = pt!(99);
            c.circle(pt!(0), r, r * 1.25 * segment)?;
            let d = pt!(141.4);
            let rr = pt!(36);
            c.circle(pt!(0), d + rr, rr)?;
            c.stroke()?;
            c.concat(Matrix::rotate(segment))?;
        }
        c.concat(Matrix::rotate(segment / 2.))?;
        for _i in 0..n {
            let mut r0 = pt!(58.66);
            let mut r = 0.7705 * r0 * segment;
            for _j in 0..(n + 1) / 3 {
                c.circle(pt!(0), r0, r)?;
                let r2 = 1.058 * r;
                r0 = r0 + r + r2;
                r = r2;
            }
            c.stroke()?;
            c.concat(Matrix::rotate(segment))?;
        }
        Ok(())
    })?;
    document.finish()
}
