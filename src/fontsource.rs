use encoding::{Encoding, FontEncoding, MAC_ROMAN_ENCODING, SYMBOL_ENCODING, WIN_ANSI_ENCODING, ZAPFDINGBATS_ENCODING};
use fontmetrics::{get_builtin_metrics, FontMetrics};
use std::io::{self, Write};
use units::Pt;
use Pdf;

/// The "Base14" built-in fonts in PDF.
/// Underscores in these names are hyphens in the real names.
#[allow(non_camel_case_types, missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum BuiltinFont {
    Courier,
    Courier_Bold,
    Courier_Oblique,
    Courier_BoldOblique,
    Helvetica,
    Helvetica_Bold,
    Helvetica_Oblique,
    Helvetica_BoldOblique,
    Times_Roman,
    Times_Bold,
    Times_Italic,
    Times_BoldItalic,
    Symbol,
    ZapfDingbats,
}

impl FontSource for BuiltinFont {
    fn write_object(&self, pdf: &mut Pdf) -> io::Result<usize> {
        pdf.write_new_object(|font_object_id, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Font /Subtype /Type1 /BaseFont /{} /Encoding /{} >>",
                self.name(),
                if cfg!(target_os = "macos") {
                    "MacRomanEncoding"
                } else {
                    "WinAnsiEncoding"
                }
            )?;
            Ok(font_object_id)
        })
    }

    fn name(&self) -> String {
        use BuiltinFont::*;
        match *self {
            Courier => String::from("Courier"),
            Courier_Bold => String::from("Courier-Bold"),
            Courier_Oblique => String::from("Courier-Oblique"),
            Courier_BoldOblique => String::from("Courier-BoldOblique"),
            Helvetica => String::from("Helvetica"),
            Helvetica_Bold => String::from("Helvetica-Bold"),
            Helvetica_Oblique => String::from("Helvetica-Oblique"),
            Helvetica_BoldOblique => String::from("Helvetica-BoldOblique"),
            Times_Roman => String::from("Times-Roman"),
            Times_Bold => String::from("Times-Bold"),
            Times_Italic => String::from("Times-Italic"),
            Times_BoldItalic => String::from("Times-BoldItalic"),
            Symbol => String::from("Symbol"),
            ZapfDingbats => String::from("ZapfDingbats"),
        }
    }

    fn encoding(&self) -> Encoding {
        match *self {
            BuiltinFont::Symbol => SYMBOL_ENCODING.clone(),
            BuiltinFont::ZapfDingbats => ZAPFDINGBATS_ENCODING.clone(),
            _ => if cfg!(target_os = "macos") {
                MAC_ROMAN_ENCODING.clone()
            } else {
                WIN_ANSI_ENCODING.clone()
            },
        }
    }

    fn text_width<U: Into<Pt>>(&self, size: U, text: &str) -> Pt {
        Pt(size.into().0 * self.raw_text_width(text) as f32 / 1000.0)
    }

    fn raw_text_width(&self, text: &str) -> u32 {
        self.encoding()
            .encode_string(text)
            .iter()
            .map(|&ch| u32::from(self.metrics().get_width(ch).unwrap_or(100)))
            .sum()
    }

    fn metrics(&self) -> FontMetrics {
        get_builtin_metrics(*self).clone()
    }
}

/// Defines a font dictionary to represent text in specified font.
/// At the moment, FontSource only supports Type1 fonts, e.g.
/// the standard fonts (see BuiltinFont).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Font {
    name: String,
    encoding: FontEncoding,
}

impl Font {
    pub fn from_src<F: FontSource>(source: &F) -> Self {
        Self {
            name: source.name(),
            encoding: FontEncoding::with_encoding(source.encoding().clone()),
        }
    }

    pub fn write_object(&self, pdf: &mut Pdf) -> io::Result<usize> {
        pdf.write_new_object(|font_object_id, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Font /Subtype /Type1 /BaseFont /{} \
                 /Encoding /{} >>",
                self.name,
                self.encoding.base()
            )?;
            Ok(font_object_id)
        })
    }
}

/// This trait is implemented by any kind of font that the pdf library
/// supports.
///
/// Currently, only BuiltinFont implements this.
/// TODO Add implementation(s) for other fonts.
pub trait FontSource {
    /// Write the object(s) for this font to a pdf file.
    ///
    /// This is called automatically for each font used in a document.
    /// There should be no need to call this method from user code.
    fn write_object(&self, pdf: &mut Pdf) -> io::Result<usize>;

    /// Get the PDF name of this font.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// assert_eq!("Times-Roman", BuiltinFont::Times_Roman.name());
    /// ```
    fn name(&self) -> String;

    /// Get the encoding that this font uses.
    fn encoding(&self) -> Encoding;

    /// Get the width of a string in this font at given size.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// let proportional = BuiltinFont::Helvetica;
    /// assert_eq!(62.004, proportional.text_width(12.0, "Hello World").0);
    /// let fixed = BuiltinFont::Courier;
    /// assert_eq!(60.0, fixed.text_width(10.0, "0123456789").0);
    /// ```
    fn text_width<U: Into<Pt>>(&self, size: U, text: &str) -> Pt;

    /// Get the width of a string in thousands of unit of text space.
    /// This unit is what is used in some places internally in pdf files.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// assert_eq!(5167, BuiltinFont::Helvetica.raw_text_width("Hello World"));
    /// assert_eq!(600, BuiltinFont::Courier.raw_text_width("A"));
    /// ```
    fn raw_text_width(&self, text: &str) -> u32;

    /// Get the font metrics for font.
    fn metrics(&self) -> FontMetrics;
}
