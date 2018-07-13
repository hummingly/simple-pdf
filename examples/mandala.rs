///! Example program drawing mandalas on a page.
extern crate simple_pdf;

use simple_pdf::graphicsstate::{Color, Matrix};
use simple_pdf::units::Pt;
use simple_pdf::Pdf;
use std::env;
use std::f32::consts::PI;

/// Create a `mandala.pdf` file.
fn main() {
    // Open our pdf document.
    let mut document = Pdf::create("mandala.pdf").expect("Create PDF file");
    let mut args = env::args().skip(1);
    let n: u8 = args.next().map(|s| s.parse().expect("number")).unwrap_or(7);

    // Render a page with something resembling a mandala on it.
    document
        .render_page(Pt(600.0), Pt(600.0), |c| {
            try!(c.concat(&Matrix::translate(300., 300.)));
            try!(c.set_stroke_color(Color::gray(0)));
            let segment = 2. * PI / n as f32;
            for _i in 0..n {
                try!(c.move_to(0., 33.5));
                try!(c.line_to(0., 250.));
                let r = 99.;
                try!(c.circle(0., r, r * 1.25 * segment));
                let d = 141.4;
                let rr = 36.;
                try!(c.circle(0., d + rr, rr));
                try!(c.stroke());
                try!(c.concat(&Matrix::rotate(segment)));
            }
            try!(c.concat(&Matrix::rotate(segment / 2.)));
            for _i in 0..n {
                let mut r0 = 58.66;
                let mut r = 0.7705 * r0 * segment;
                for _j in 0..(n + 1) / 3 {
                    try!(c.circle(0., r0, r));
                    let r2 = 1.058 * r;
                    r0 = r0 + r + r2;
                    r = r2;
                }
                try!(c.stroke());
                try!(c.concat(&Matrix::rotate(segment)));
            }

            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
}
