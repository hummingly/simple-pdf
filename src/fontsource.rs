use encoding::{Encoding, FontEncoding, SYMBOL_ENCODING, ZAPFDINGBATS_ENCODING};
use fontmetrics::{get_builtin_metrics, FontMetrics};
use std::io::{self, Write};
use std::ops::Add;
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

use BuiltinFont::*;
impl BuiltinFont {
    fn name(self) -> String {
        match self {
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
}

impl From<BuiltinFont> for FontSource {
    fn from(font: BuiltinFont) -> Self {
        let metrics = get_builtin_metrics(&font).clone();
        let encoding = match font {
            Symbol => FontEncoding::with_encoding(SYMBOL_ENCODING.clone()),
            ZapfDingbats => FontEncoding::with_encoding(ZAPFDINGBATS_ENCODING.clone()),
            _ => FontEncoding::new(),
        };
        FontSource {
            name: font.name(),
            encoding,
            metrics,
        }
    }
}

/// Defines a font dictionary to represent text in specified font.
/// At the moment, FontSource only supports Type1 fonts, e.g. 
/// the standard fonts (see BuiltinFont).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontSource {
    name: String,
    encoding: FontEncoding,
    metrics: FontMetrics,
}

impl FontSource {
    /// This creates a new font resource to embed into the PDF document.
    pub fn new(name: String, encoding: FontEncoding, metrics: FontMetrics) -> Self {
        FontSource {
            metrics,
            name,
            // must be a ref number
            // descriptor: FontDescriptor,
            encoding,
            // to_unicode: UnicodeMap,
        }
    }

    /// Write the object(s) for this font to a pdf file.
    ///
    /// This is called automatically for each font used in a document.
    /// There should be no need to call this method from user code.
    pub(crate) fn write_object(&self, pdf: &mut Pdf) -> io::Result<usize> {
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

    /// Get the PDF name of this font.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// assert_eq!("Times-Roman", FontSource::from(BuiltinFont::Times_Roman).name());
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the encoding that this font uses.
    pub fn encoding(&self) -> Encoding {
        self.encoding.encoding()
    }

    /// Get the width of a string in this font at given size.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// let proportional = FontSource::from(BuiltinFont::Helvetica);
    /// assert_eq!(62.004, proportional.text_width(12.0, "Hello World"));
    /// let fixed = FontSource::from(BuiltinFont::Courier);
    /// assert_eq!(60.0, fixed.text_width(10.0, "0123456789"));
    /// ```
    pub fn text_width(&self, size: f32, text: &str) -> f32 {
        size * self.raw_text_width(text) as f32 / 1000.0
    }

    /// Get the width of a string in thousands of unit of text space.
    /// This unit is what is used in some places internally in pdf files.
    ///
    /// # Examples
    /// ```
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// assert_eq!(5167, FontSource::from(BuiltinFont::Helvetica).raw_text_width("Hello World"));
    /// assert_eq!(600, FontSource::from(BuiltinFont::Courier).raw_text_width("A"));
    /// ```
    pub fn raw_text_width(&self, text: &str) -> u32 {
        self.encoding
            .encode_string(text)
            .iter()
            .map(|&ch| u32::from(self.metrics.get_width(ch).unwrap_or(100)))
            .fold(0, Add::add)
    }

    /// Get the font metrics for font.
    pub fn metrics(&self) -> FontMetrics {
        self.metrics.clone()
    }
}
