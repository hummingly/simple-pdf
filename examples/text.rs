//! Example program drawing some text on a page.
extern crate simple_pdf;

use simple_pdf::graphicsstate::Color;
use simple_pdf::units::{Mm, Pt};
use simple_pdf::{BuiltinFont, Pdf};

/// Create a `text.pdf` file, with a single page containg some
/// text lines positioned in various ways on some helper lines.
fn main() {
    let mut document = Pdf::create("text.pdf").unwrap();
    document.set_title("Text example");

    let h: Pt = Mm(297.0).into();
    let w: Pt = Mm(210.0).into();

    document
        .render_page(Mm(210.0), Mm(297.0), |c| {
            c.set_stroke_color(Color::rgb(200, 200, 255))?;
            c.set_dash(&[Pt(1.0), Pt(2.5)], -0.5)?;
            c.rectangle(10.0, 10.0, w.0 - 20.0, h.0 - 20.0)?;
            c.line(10.0, h.0 / 2.0, w.0 - 10.0, h.0 / 2.0)?;
            c.line(w.0 / 2.0, 10.0, w.0 / 2.0, h.0 - 10.0)?;
            c.stroke()?;
            let helvetica = BuiltinFont::Helvetica;
            c.left_text(10.0, h.0 - 20.0, &helvetica, 12.0, "Top left")?;
            c.left_text(10.0, 10.0, &helvetica, 12.0, "Bottom left")?;
            c.right_text(
                w.0 - 10.0,
                h.0 - 20.0,
                &helvetica,
                12.0,
                "Top right"
            )?;
            c.right_text(w.0 - 10.0, 10.0, &helvetica, 12.0, "Bottom right")?;
            c.center_text(
                w.0 / 2.0,
                h.0 - 30.0,
                &BuiltinFont::Times_Bold,
                24.0,
                "Centered"
            )?;
            let times = c.get_font(&BuiltinFont::Times_Roman);
            c.text(|t| {
                t.set_font(&times, 14.0)?;
                t.set_leading(18.0)?;
                t.pos(10.0, h.0 - 100.0)?;
                t.show("Some lines of text in what might look like a")?;
                t.show_line("paragraph of three lines. Lorem ipsum dolor")?;
                t.show_line("sit amet. Blahonga. ")?;
                t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)])?;
                t.pos(0., -30.)?;
                t.show_adjusted(
                    &(-19..21).map(|i| ("o", 16 * i)).collect::<Vec<_>>()
                )
            })?;

            //In Swedish, we use the letters å, ä, and ö
            //in words like sloe liqueur.  That is why rust-pdf
            //uses /WinAnsiEncoding for text.
            let times_italic = BuiltinFont::Times_Italic;
            c.right_text(
                w.0 - 10.0,
                500.0,
                &times_italic,
                14.0,
                "På svenska använder vi bokstäverna å, ä & ö"
            )?;
            c.right_text(
                w.0 - 10.0,
                480.0,
                &times_italic,
                14.0,
                "i ord som slånbärslikör. Därför använder"
            )?;

            c.right_text(
                w.0 - 10.0,
                460.0,
                &times_italic,
                14.0,
                "rust-pdf /WinAnsiEncoding för text."
            )?;

            c.center_text(
                w.0 / 2.0,
                400.0,
                &BuiltinFont::Symbol,
                14.0,
                "Hellas ΑΒΓΔαβγδ"
            )?;
            c.center_text(
                w.0 / 2.0,
                380.0,
                &BuiltinFont::Symbol,
                14.0,
                "∀ μ < δ : ∃ σ ∈ Σ"
            )?;

            c.center_text(
                w.0 / 2.0,
                320.0,
                &BuiltinFont::ZapfDingbats,
                18.0,
                "☎  ✌  ✖  ✤  ✰ ✴  ❐  ❝  ❤  ❞"
            )?;
            Ok(())
        }).unwrap();
    document.finish().unwrap();
}
