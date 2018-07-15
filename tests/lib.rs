extern crate simple_pdf;

use simple_pdf::{BuiltinFont, FontSource};
use simple_pdf::units::Pt;

fn is_close(a: f32, b: f32) -> bool {
    let comparision = (a - b).abs() / (a.abs() + b.abs());
    println!("Checking if {} and {} is close: {}", a, b, comparision);
    comparision < 1e-7
}

#[test]
fn metrics_for_long_string() {
    let font = FontSource::from(BuiltinFont::Times_Roman);
    let size = Pt(12.0);
    let some_text = "This is a test. ";
    let some_len = font.text_width(size, some_text);
    let n = 500;
    assert_eq!(Pt(66.336), some_len);
    let mut long_text = String::new();
    for _ in 0..n {
        long_text = long_text + some_text;
    }
    assert!(is_close(
        (Pt(n as f32) * some_len).0,
        font.text_width(size, &long_text).0
    ));
}
