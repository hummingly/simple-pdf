use std::collections::BTreeMap;

/// A FontEncoding containing the base and actual font encoding to represent a
/// font properly in a PDF.
///
/// The base encoding is plattform specific and the encoding is specified by
/// the font.
/// Currently, only WinAnsiEncoding and MacRomanEncoding is supported.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontEncoding {
    base: BaseEncoding,
    encoding: Encoding,
}

impl Default for FontEncoding {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn get_base_enc() -> BaseEncoding {
    if cfg!(target_os = "macos") {
        BaseEncoding::MacRomanEncoding
    } else {
        BaseEncoding::WinAnsiEncoding
    }
}

impl FontEncoding {
    /// Creates a new FontEncoding with the plattform encoding (WinAnsiEncoding
    /// or MacRomanEncoding) as default value.
    pub fn new() -> Self {
        let enc = get_base_enc();
        FontEncoding {
            base: enc,
            encoding: enc.to_encoding().clone(),
        }
    }

    /// Creates a new FontEncoding with the font's encoding.
    pub fn with_encoding(encoding: Encoding) -> Self {
        FontEncoding {
            base: get_base_enc(),
            encoding,
        }
    }

    /// Returns the encoding of each character for a font.
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    /// Returns the name of the encoding.
    pub fn name(&self) -> String {
        self.encoding.name()
    }

    /// Returns the base encoding, not to be confused with encoding. The
    /// values are WinAnsiEncoding, MacRomanEncoding and
    /// MacExpertEncoding.
    pub fn base_name(&self) -> String {
        self.base.name()
    }

    // /// Sets the base encoding to MacExpertEncoding. Only supported fonts
    // /// should use this.
    // pub fn set_expert_encoding(&self) {
    //     self.base = MacExpertEncoding;
    // }

    /// Convert a String to a vector of bytes in the encoding. For examples,
    /// see the Encoding documentation.
    pub fn encode_string(&self, text: &str) -> Vec<u8> {
        self.encoding.encode_string(text)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum BaseEncoding {
    WinAnsiEncoding,
    MacRomanEncoding, // MacExpertEncoding
}

impl BaseEncoding {
    pub fn to_encoding(self) -> &'static Encoding {
        match self {
            BaseEncoding::WinAnsiEncoding => &WIN_ANSI_ENCODING,
            BaseEncoding::MacRomanEncoding => &MAC_ROMAN_ENCODING
            // MacExpertEncoding => &MAC_EXPERT_ENCODING,
        }
    }

    pub fn name(self) -> String {
        match self {
            BaseEncoding::WinAnsiEncoding => "WinAnsiEncoding".to_string(),
            BaseEncoding::MacRomanEncoding => "MacRomanEncoding".to_string()
            // MacExpertEncoding => "MacExpertEncoding".to_string(),
        }
    }
}

/// Represent a text encoding used in PDF. An encoding maintains the
/// connection between unicode code points, bytes in PDF strings, and glyph
/// names.
///
/// Currently, only WIN_ANSI_ENCODING and MAC_ROMAN_ENCODING is supported.
///
/// # Example
/// ````
/// use simple_pdf::{BuiltinFont, FontSource};
/// assert_eq!("WinAnsiEncoding", BuiltinFont::Helvetica.encoding().name());
/// assert_eq!("SymbolEncoding", BuiltinFont::Symbol.encoding().name());
/// ````
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Encoding {
    name: String,
    name_to_code: BTreeMap<&'static str, u8>,
    unicode_to_code: BTreeMap<char, u8>,
}

impl Encoding {
    /// Creates a new encoding from the code and unicode maps.
    pub fn new(
        name: String,
        code: BTreeMap<&'static str, u8>,
        unicode: BTreeMap<char, u8>,
    ) -> Self {
        Encoding {
            name,
            name_to_code: code,
            unicode_to_code: unicode,
        }
    }

    /// The name of the encoding, as used in the font object.
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Get the encoded code point from a type1 character name. Character
    /// names are case sensitive and contain only ascii letters. If the name
    /// is not available in the encoding, or is not a proper character name,
    /// None is returned.
    ///
    /// # Example
    /// ````
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// let enc = BuiltinFont::Helvetica.encoding();
    /// assert_eq!(Some(32), enc.get_code("space"));
    /// assert_eq!(Some(65), enc.get_code("A"));
    /// assert_eq!(Some(229), enc.get_code("aring"));
    /// assert_eq!(None, enc.get_code("Lslash"));
    /// assert_eq!(None, enc.get_code(""));
    /// assert_eq!(None, enc.get_code("☺"));
    /// ````
    pub fn get_code(&self, name: &str) -> Option<u8> {
        self.name_to_code.get(name).cloned()
    }

