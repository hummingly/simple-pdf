//! Example program drawing some text on a page.
extern crate simple_pdf;

use simple_pdf::graphicsstate::Color;
use simple_pdf::units::{Mm, Pt};
use simple_pdf::{BuiltinFont, FontSource, Pdf};

/// Create a `text.pdf` file, with a single page containg some
/// text lines positioned in various ways on some helper lines.
fn main() {
    let mut document = Pdf::create("text.pdf").unwrap();
    document.set_title("Text example");
    // let h: Pt = Mm(297.0).into();
    // let w: Pt = Mm(210.0).into();
    let h: Pt = Mm(297.0).into();
    let w: Pt = Mm(210.0).into();
    document
        .render_page(w, h, |c| {
            try!(c.set_stroke_color(Color::rgb(200, 200, 255)));
            try!(c.rectangle(10.0, 10.0, w.0 - 20.0, h.0 - 20.0));
            try!(c.line(10.0, h.0 / 2.0, w.0 - 10.0, h.0 / 2.0));
            try!(c.line(w.0 / 2.0, 10.0, w.0 / 2.0, h.0 - 10.0));
            try!(c.stroke());
            let helvetica = FontSource::from(BuiltinFont::Helvetica);
            try!(c.left_text(10.0, h.0 - 20.0, &helvetica, 12.0, "Top left"));
            try!(c.left_text(10.0, 10.0, &helvetica, 12.0, "Bottom left"));
            try!(c.right_text(w.0 - 10.0, h.0 - 20.0, &helvetica, 12.0, "Top right"));
            try!(c.right_text(w.0 - 10.0, 10.0, &helvetica, 12.0, "Bottom right"));
            try!(c.center_text(
                w.0 / 2.0,
                h.0 - 30.0,
                &BuiltinFont::Times_Bold.into(),
                24.0,
                "Centered"
            ));
            let times = c.get_font(&BuiltinFont::Times_Roman.into());
            try!(c.text(|t| {
                try!(t.set_font(&times, 14.0));
                try!(t.set_leading(18.0));
                try!(t.pos(10.0, h.0 - 100.0));
                try!(t.show("Some lines of text in what might look like a"));
                try!(t.show_line("paragraph of three lines. Lorem ipsum dolor"));
                try!(t.show_line("sit amet. Blahonga. "));
                try!(t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)]));
                try!(t.pos(0., -30.));
                t.show_adjusted(&(-19..21).map(|i| ("o", 16 * i)).collect::<Vec<_>>())
            }));

            //In Swedish, we use the letters å, ä, and ö
            //in words like sloe liqueur.  That is why rust-pdf
            //uses /WinAnsiEncoding for text.
            let times_italic = FontSource::from(BuiltinFont::Times_Italic);
            try!(c.right_text(
                w.0 - 10.0,
                500.0,
                &times_italic,
                14.0,
                "På svenska använder vi bokstäverna å, ä & ö"
            ));
            try!(c.right_text(
                w.0 - 10.0,
                480.0,
                &times_italic,
                14.0,
                "i ord som slånbärslikör. Därför använder"
            ));
            try!(c.right_text(
                w.0 - 10.0,
                460.0,
                &times_italic,
                14.0,
                "rust-pdf /WinAnsiEncoding för text."
            ));

            try!(c.center_text(
                w.0 / 2.0,
                400.0,
                &BuiltinFont::Symbol.into(),
                14.0,
                "Hellas ΑΒΓΔαβγδ"
            ));
            try!(c.center_text(
                w.0 / 2.0,
                380.0,
                &BuiltinFont::Symbol.into(),
                14.0,
                "∀ μ < δ : ∃ σ ∈ Σ"
            ));

            try!(c.center_text(
                w.0 / 2.0,
                320.0,
                &BuiltinFont::ZapfDingbats.into(),
                18.0,
                "☎  ✌  ✖  ✤  ✰ ✴  ❐  ❝  ❤  ❞"
            ));
            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
}
