#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;

#[allow(dead_code)]
mod encoding;
use encoding::{
    Encoding, MAC_ROMAN_ENCODING, SYMBOL_ENCODING, WIN_ANSI_ENCODING,
    ZAPFDINGBATS_ENCODING,
};

fn write_cond(f: &mut File, name: &str, encoding: &Encoding) -> Result<()> {
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
    writeln!(f, "]);")?;
    Ok(())
}

fn main() {
    let dst = Path::new(
        &env::var("OUT_DIR").expect("Could not find directory.")
    ).join("metrics_data.rs");
    let f = &mut File::create(&dst).expect("Could not create file.");
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
    ).expect("Could not write to file.");
    for font in textfonts.iter() {
        writeln!(
            f,
            "BuiltinFont::{} => &METRICS_{},",
            font,
            font.to_uppercase()
        ).expect("Could not write to file.");
    }
    writeln!(
        f,
        "}}\n\
         }}\n\
         lazy_static! {{"
    ).expect("Could not write to file.");

    for font in textfonts.iter().take(12) {
        if cfg!(target_os = "macos") {
            write_cond(f, font, &MAC_ROMAN_ENCODING)
                .expect("Could not write to file.");
        } else {
            write_cond(f, font, &WIN_ANSI_ENCODING)
                .expect("Could not write to file.");
        };
    }

    write_cond(f, "Symbol", &SYMBOL_ENCODING)
        .expect("Could not write to file.");
    write_cond(f, "ZapfDingbats", &ZAPFDINGBATS_ENCODING)
        .expect("Could not write to file.");
    writeln!(f, "}}").expect("Could not write to file.");
}
