//! A library for creating pdf files based on [pdf-canvas](https://github.com/kaj/rust-pdf).
//!
//! Currently, simple vector graphics and text set in the 14 built-in fonts are
//! supported. The main entry point of the crate is the [struct
//! Pdf](struct.Pdf.html), representing a PDF file being written.

//! # Example
//!
//! ```
//! use simple_pdf::graphicsstate::Color;
//! use simple_pdf::{BuiltinFont, FontSource, Pdf};
//!
//! let mut document = Pdf::create("example.pdf").expect("Create pdf file");
//! // The 14 builtin fonts are available
//! let font = BuiltinFont::Times_Roman;
//!
//! // Add a page to the document.  This page will be 180 by 240 pt large.
//! document
//!     .render_page(180.0, 240.0, |canvas| {
//!         // This closure defines the content of the page
//!         let hello = "Hello World!";
//!         let w = font.text_width(24.0, hello).0 + 8.0;
//!
//!         // Some simple graphics
//!         canvas.set_stroke_color(Color::rgb(0, 0, 248))?;
//!         canvas.rectangle(90.0 - w / 2.0, 194.0, w, 26.0)?;
//!         canvas.stroke()?;
//!
//!         // Some text
//!         canvas.center_text(90.0, 200.0, &font, 24.0, hello)
//!     })
//!     .expect("Write page");
//! // Write all pending content, including the trailer and index
//! document.finish().expect("Finish pdf document");
//! ```
//!
//! To use this library you need to add it as a dependency in your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! simple-pdf = "0.1"
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

extern crate time;

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Seek, SeekFrom, Write};
use std::mem;

mod fontsource;
use fontsource::Font;
pub use fontsource::{BuiltinFont, FontSource};

mod fontref;
pub use fontref::FontRef;

mod fontmetrics;
pub use fontmetrics::FontMetrics;

mod encoding;
pub use encoding::{Encoding, FontEncoding};

pub mod graphicsstate;

mod outline;
use outline::OutlineItem;

mod canvas;
pub use canvas::Canvas;

mod textobject;
pub use textobject::TextObject;

pub mod units;
use units::Pt;

/// The top-level object for writing a PDF.
///
/// A PDF file is created with the `create` or `new` methods. Some metadata can
/// be stored with `set_foo` methods, and pages are appended with the
/// `render_page` method.
/// Don't forget to call `finish` when done, to write the document trailer,
/// without it the written file won't be a proper PDF.
pub struct Pdf {
    output: BufWriter<File>,
    object_offsets: Vec<i64>,
    page_objects_ids: Vec<usize>,
    font_object_ids: HashMap<Font, usize>,
    outline_items: Vec<OutlineItem>,
    document_info: BTreeMap<String, String>
}

const DEFAULT_BUF_SIZE: usize = 65_536;
const ROOT_OBJECT_ID: usize = 1;
const PAGES_OBJECT_ID: usize = 2;

impl Pdf {
    /// Create a new PDF document as a new file with given filename.
    pub fn create(filename: &str) -> Result<Pdf> {
        let file = File::create(filename)?;
        Pdf::new(file)
    }

    /// Create a new PDF document, writing to `output`.
    pub fn new(mut output: File) -> Result<Pdf> {
        // TODO Maybe use a lower version?  Possibly decide by features used?
        output.write_all(b"%PDF-1.7\n%\xB5\xED\xAE\xFB\n")?;
        Ok(Pdf {
            output: BufWriter::with_capacity(DEFAULT_BUF_SIZE, output),
            // Object ID 0 is special in PDF.
            // We reserve IDs 1 and 2 for the catalog and page tree.
            object_offsets: vec![-1, -1, -1],
            page_objects_ids: Vec::new(),
            font_object_ids: HashMap::new(),
            outline_items: Vec::new(),
            document_info: BTreeMap::new()
        })
    }
    /// Set metadata: the document's title.
    pub fn set_title(&mut self, title: &str) {
        self.document_info
            .insert("Title".to_string(), title.to_string());
    }
    /// Set metadata: the name of the person who created the document.
    pub fn set_author(&mut self, author: &str) {
        self.document_info
            .insert("Author".to_string(), author.to_string());
    }
    /// Set metadata: the subject of the document.
    pub fn set_subject(&mut self, subject: &str) {
        self.document_info
            .insert("Subject".to_string(), subject.to_string());
    }
    /// Set metadata: keywords associated with the document.
    pub fn set_keywords(&mut self, keywords: &str) {
        self.document_info
            .insert("Subject".to_string(), keywords.to_string());
    }
    /// Set metadata: If the document was converted to PDF from another format,
    /// the name of the conforming product that created the original document
    /// from which it was converted.
    pub fn set_creator(&mut self, creator: &str) {
        self.document_info
            .insert("Creator".to_string(), creator.to_string());
    }
    /// Set metadata: If the document was converted to PDF from another format,
    /// the name of the conforming product that converted it to PDF.
    pub fn set_producer(&mut self, producer: &str) {
        self.document_info
            .insert("Producer".to_string(), producer.to_string());
    }