    /// Get the encoded code point from a (unicode) character.
    /// If the character is not available in the encoding, None is returned.
    ///
    /// # Example
    /// ````
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// let enc = BuiltinFont::Helvetica.encoding();
    /// assert_eq!(Some(b' '), enc.encode_char(' '));
    /// assert_eq!(Some(b'A'), enc.encode_char('A'));
    /// assert_eq!(Some(b'\\'), enc.encode_char('\\'));
    /// assert_eq!(Some(229), enc.encode_char('å'));
    /// assert_eq!(None, enc.encode_char('Ł'));
    /// assert_eq!(None, enc.encode_char(char::from(0)));
    /// assert_eq!(None, enc.encode_char('☺'));
    /// ````
    pub fn encode_char(&self, ch: char) -> Option<u8> {
        self.unicode_to_code.get(&ch).cloned()
    }

    /// Convert a rust string to a vector of bytes in the encoding.
    /// # Example
    /// ````
    /// use simple_pdf::{BuiltinFont, FontSource};
    /// let enc = BuiltinFont::Helvetica.encoding();
    /// let symb_enc = BuiltinFont::Symbol.encoding();
    /// assert_eq!(
    ///     vec![92, 92, 92, 40, 65, 66, 67, 92, 41, 92, 92],
    ///     enc.encode_string("\\(ABC)\\")
    /// );
    /// assert_eq!(
    ///     vec![82, 228, 107, 115, 109, 246, 114, 103, 229, 115],
    ///     enc.encode_string("Räksmörgås")
    /// );
    /// assert_eq!(
    ///     vec![67, 111, 102, 102, 101, 101, 32, 128, 49, 46, 50, 48],
    ///     enc.encode_string("Coffee €1.20")
    /// );
    /// assert_eq!(
    ///     vec![97, 32, 206, 32, 194],
    ///     symb_enc.encode_string("α ∈ ℜ")
    /// );
    /// ````
    pub fn encode_string(&self, text: &str) -> Vec<u8> {
        let mut result = Vec::with_capacity(text.len());
        for ch in text.chars() {
            match self.encode_char(ch) {
                Some(ch) => {
                    if ch == b'\\' || ch == b'(' || ch == b')' {
                        result.push(b'\\');
                    }
                    result.push(ch);
                }
                None => result.push(b'?'),
            }
        }
        result.shrink_to_fit();
        result
    }

    fn init_block(&mut self, start: u8, data: &[&'static str]) {
        for (i, name) in data.iter().enumerate() {
            self.name_to_code.insert(name, start + (i as u8));
        }
    }
}

lazy_static! {
    pub static ref WIN_ANSI_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        // /WinAnsiEncoding is kind of close to first byte of unicode
        // Except for the 16 chars that are reserved in 8859-1 and
        // used in Windows-1252.
        for code in 1..=255 {
            codes.insert(code as char, code);
        }
        codes.insert('€', 128);
        codes.insert('‚', 130);
        codes.insert('ƒ', 131);
        codes.insert('„', 132);
        codes.insert('…', 133);
        codes.insert('†', 134);
        codes.insert('‡', 135);
        codes.insert('ˆ', 136);
        codes.insert('‰', 137);
        codes.insert('Š', 138);
        codes.insert('‹', 139);
        codes.insert('Œ', 140);
        codes.insert('Ž', 142);
        codes.insert('‘', 145);
        codes.insert('’', 146);
        codes.insert('“', 147);
        codes.insert('”', 148);
        codes.insert('•', 149);
        codes.insert('–', 150);
        codes.insert('—', 151);
        codes.insert('˜', 152);
        codes.insert('™', 153);
        codes.insert('š', 154);
        codes.insert('›', 155);
        codes.insert('ž', 158);
        codes.insert('Ÿ', 159);
        let mut result = Encoding::new("WinAnsiEncoding".to_string(), BTreeMap::new(), codes);
        result.init_block(0o40, &["space", "exclam", "quotedbl", "numbersign","dollar", "percent", "ampersand", "quotesingle"]);
        result.init_block(0o50, &[
            "parenleft", "parenright", "asterisk", "plus",
            "comma", "hyphen", "period", "slash"]);
        result.init_block(0o60, &[
            "zero", "one", "two", "three", "four", "five", "six", "seven"]);
        result.init_block(0o70, &[
            "eight", "nine", "colon", "semicolon",
            "less", "equal", "greater", "question"]);
        result.init_block(0o100, &[
            "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
            "W", "X", "Y", "Z"]);
        result.init_block(0o133, &[
            "bracketleft",
            "backslash", "bracketright", "asciicircum", "underscore"]);
        result.init_block(0o140, &[
            "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
            "w", "x", "y", "z"]);
        result.init_block(0o173, &[
            "braceleft", "bar", "braceright", "asciitilde"]);
        result.init_block(0o200, &[
            "Euro", "..1", "quotesinglbase", "florin",
            "quotedblbase", "ellipsis", "dagger", "daggerdbl"]);
        result.init_block(0o210, &[
            "circumflex", "perthousand", "Scaron", "guilsinglleft",
            "OE", "..5", "Zcaron", "..7"]);
        result.init_block(0o220, &[
            "..0", "quoteleft", "quoteright", "quotedblleft",
            "quotedblright", "bullet", "endash", "emdash"]);
        result.init_block(0o230, &[
            "tilde", "trademark", "scaron", "guilsinglright",
            "oe", "..5", "zcaron", "Ydieresis"]);
        result.init_block(0o240, &[
            "..0", "exclamdown", "cent", "sterling",
            "currency", "yen", "brokenbar", "section"]);
        result.init_block(0o250, &[
            "dieresis", "copyright", "ordfeminine", "guillemotleft",
            "logicalnot", "..5", "registered", "macron"]);
        result.init_block(0o260, &[
            "degree", "plusminus", "twosuperior", "threesuperior",
            "acute", "mu", "paragraph", "periodcentered"]);
        result.init_block(0o270, &[
            "cedilla", "onesuperior", "ordmasculine", "guillemotright",
            "onequarter", "onehalf", "threequarters", "questiondown"]);
        result.init_block(0o300, &[
            "Agrave", "Aacute", "Acircumflex", "Atilde",
            "Adieresis", "Aring", "AE", "Ccedilla"]);
        result.init_block(0o310, &[
            "Egrave", "Eacute", "Ecircumflex", "Edieresis",
            "Igrave", "Iacute", "Icircumflex", "Idieresis"]);
        result.init_block(0o320, &[
            "Eth", "Ntilde", "Ograve", "Oacute",
            "Ocircumflex", "Otilde", "Odieresis", "multiply"]);
        result.init_block(0o330, &[
            "Oslash", "Ugrave", "Uacute", "Ucircumflex",
            "Udieresis", "Yacute", "Thorn", "germandbls"]);
        result.init_block(0o340, &[
            "agrave", "aacute", "acircumflex", "atilde",
            "adieresis", "aring", "ae", "ccedilla"]);
        result.init_block(0o350, &[
            "egrave", "eacute", "ecircumflex", "edieresis",
            "igrave", "iacute", "icircumflex", "idieresis"]);
        result.init_block(0o360, &[
            "eth", "ntilde", "ograve", "oacute",
            "ocircumflex", "otilde", "odieresis", "divide"]);
        result.init_block(0o370, &[
            "oslash", "ugrave", "uacute", "ucircumflex",
            "udieresis", "yacute", "thorn", "ydieresis"]);
        result
    };

