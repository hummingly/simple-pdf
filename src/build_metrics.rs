#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::Path;

#[allow(dead_code)]
mod encoding;
use encoding::{
    get_base_enc, Encoding, SYMBOL_ENCODING, ZAPFDINGBATS_ENCODING,
};

fn write_cond(
    f: &mut BufWriter<File>,
    name: &str,
    encoding: &Encoding,
) -> Result<()> {
    write!(
        f,
        "  static ref METRICS_{name}: FontMetrics = \
         FontMetrics::from_slice(&[",
        name = name.to_uppercase()
    )?;
    let filename = format!("data/{}.afm", name.replace("_", "-"));
    println!("cargo:rerun-if-changed={}", filename);
    let afm_file = File::open(filename)?;
    for lineresult in BufReader::new(afm_file).lines() {
        let line = lineresult?;
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "C" && words[3] == "WX" && words[6] == "N" {
            if let (Some(c), Ok(w)) =
                (encoding.get_code(words[7]), words[4].parse::<u16>())
            {
                write!(f, "({}, {}), ", c, w)?;
            }
        }
    }
    writeln!(f, "]);")
}

fn main() -> Result<()> {
    let dst = Path::new(
        &env::var("OUT_DIR").expect("Could not find directory."),
    ).join("metrics_data.rs");
    let f = File::create(&dst).expect("Could not create file.");
    let f = &mut BufWriter::new(f);
    let textfonts = [
        "Courier",
        "Courier_Bold",
        "Courier_Oblique",
        "Courier_BoldOblique",
        "Helvetica",
        "Helvetica_Bold",
        "Helvetica_Oblique",
        "Helvetica_BoldOblique",
        "Times_Roman",
        "Times_Bold",
        "Times_Italic",
        "Times_BoldItalic",
        "Symbol",
        "ZapfDingbats",
    ];
    writeln!(
        f,
        "pub fn get_builtin_metrics(font: BuiltinFont) \
         -> &'static FontMetrics {{\n\
         match font {{"
    )?;
    for font in textfonts.iter() {
        writeln!(
            f,
            "BuiltinFont::{} => &METRICS_{},",
            font,
            font.to_uppercase()
        )?;
    }
    writeln!(
        f,
        "}}\n\
         }}\n\
         lazy_static! {{"
    )?;

    let encoding = get_base_enc();

    for font in textfonts.iter().take(12) {
        write_cond(f, font, encoding.to_encoding())?;
    }

    write_cond(f, "Symbol", &SYMBOL_ENCODING)?;
    write_cond(f, "ZapfDingbats", &ZAPFDINGBATS_ENCODING)?;
    writeln!(f, "}}")
}
