use encoding::{
    get_base_enc, Encoding, FontEncoding, SYMBOL_ENCODING,
    ZAPFDINGBATS_ENCODING,
};
use fontmetrics::{get_builtin_metrics, FontMetrics};
use std::fmt;
use std::io::{Result, Write};
use units::{LengthUnit, UserSpace};
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
    fn write_object(&self, pdf: &mut Pdf) -> Result<usize> {
        pdf.write_new_object(|font_object_id, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Font /Subtype /Type1 /BaseFont /{} /Encoding /{} >>",
                *self,
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
        format!("{}", *self)
    }

    fn encoding(&self) -> &'static Encoding {
        match *self {
            BuiltinFont::Symbol => &SYMBOL_ENCODING,
            BuiltinFont::ZapfDingbats => &ZAPFDINGBATS_ENCODING,
            _ => get_base_enc().to_encoding(),
        }
    }

    fn text_width<T: LengthUnit>(
        &self,
        size: UserSpace<T>,
        text: &str,
    ) -> UserSpace<T> {
        size * self.raw_text_width(text) as f32 / 1000.0
    }

    fn raw_text_width(&self, text: &str) -> u32 {
        self.encoding()
            .encode_string(text)
            .iter()
            .fold(0, |result, &ch| {
                result + u32::from(self.metrics().get_width(ch).unwrap_or(100))
            })
    }

    fn metrics(&self) -> FontMetrics {
        get_builtin_metrics(*self).clone()
    }
}

impl fmt::Display for BuiltinFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            BuiltinFont::Courier => "Courier",
            BuiltinFont::Courier_Bold => "Courier-Bold",
            BuiltinFont::Courier_Oblique => "Courier-Oblique",
            BuiltinFont::Courier_BoldOblique => "Courier-BoldOblique",
            BuiltinFont::Helvetica => "Helvetica",
            BuiltinFont::Helvetica_Bold => "Helvetica-Bold",
            BuiltinFont::Helvetica_Oblique => "Helvetica-Oblique",
            BuiltinFont::Helvetica_BoldOblique => "Helvetica-BoldOblique",
            BuiltinFont::Times_Roman => "Times-Roman",
            BuiltinFont::Times_Bold => "Times-Bold",
            BuiltinFont::Times_Italic => "Times-Italic",
            BuiltinFont::Times_BoldItalic => "Times-BoldItalic",
            BuiltinFont::Symbol => "Symbol",
            BuiltinFont::ZapfDingbats => "ZapfDingbats",
        };
        write!(f, "{}", name)
    }
}

/// Defines a font dictionary to represent text in specified font. At the
/// moment, FontSource only supports Type1 fonts, e.g. the standard fonts (see
/// BuiltinFont).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Font {
    name: String,
    encoding: FontEncoding,
}

impl Font {
    pub fn from_src<F: FontSource>(source: &F) -> Self {
        Font {
            name: source.name(),
            encoding: FontEncoding::with_encoding(source.encoding().clone()),
        }
    }

    pub fn write_object(&self, pdf: &mut Pdf) -> Result<usize> {
        pdf.write_new_object(|font_object_id, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Font /Subtype /Type1 /BaseFont /{} \
                 /Encoding /{} >>",
                self.name,
                self.encoding.base_name()
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
    fn write_object(&self, pdf: &mut Pdf) -> Result<usize>;

    /// Get the PDF name of this font.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// assert_eq!("Times-Roman", BuiltinFont::Times_Roman.name());
    /// ```
    fn name(&self) -> String;

    /// Get the encoding that this font uses.
    fn encoding(&self) -> &Encoding;

    /// Get the width of a string in this font at given size.
    ///
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate simple_pdf;
    ///
    /// use simple_pdf::units::{LengthUnit, Points, UserSpace};
    /// use simple_pdf::{BuiltinFont, FontSource};
    ///
    /// fn main() {
    ///     let proportional = BuiltinFont::Helvetica;
    ///     assert_eq!(
    ///         pt!(62.004),
    ///         proportional.text_width(pt!(12), "Hello World")
    ///     );
    ///     let fixed = BuiltinFont::Courier;
    ///     assert_eq!(pt!(60.0), fixed.text_width(pt!(10), "0123456789"));
    /// }
    /// ```
    fn text_width<T: LengthUnit>(
        &self,
        size: UserSpace<T>,
        text: &str,
    ) -> UserSpace<T>;

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