    pub static ref MAC_ROMAN_ENCODING: Encoding = {
        // the encoding is closer to the StandardEncoding
        let mut codes = BTreeMap::new();
        let mut names = BTreeMap::new();
        for code in 1..255 {
            codes.insert(code as char, code);
        }
        {
            let mut enc = |ch: char, name: &'static str, code: u8| {
                codes.insert(ch, code);
                names.insert(name, code);
            };
            enc('Æ', "AE", 0o256);
            enc('Á', "Aacute", 0o347);
            enc('Â', "Acircumflex", 0o345);
            enc('Ä', "Adieresis", 0o200);
            enc('À', "Agrave", 0o313);
            enc('Å', "Aring", 0o201);
            enc('Ã', "Atilde", 0o314);
            enc('Ç', "Ccedilla", 0o202);
            enc('É', "Eacute", 0o203);
            enc('Ê', "Ecircumflex", 0o346);
            enc('Ë', "Edieresis", 0o350);
            enc('È', "Egrave", 0o351);
            enc('€', "Euro", 0o333);
            enc('Í', "Iacute", 0o352);
            enc('Î', "Icircumflex", 0o353);
            enc('Ï', "Idieresis", 0o354);
            enc('Ì', "Igrave", 0o355);
            enc('Ñ', "Ntilde", 0o204);
            enc('Œ', "OE", 0o316);
            enc('Ó', "Oacute", 0o356);
            enc('Ô', "Ocircumflex", 0o357);
            enc('Ö', "Odieresis", 0o205);
            enc('Ò', "Ograve", 0o361);
            enc('Ø', "Oslash", 0o257);
            enc('Õ', "Otilde", 0o315);
            enc('Ú', "Uacute", 0o362);
            enc('Û', "Ucircumflex", 0o363);
            enc('Ü', "Udieresis", 0o206);
            enc('Ù', "Ugrave", 0o364);
            enc('Ÿ', "Ydieresis", 0o331);
            enc('á', "aacute", 0o207);
            enc('â', "acircumflex", 0o211);
            enc('´', "acute", 0o253);
            enc('ä', "adieresis", 0o212);
            enc('æ', "ae", 0o276);
            enc('à', "agrave", 0o210);
            enc('å', "aring", 0o214);
            enc('ã', "atilde", 0o213);
            enc('˘', "breve", 0o371);
            enc('•', "bullet", 0o245);
            enc('ˇ', "caron", 0o377);
            enc('ç', "ccedilla", 0o215);
            enc('¸', "cedilla", 0o374);
            enc('ˆ', "circumflex", 0o366);
            enc('©', "copyright", 0o251);
            enc('†', "dagger", 0o240);
            enc('‡', "daggerdbl", 0o340);
            enc('°', "degree", 0o241);
            enc('¨', "dieresis", 0o254);
            enc('÷', "divide", 0o326);
            enc('˙', "dotaccent", 0o307);
            enc('é', "eacute", 0o216);
            enc('ê', "ecircumflex", 0o220);
            enc('ë', "edieresis", 0o221);
            enc('è', "egrave", 0o217);
            enc('…', "ellipsis", 0o311);
            enc('—', "emdash", 0o321);
            enc('–', "endash", 0o320);
            enc('¡', "exclamdown", 0o301);
            enc('ﬁ', "fi", 0o336);
            enc('ﬂ', "fl", 0o337);
            enc('ƒ', "florin", 0o304);
            enc('⁄', "fraction", 0o332);
            enc('∕', "fraction", 0o332);
            enc('ß', "germandbls", 0o247);
            enc('`', "grave", 0o140);
            enc('«', "guillemotleft", 0o307);
            enc('»', "guillemotright", 0o310);
            enc('‹', "guilsinglleft", 0o334);
            enc('›', "guilsinglright", 0o335);
            enc('˝', "hungarumlaut", 0o375);
            enc('í', "iacute", 0o222);
            enc('î', "icircumflex", 0o224);
            enc('ï', "idieresis", 0o225);
            enc('ì', "igrave", 0o223);
            enc('¬', "logicalnot", 0o302);
            enc('¯', "macron", 0o370);
            enc('ˉ', "macron", 0o370);
            enc('µ', "mu", 0o265);
            enc('ñ', "ntilde", 0o226);
            enc('ó', "oacute", 0o227);
            enc('ô', "ocircumflex", 0o231);
            enc('ö', "odieresis", 0o232);
            enc('œ', "oe", 0o317);
            enc('˛', "ogonek", 0o376);
            enc('ò', "ograve", 0o230);
            enc('ª', "ordfeminine", 0o273);
            enc('º', "ordmasculine", 0o274);
            enc('ø', "oslash", 0o277);
            enc('õ', "otilde", 0o233);
            enc('·', "periodcentered", 0o341);
            enc('∙', "periodcentered", 0o341);
            enc('‰', "perthousand", 0o344);
            enc('±', "plusminus", 0o261);
            enc('¿', "questiondown", 0o300);
            enc('„', "quotedblbase", 0o343);
            enc('“', "quotedblleft", 0o322);
            enc('”', "quotedblright", 0o323);
            enc('‘', "quoteleft", 0o324);
            enc('’', "quoteright", 0o325);
            enc('‚', "quotesinglebase", 0o342);
            enc('\'', "quotesingle", 0o047);
            enc('®', "registered", 0o250);
            enc('˚', "ring", 0o373);
            enc('§', "section", 0o244);
            enc('˜', "tilde", 0o367);
            enc('™', "trademark", 0o252);
            enc('ú', "uacute", 0o234);
            enc('û', "ucircumflex", 0o236);
            enc('ü', "udieresis", 0o237);
            enc('ù', "ugrave", 0o235);
            enc('ÿ', "ydieres", 0o330);
            enc('¥', "yen", 0o264);
        }
        Encoding::new("MacRomanEncoding".to_string(), names, codes)
    };

