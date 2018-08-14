//! Example program drawing some text on a page.
#[macro_use]
extern crate simple_pdf;

use simple_pdf::graphicsstate::Color;
use simple_pdf::units::{Millimeters, Points, UserSpace};
use simple_pdf::{BuiltinFont, Pdf};
use std::io;

/// Create a `text.pdf` file, with a single page containg some
/// text lines positioned in various ways on some helper lines.
fn main() -> io::Result<()> {
    let mut document = Pdf::create("text.pdf").expect("Could not create file.");
    document.set_title("Text example");

    let h = pt!(mm!(297));
    let w = pt!(mm!(210));

    document.render_page(w, h, |c| {
        c.set_stroke_color(Color::rgb(200, 200, 255))?;
        c.set_dash(&[pt!(0), pt!(0), pt!(0)], pt!(-0.5))?;
        c.rectangle(pt!(10), pt!(10), w - pt!(20), h - pt!(20))?;
        c.line(pt!(10), h / 2, w - pt!(10), h / 2)?;
        c.line(w / 2, pt!(10), w / 2, h - pt!(10))?;
        c.stroke()?;
        let helvetica = BuiltinFont::Helvetica;
        c.left_text(pt!(10), h - pt!(20), &helvetica, pt!(12), "Top left")?;
        c.left_text(pt!(10), pt!(10), &helvetica, pt!(12), "Bottom left")?;
        c.right_text(
            w - pt!(10),
            h - pt!(20),
            &helvetica,
            pt!(12),
            "Top right",
        )?;
        c.right_text(
            w - pt!(10),
            pt!(10),
            &helvetica,
            pt!(12),
            "Bottom right",
        )?;
        c.center_text(
            w / 2,
            h - pt!(30),
            &BuiltinFont::Times_Bold,
            pt!(24),
            "Centered",
        )?;
        let times = c.get_font(&BuiltinFont::Times_Roman);
        c.text(|t| {
            t.set_font(&times, pt!(14))?;
            t.set_leading(pt!(18))?;
            t.pos(pt!(10), h - pt!(100))?;
            t.show("Some lines of text in what might look like a")?;
            t.show_line("paragraph of three lines. Lorem ipsum dolor")?;
            t.show_line("sit amet. Blahonga. ")?;
            t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)])?;
            t.pos(pt!(0), pt!(-30))?;
            t.show_adjusted(
                &(-19..21).map(|i| ("o", 16 * i)).collect::<Vec<_>>(),
            )
        })?;

        //In Swedish, we use the letters å, ä, and ö
        //in words like sloe liqueur.  That is why rust-pdf
        //uses /WinAnsiEncoding for text.
        let times_italic = BuiltinFont::Times_Italic;
        c.right_text(
            w - pt!(10),
            pt!(500),
            &times_italic,
            pt!(14),
            "På svenska använder vi bokstäverna å, ä & ö",
        )?;
        c.right_text(
            w - pt!(10),
            pt!(480),
            &times_italic,
            pt!(14),
            "i ord som slånbärslikör. Därför använder",
        )?;

        c.right_text(
            w - pt!(10),
            pt!(460),
            &times_italic,
            pt!(14),
            "rust-pdf /WinAnsiEncoding för text.",
        )?;

        c.center_text(
            w / 2,
            pt!(400),
            &BuiltinFont::Symbol,
            pt!(14),
            "Hellas ΑΒΓΔαβγδ",
        )?;
        c.center_text(
            w / 2,
            pt!(380),
            &BuiltinFont::Symbol,
            pt!(14),
            "∀ μ < δ : ∃ σ ∈ Σ",
        )?;

        c.center_text(
            w / 2,
            pt!(320),
            &BuiltinFont::ZapfDingbats,
            pt!(18),
            "☎  ✌  ✖  ✤  ✰ ✴  ❐  ❝  ❤  ❞",
        )?;
        Ok(())
    })?;
    document.finish()
}