    /// Return the current read/write position in the output file.
    fn tell(&mut self) -> Result<u64> {
        self.output.seek(SeekFrom::Current(0))
    }

    /// Create a new page in the PDF document.
    ///
    /// The page will be `width` x `height` points large, and the actual
    /// content of the page will be created by the function `render_contents` by
    /// applying drawing methods on the Canvas.
    pub fn render_page<F, U>(
        &mut self,
        width: U,
        height: U,
        render_contents: F
    ) -> Result<()>
    where
        F: FnOnce(&mut Canvas) -> Result<()>,
        U: Into<Pt>
    {
        let (contents_object_id, content_length, fonts, outline_items) = self
            .write_new_object(move |contents_object_id, pdf| {
                // Guess the ID of the next object. (We’ll assert it below.)
                writeln!(
                    pdf.output,
                    "<< /Length {} 0 R >>\n\
                     stream",
                    contents_object_id + 1
                )?;

                let start = pdf.tell()?;
                writeln!(pdf.output, "/DeviceRGB cs /DeviceRGB CS")?;
                let mut fonts = HashMap::new();
                let mut outline_items = Vec::new();
                render_contents(&mut Canvas::new(
                    &mut pdf.output,
                    &mut fonts,
                    &mut outline_items
                ))?;
                let end = pdf.tell()?;

                writeln!(pdf.output, "endstream")?;
                Ok((contents_object_id, end - start, fonts, outline_items))
            })?;

        self.write_new_object(|length_object_id, pdf| {
            assert!(length_object_id == contents_object_id + 1);
            writeln!(pdf.output, "{}", content_length)
        })?;

        let mut font_oids = NamedRefs::new();
        for (src, r) in fonts {
            if let Some(&object_id) = self.font_object_ids.get(&src) {
                font_oids.insert(r, object_id);
            } else {
                let object_id = src.write_object(self)?;
                font_oids.insert(r, object_id);
                self.font_object_ids.insert(src, object_id);
            }
        }
        let page_oid = self.write_page_dict(
            contents_object_id,
            width.into(),
            height.into(),
            &font_oids
        )?;
        // Take the outline_items from this page, mark them with the page ref,
        // and save them for the document outline.
        for mut item in outline_items {
            item.set_page(page_oid);
            self.outline_items.push(item);
        }
        self.page_objects_ids.push(page_oid);
        Ok(())
    }