    // http://ftp.unicode.org/Public/MAPPINGS/VENDORS/ADOBE/symbol.txt
    pub static ref SYMBOL_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        let mut names = BTreeMap::new();
        for code in 1..255 {
            codes.insert(code as char, code);
        }
        {
            let mut enc = |ch: char, name: &'static str, code: u8| {
                codes.insert(ch, code);
                names.insert(name, code);
            };
            enc(' ', "space", 0o40);
            enc(' ', "space", 0o40);
            enc('!', "exclam", 0o41);
            enc('∀', "universal", 0o42);
            enc('#', "numbersign", 0o43);
            enc('∃', "existential", 0o44);
            enc('%', "percent", 0o45);
            enc('&', "ampersand", 0o46);
            enc('∋', "suchthat", 0o47);
            enc('(', "parenleft", 0o50);
            enc(')', "parenright", 0o51);
            enc('∗', "asteriskmath", 0o52);
            enc('+', "plus", 0o53);
            enc(',', "comma", 0o54);
            enc('−', "minus", 0o55);
            enc('.', "period", 0o56);
            enc('/', "slash", 0o57);
            enc('0', "zero", 0o60);
            enc('1', "one", 0o61);
            enc('2', "two", 0o62);
            enc('3', "three", 0o63);
            enc('4', "four", 0o64);
            enc('5', "five", 0o65);
            enc('6', "six", 0o66);
            enc('7', "seven", 0o67);
            enc('8', "eight", 0o70);
            enc('9', "nine", 0o71);
            enc(':', "colon", 0o72);
            enc(';', "semicolon", 0o73);
            enc('<', "less", 0o74);
            enc('=', "equal", 0o75);
            enc('>', "greater", 0o76);
            enc('?', "question", 0o77);
            enc('≅', "congruent", 0o100);
            enc('Α', "Alpha", 0o101);
            enc('Β', "Beta", 0o102);
            enc('Χ', "Chi", 0o103);
            enc('Δ', "Delta", 0o104);
            enc('∆', "Delta", 0o104);
            enc('Ε', "Epsilon", 0o105);
            enc('Φ', "Phi", 0o106);
            enc('Γ', "Gamma", 0o107);
            enc('Η', "Eta", 0o110);
            enc('Ι', "Iota", 0o111);
            enc('ϑ', "theta1", 0o112);
            enc('Κ', "Kappa", 0o113);
            enc('Λ', "Lambda", 0o114);
            enc('Μ', "Mu", 0o115);
            enc('Ν', "Nu", 0o116);
            enc('Ο', "Omicron", 0o117);
            enc('Π', "Pi", 0o120);
            enc('Θ', "Theta", 0o121);
            enc('Ρ', "Rho", 0o122);
            enc('Σ', "Sigma", 0o123);
            enc('Τ', "Tau", 0o124);
            enc('Υ', "Upsilon", 0o125);
            enc('ς', "sigma1", 0o126);
            enc('Ω', "Omega", 0o127);
            enc('Ω', "Omega", 0o127);
            enc('Ξ', "Xi", 0o130);
            enc('Ψ', "Psi", 0o131);
            enc('Ζ', "Zeta", 0o132);
            enc('[', "bracketleft", 0o133);
            enc('∴', "therefore", 0o134);
            enc(']', "bracketright", 0o135);
            enc('⊥', "perpendicular", 0o136);
            enc('_', "underscore", 0o137);
            enc('\u{0305}', "radicalex", 0o140); // Note: (CUS) cannot be displayed
            enc('α', "alpha", 0o141);
            enc('β', "beta", 0o142);
            enc('χ', "chi", 0o143);
            enc('δ', "delta", 0o144);
            enc('ε', "epsilon", 0o145);
            enc('φ', "phi", 0o146);
            enc('γ', "gamma", 0o147);
            enc('η', "eta", 0o150);
            enc('ι', "iota", 0o151);
            enc('ϕ', "phi1", 0o152);
            enc('κ', "kappa", 0o153);
            enc('λ', "lambda", 0o154);
            enc('µ', "mu", 0o155);
            enc('μ', "mu", 0o155);
            enc('ν', "nu", 0o156);
            enc('ο', "omicron", 0o157);
            enc('π', "pi", 0o160);
            enc('θ', "theta", 0o161);
            enc('ρ', "rho", 0o162);
            enc('σ', "sigma", 0o163);
            enc('τ', "tau", 0o164);
            enc('υ', "upsilon", 0o165);
            enc('ϖ', "omega1", 0o166);
            enc('ω', "omega", 0o167);
            enc('ξ', "xi", 0o170);
            enc('ψ', "psi", 0o171);
            enc('ζ', "zeta", 0o172);
            enc('{', "braceleft", 0o173);
            enc('|', "bar", 0o174);
            enc('}', "braceright", 0o175);
            enc('∼', "similar", 0o176);
            enc('€', "Euro", 0o240);
            enc('ϒ', "Upsilon1", 0o241);
            enc('′', "minute", 0o242);
            enc('≤', "lessequal", 0o243);
            enc('⁄', "fraction", 0o244);
            enc('∕', "fraction", 0o244);
            enc('∞', "infinity", 0o245);
            enc('ƒ', "florin", 0o246);
            enc('♣', "club", 0o247);
            enc('♦', "diamond", 0o250);
            enc('♥', "heart", 0o251);
            enc('♠', "spade", 0o252);
            enc('↔', "arrowboth", 0o253);
            enc('←', "arrowleft", 0o254);
            enc('↑', "arrowup", 0o255);
            enc('→', "arrowright", 0o256);
            enc('↓', "arrowdown", 0o257);
            enc('°', "degree", 0o260);
            enc('±', "plusminus", 0o261);
            enc('″', "second", 0o262);
            enc('≥', "greaterequal", 0o263);
            enc('×', "multiply", 0o264);
            enc('∝', "proportional", 0o265);
            enc('∂', "partialdiff", 0o266);
            enc('•', "bullet", 0o267);
            enc('÷', "divide", 0o270);
            enc('≠', "notequal", 0o271);
            enc('≡', "equivalence", 0o272);
            enc('≈', "approxequal", 0o273);
            enc('…', "ellipsis", 0o274);
            enc('⏐', "arrowvertex", 0o275); // Note: (CUS) cannot be displayed
            enc('⎯', "arrowhorizex", 0o276); // Note: (CUS) cannot be displayed
            enc('↵', "carriagereturn", 0o277);
            enc('ℵ', "aleph", 0o300);
            enc('ℑ', "Ifraktur", 0o301);
            enc('ℜ', "Rfraktur", 0o302);
            enc('℘', "weierstrass", 0o303);
            enc('⊗', "circlemultiply", 0o304);
            enc('⊕', "circleplus", 0o305);
            enc('∅', "emptyset", 0o306);
            enc('∩', "intersection", 0o307);
            enc('∪', "union", 0o310);
            enc('⊃', "propersuperset", 0o311);
            enc('⊇', "reflexsuperset", 0o312);
            enc('⊄', "notsubset", 0o313);
            enc('⊂', "propersubset", 0o314);
            enc('⊆', "reflexsubset", 0o315);
            enc('∈', "element", 0o316);
            enc('∉', "notelement", 0o317);
            enc('∠', "angle", 0o320);
            enc('∇', "gradient", 0o321);
            enc('®', "registerserif", 0o322); // Note: (CUS) cannot be displayed
            enc('©', "copyrightserif", 0o323); // Note: (CUS) cannot be displayed
            enc('™', "trademarkserif", 0o324); // Note: (CUS) cannot be displayed
            enc('∏', "product", 0o325);
            enc('√', "radical", 0o326);
            enc('⋅', "dotmath", 0o327);
            enc('¬', "logicalnot", 0o330);
            enc('∧', "logicaland", 0o331);
            enc('∨', "logicalor", 0o332);
            enc('⇔', "arrowdblboth", 0o333);
            enc('⇐', "arrowdblleft", 0o334);
            enc('⇑', "arrowdblup", 0o335);
            enc('⇒', "arrowdblright", 0o336);
            enc('⇓', "arrowdbldown", 0o337);
            enc('◊', "lozenge", 0o340);
            enc('〈', "angleleft", 0o341);
            enc('®', "registersans", 0o342); // Note: (CUS) cannot be displayed
            enc('©', "copyrightsans", 0o343); // Note: (CUS) cannot be displayed
            enc('™', "trademarksans", 0o344); // Note: (CUS) cannot be displayed
            enc('∑', "summation", 0o345);
            enc('⎛', "parenlefttp", 0o346); // Note: (CUS) cannot be displayed
            enc('⎜', "parenleftex", 0o347); // Note: (CUS) cannot be displayed
            enc('⎝', "parenleftbt", 0o350); // Note: (CUS) cannot be displayed
            enc('⎡', "bracketlefttp", 0o351); // Note: (CUS) cannot be displayed
            enc('⎢', "bracketleftex", 0o352); // Note: (CUS) cannot be displayed
            enc('⎣', "bracketleftbt", 0o353); // Note: (CUS) cannot be displayed
            enc('⎧', "bracelefttp", 0o354); // Note: (CUS) cannot be displayed
            enc('⎨', "braceleftmid", 0o355); // Note: (CUS) cannot be displayed
            enc('⎩', "braceleftbt", 0o356); // Note: (CUS) cannot be displayed
            enc('⎪', "braceex", 0o357); // Note: (CUS) cannot be displayed
            enc('〉', "angleright", 0o361);
            enc('∫', "integral", 0o362);
            enc('⌠', "integraltp", 0o363);
            enc('⎮', "integralex", 0o364); // Note: (CUS) cannot be displayed
            enc('⌡', "integralbt", 0o365);
            enc('⎞', "parenrighttp", 0o366); // Note: (CUS) cannot be displayed
            enc('⎟', "parenrightex", 0o367); // Note: (CUS) cannot be displayed
            enc('⎠', "parenrightbt", 0o370); // Note: (CUS) cannot be displayed
            enc('⎤', "bracketrighttp", 0o371); // Note: (CUS) cannot be displayed
            enc('⎥', "bracketrightex", 0o372); // Note: (CUS) cannot be displayed
            enc('⎦', "bracketrightbt", 0o373); // Note: (CUS) cannot be displayed
            enc('⎫', "bracerighttp", 0o374); // Note: (CUS) cannot be displayed
            enc('⎬', "bracerightmid", 0o375); // Note: (CUS) cannot be displayed
            enc('⎭', "bracerightbt", 0o376); // Note: (CUS) cannot be displayed
        }
        Encoding::new("SymbolEncoding".to_string(), names, codes)
    };

    // https://unicode.org/Public/MAPPINGS/VENDORS/ADOBE/zdingbat.txt
    pub static ref ZAPFDINGBATS_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        let mut names = BTreeMap::new();
        for code in 1..255 {
            codes.insert(code as char, code);
        }
        {
            let mut enc = |ch: char, name: &'static str, code: u8| {
                codes.insert(ch, code);
                names.insert(name, code);
            };
            enc(' ', "space", 0o40);
            enc(' ', "space", 0o40);
            enc('✁', "a1", 0o41);
            enc('✂', "a2", 0o42);
            enc('✃', "a202", 0o43);
            enc('✄', "a3", 0o44);
            enc('☎', "a4", 0o45);
            enc('✆', "a5", 0o46);
            enc('✇', "a119", 0o47);
            enc('✈', "a118", 0o50);
            enc('✉', "a117", 0o51);
            enc('☛', "a11", 0o52);
            enc('☞', "a12", 0o53);
            enc('✌', "a13", 0o54);
            enc('✍', "a14", 0o55);
            enc('✎', "a15", 0o56);
            enc('✏', "a16", 0o57);
            enc('✐', "a105", 0o60);
            enc('✑', "a17", 0o61);
            enc('✒', "a18", 0o62);
            enc('✓', "a19", 0o63);
            enc('✔', "a20", 0o64);
            enc('✕', "a21", 0o65);
            enc('✖', "a22", 0o66);
            enc('✗', "a23", 0o67);
            enc('✘', "a24", 0o70);
            enc('✙', "a25", 0o71);
            enc('✚', "a26", 0o72);
            enc('✛', "a27", 0o73);
            enc('✜', "a28", 0o74);
            enc('✝', "a6", 0o75);
            enc('✞', "a7", 0o76);
            enc('✟', "a8", 0o77);
            enc('✠', "a9", 0o100);
            enc('✡', "a10", 0o101);
            enc('✢', "a29", 0o102);
            enc('✣', "a30", 0o103);
            enc('✤', "a31", 0o104);
            enc('✥', "a32", 0o105);
            enc('✦', "a33", 0o106);
            enc('✧', "a34", 0o107);
            enc('★', "a35", 0o110);
            enc('✩', "a36", 0o111);
            enc('✪', "a37", 0o112);
            enc('✫', "a38", 0o113);
            enc('✬', "a39", 0o114);
            enc('✭', "a40", 0o115);
            enc('✮', "a41", 0o116);
            enc('✯', "a42", 0o117);
            enc('✰', "a43", 0o120);
            enc('✱', "a44", 0o121);
            enc('✲', "a45", 0o122);
            enc('✳', "a46", 0o123);
            enc('✴', "a47", 0o124);
            enc('✵', "a48", 0o125);
            enc('✶', "a49", 0o126);
            enc('✷', "a50", 0o127);
            enc('✸', "a51", 0o130);
            enc('✹', "a52", 0o131);
            enc('✺', "a53", 0o132);
            enc('✻', "a54", 0o133);
            enc('✼', "a55", 0o134);
            enc('✽', "a56", 0o135);
            enc('✾', "a57", 0o136);
            enc('✿', "a58", 0o137);
            enc('❀', "a59", 0o140);
            enc('❁', "a60", 0o141);
            enc('❂', "a61", 0o142);
            enc('❃', "a62", 0o143);
            enc('❄', "a63", 0o144);
            enc('❅', "a64", 0o145);
            enc('❆', "a65", 0o146);
            enc('❇', "a66", 0o147);
            enc('❈', "a67", 0o150);
            enc('❉', "a68", 0o151);
            enc('❊', "a69", 0o152);
            enc('❋', "a70", 0o153);
            enc('●', "a71", 0o154);
            enc('❍', "a72", 0o155);
            enc('■', "a73", 0o156);
            enc('❏', "a74", 0o157);
            enc('❐', "a203", 0o160);
            enc('❑', "a75", 0o161);
            enc('❒', "a204", 0o162);
            enc('▲', "a76", 0o163);
            enc('▼', "a77", 0o164);
            enc('◆', "a78", 0o165);
            enc('❖', "a79", 0o166);
            enc('◗', "a81", 0o167);
            enc('❘', "a82", 0o170);
            enc('❙', "a83", 0o171);
            enc('❚', "a84", 0o172);
            enc('❛', "a97", 0o173);
            enc('❜', "a98", 0o174);
            enc('❝', "a99", 0o175);
            enc('❞', "a100", 0o176);
            enc('❨', "a89", 0o200); // Note: (CUS) cannot be displayed
            enc('❩', "a90", 0o201); // Note: (CUS) cannot be displayed
            enc('❪', "a93", 0o202); // Note: (CUS) cannot be displayed
            enc('❫', "a94", 0o203); // Note: (CUS) cannot be displayed
            enc('❬', "a91", 0o204); // Note: (CUS) cannot be displayed
            enc('❭', "a92", 0o205); // Note: (CUS) cannot be displayed
            enc('❮', "a205", 0o206); // Note: (CUS) cannot be displayed
            enc('❯', "a85", 0o207); // Note: (CUS) cannot be displayed
            enc('❰', "a206", 0o210); // Note: (CUS) cannot be displayed
            enc('❱', "a86", 0o211); // Note: (CUS) cannot be displayed
            enc('❲', "a87", 0o212); // Note: (CUS) cannot be displayed
            enc('❳', "a88", 0o213); // Note: (CUS) cannot be displayed
            enc('❴', "a95", 0o214); // Note: (CUS) cannot be displayed
            enc('❵', "a96", 0o215); // Note: (CUS) cannot be displayed
            enc('❡', "a101", 0o241);
            enc('❢', "a102", 0o242);
            enc('❣', "a103", 0o243);
            enc('❤', "a104", 0o244);
            enc('❥', "a106", 0o245);
            enc('❦', "a107", 0o246);
            enc('❧', "a108", 0o247);
            enc('♣', "a112", 0o250);
            enc('♦', "a111", 0o251);
            enc('♥', "a110", 0o252);
            enc('♠', "a109", 0o253);
            enc('①', "a120", 0o254);
            enc('②', "a121", 0o255);
            enc('③', "a122", 0o256);
            enc('④', "a123", 0o257);
            enc('⑤', "a124", 0o260);
            enc('⑥', "a125", 0o261);
            enc('⑦', "a126", 0o262);
            enc('⑧', "a127", 0o263);
            enc('⑨', "a128", 0o264);
            enc('⑩', "a129", 0o265);
            enc('❶', "a130", 0o266);
            enc('❷', "a131", 0o267);
            enc('❸', "a132", 0o270);
            enc('❹', "a133", 0o271);
            enc('❺', "a134", 0o272);
            enc('❻', "a135", 0o273);
            enc('❼', "a136", 0o274);
            enc('❽', "a137", 0o275);
            enc('❾', "a138", 0o276);
            enc('❿', "a139", 0o277);
            enc('➀', "a140", 0o300);
            enc('➁', "a141", 0o301);
            enc('➂', "a142", 0o302);
            enc('➃', "a143", 0o303);
            enc('➄', "a144", 0o304);
            enc('➅', "a145", 0o305);
            enc('➆', "a146", 0o306);
            enc('➇', "a147", 0o307);
            enc('➈', "a148", 0o310);
            enc('➉', "a149", 0o311);
            enc('➊', "a150", 0o312);
            enc('➋', "a151", 0o313);
            enc('➌', "a152", 0o314);
            enc('➍', "a153", 0o315);
            enc('➎', "a154", 0o316);
            enc('➏', "a155", 0o317);
            enc('➐', "a156", 0o320);
            enc('➑', "a157", 0o321);
            enc('➒', "a158", 0o322);
            enc('➓', "a159", 0o323);
            enc('➔', "a160", 0o324);
            enc('→', "a161", 0o325);
            enc('↔', "a163", 0o326);
            enc('↕', "a164", 0o327);
            enc('➘', "a196", 0o330);
            enc('➙', "a165", 0o331);
            enc('➚', "a192", 0o332);
            enc('➛', "a166", 0o333);
            enc('➜', "a167", 0o334);
            enc('➝', "a168", 0o335);
            enc('➞', "a169", 0o336);
            enc('➟', "a170", 0o337);
            enc('➠', "a171", 0o340);
            enc('➡', "a172", 0o341);
            enc('➢', "a173", 0o342);
            enc('➣', "a162", 0o343);
            enc('➤', "a174", 0o344);
            enc('➥', "a175", 0o345);
            enc('➦', "a176", 0o346);
            enc('➧', "a177", 0o347);
            enc('➨', "a178", 0o350);
            enc('➩', "a179", 0o351);
            enc('➪', "a193", 0o352);
            enc('➫', "a180", 0o353);
            enc('➬', "a199", 0o354);
            enc('➭', "a181", 0o355);
            enc('➮', "a200", 0o356);
            enc('➯', "a182", 0o357);
            enc('➱', "a201", 0o361);
            enc('➲', "a183", 0o362);
            enc('➳', "a184", 0o363);
            enc('➴', "a197", 0o364);
            enc('➵', "a185", 0o365);
            enc('➶', "a194", 0o366);
            enc('➷', "a198", 0o367);
            enc('➸', "a186", 0o370);
            enc('➹', "a195", 0o371);
            enc('➺', "a187", 0o372);
            enc('➻', "a188", 0o373);
            enc('➼', "a189", 0o374);
            enc('➽', "a190", 0o375);
            enc('➾', "a191", 0o376);
        }
        Encoding::new("ZapfDingbatsEncoding".to_string(), names, codes)
    };
}

#[test]
fn test_get_winansi_points() {
    let ref enc = WIN_ANSI_ENCODING;
    assert_eq!(Some('A' as u8), enc.get_code("A"));
    assert_eq!(Some('Z' as u8), enc.get_code("Z"));
    assert_eq!(Some('a' as u8), enc.get_code("a"));
    assert_eq!(Some('z' as u8), enc.get_code("z"));
    assert_eq!(Some(' ' as u8), enc.get_code("space"));
    assert_eq!(Some('&' as u8), enc.get_code("ampersand"));
}
