use encoding::Encoding;
use fontmetrics::FontMetrics;
use std::fmt;
use std::sync::Arc;
use units::Pt;

/// A font ready to be used in a TextObject.
///
/// The way to get FontRef is to call
/// [Canvas::get_font](struct.Canvas.html#method.get_font) with a
/// [FontSource](trait.FontSource.html).
/// In PDF terms, a FontSource is everything needed to build a font
/// dictionary, while a FontRef is the name that can be used in a page
/// stream to use a font.
/// Calling Canvas::get_font will make sure the font dictionary is
/// created in the file, associate it with a name in the page
/// resources and return a FontRef representing that name.
///
/// The `serif` variable in
/// [the TextObject example](struct.TextObject.html#example) is a FontRef.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontRef {
    n: usize,
    encoding: Encoding,
    metrics: Arc<FontMetrics>,
}

impl FontRef {
    // Should not be called by user code.
    pub(crate) fn new(n: usize, encoding: Encoding, metrics: Arc<FontMetrics>) -> FontRef {
        FontRef {
            n,
            encoding,
            metrics,
        }
    }
    /// Get the encoding used by the referenced font.
    pub fn encoding(&self) -> Encoding {
        self.encoding.clone()
    }

    /// Get the width of the given text in this font at given size.
    pub fn text_width<U: Into<Pt>>(&self, size: U, text: &str) -> Pt {
        Pt(size.into().0 * self.raw_text_width(text) as f32 / 1000.0)
    }

    /// Get the width of the given text in thousands of unit of text
    /// space.
    /// This unit is what is used in some places internally in pdf files
    /// and in some methods on a [TextObject](struct.TextObject.html).
    pub fn raw_text_width(&self, text: &str) -> u32 {
        let mut result = 0;
        for char in self.encoding.encode_string(text) {
            result += u32::from(self.metrics.get_width(char).unwrap_or(100));
        }
        result
    }
}

impl fmt::Display for FontRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/F{}", self.n)
    }
}