    fn write_page_dict(
        &mut self,
        content_oid: usize,
        width: Pt,
        height: Pt,
        font_oids: &NamedRefs
    ) -> Result<usize> {
        self.write_new_object(|page_oid, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Page\n   \
                 /Parent {parent} 0 R\n   \
                 /Resources << /Font << {fonts}>> >>\n   \
                 /MediaBox [ 0 0 {width} {height} ]\n   \
                 /Contents {c_oid} 0 R\n\
                 >>",
                parent = PAGES_OBJECT_ID,
                fonts = font_oids,
                width = width,
                height = height,
                c_oid = content_oid
            ).map(|_| page_oid)
        })
    }

    fn write_new_object<F, T>(&mut self, write_content: F) -> Result<T>
    where
        F: FnOnce(usize, &mut Pdf) -> Result<T>
    {
        let id = self.object_offsets.len();
        let (result, offset) =
            self.write_object(id, |pdf| write_content(id, pdf))?;
        self.object_offsets.push(offset);
        Ok(result)
    }

    fn write_object_with_id<F, T>(
        &mut self,
        id: usize,
        write_content: F
    ) -> Result<T>
    where
        F: FnOnce(&mut Pdf) -> Result<T>
    {
        assert!(self.object_offsets[id] == -1);
        let (result, offset) = self.write_object(id, write_content)?;
        self.object_offsets[id] = offset;
        Ok(result)
    }

    fn write_object<F, T>(
        &mut self,
        id: usize,
        write_content: F
    ) -> Result<(T, i64)>
    where
        F: FnOnce(&mut Pdf) -> Result<T>
    {
        // `as i64` here would overflow for PDF files bigger than 2^63 bytes
        let offset = self.tell()? as i64;
        writeln!(self.output, "{} 0 obj", id)?;
        let result = write_content(self)?;
        writeln!(self.output, "endobj")?;
        Ok((result, offset))
    }

    /// Write out the document trailer. The trailer consists of the pages
    /// object, the root object, the xref list, the trailer object and the
    /// startxref position.
    pub fn finish(mut self) -> Result<()> {
        self.write_object_with_id(PAGES_OBJECT_ID, |pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Pages\n   \
                 /Count {c}\n   \
                 /Kids [ {pages}]\n\
                 >>",
                c = pdf.page_objects_ids.len(),
                pages = pdf
                    .page_objects_ids
                    .iter()
                    .map(|id| format!("{} 0 R ", id))
                    .collect::<String>()
            )
        })?;
        let document_info_id = if !self.document_info.is_empty() {
            let info = mem::replace(&mut self.document_info, BTreeMap::new());
            self.write_new_object(|page_object_id, pdf| {
                write!(pdf.output, "<<")?;
                for (key, value) in info {
                    writeln!(pdf.output, " /{} ({})", key, value)?;
                }
                if let Ok(now) = time::strftime("%Y%m%d%H%M%S%z", &time::now())
                {
                    write!(
                        pdf.output,
                        " /CreationDate (D:{now})\n \
                         /ModDate (D:{now})",
                        now = now
                    )?;
                }
                writeln!(pdf.output, ">>")?;
                Ok(Some(page_object_id))
            })?
        } else {
            None
        };

        let outlines_id = self.write_outlines()?;

        self.write_object_with_id(ROOT_OBJECT_ID, |pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Catalog\n   \
                 /Pages {} 0 R",
                PAGES_OBJECT_ID
            )?;
            if let Some(outlines_id) = outlines_id {
                writeln!(pdf.output, "/Outlines {} 0 R", outlines_id)?;
            }
            writeln!(pdf.output, ">>")?;
            Ok(())
        })?;
        let startxref = self.tell()?;
        writeln!(
            self.output,
            "xref\n\
             0 {}\n\
             0000000000 65535 f ",
            self.object_offsets.len()
        )?;
        // Object 0 (above) is special
        // Use [1..] to skip object 0 in self.object_offsets.
        for &offset in &self.object_offsets[1..] {
            assert!(offset >= 0);
            writeln!(self.output, "{:010} 00000 n ", offset)?;
        }
        writeln!(
            self.output,
            "trailer\n\
             << /Size {size}\n   \
             /Root {root} 0 R",
            size = self.object_offsets.len(),
            root = ROOT_OBJECT_ID
        )?;
        if let Some(id) = document_info_id {
            writeln!(self.output, "   /Info {} 0 R", id)?;
        }
        writeln!(
            self.output,
            ">>\n\
             startxref\n\
             {}\n\
             %%EOF",
            startxref
        )?;
        Ok(())
    }

    fn write_outlines(&mut self) -> Result<Option<usize>> {
        if self.outline_items.is_empty() {
            return Ok(None);
        }

        let parent_id = self.object_offsets.len();
        self.object_offsets.push(-1);
        let count = self.outline_items.len();
        let mut first_id = 0;
        let mut last_id = 0;
        let items = mem::replace(&mut self.outline_items, Vec::new());
        for (i, item) in items.iter().enumerate() {
            let (is_first, is_last) = (i == 0, i == count - 1);
            let id = self.write_new_object(|object_id, pdf| {
                item.write_dictionary(
                    &mut pdf.output,
                    parent_id,
                    if is_first { None } else { Some(object_id - 1) },
                    if is_last { None } else { Some(object_id + 1) }
                ).and(Ok(object_id))
            })?;
            if is_first {
                first_id = id;
            }
            if is_last {
                last_id = id;
            }
        }
        self.write_object_with_id(parent_id, |pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Outlines\n   \
                 /First {first} 0 R\n   \
                 /Last {last} 0 R\n   \
                 /Count {count}\n\
                 >>",
                last = last_id,
                first = first_id,
                count = count
            )
        })?;
        Ok(Some(parent_id))
    }
}

struct NamedRefs {
    oids: HashMap<FontRef, usize>
}

impl NamedRefs {
    fn new() -> Self {
        NamedRefs {
            oids: HashMap::new()
        }
    }

    fn insert(&mut self, name: FontRef, oid: usize) -> Option<usize> {
        self.oids.insert(name, oid)
    }
}

impl fmt::Display for NamedRefs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, id) in &self.oids {
            write!(f, "{} {} 0 R ", name, id)?;
        }
        Ok(())
    }
}
