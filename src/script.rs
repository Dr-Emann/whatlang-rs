use utils::is_stop_char;
use std::fmt;
use fnv::FnvHashMap;
use rayon::prelude::*;

/// Represents a writing system (Latin, Cyrillic, Arabic, etc).
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Script {
    // Keep this in alphabetic order (for C bindings)
    Arabic,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Khmer,
    Latin,
    Malayalam,
    Mandarin,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai,
}

impl Script {
    pub fn name(&self) -> &str {
        match *self {
            Script::Latin      => "Latin",
            Script::Cyrillic   => "Cyrillic",
            Script::Arabic     => "Arabic",
            Script::Devanagari => "Devanagari",
            Script::Hiragana   => "Hiragana",
            Script::Katakana   => "Katakana",
            Script::Ethiopic   => "Ethiopic",
            Script::Hebrew     => "Hebrew",
            Script::Bengali    => "Bengali",
            Script::Georgian   => "Georgian",
            Script::Mandarin   => "Mandarin",
            Script::Hangul     => "Hangul",
            Script::Greek      => "Greek",
            Script::Kannada    => "Kannada",
            Script::Tamil      => "Tamil",
            Script::Thai       => "Thai",
            Script::Gujarati   => "Gujarati",
            Script::Gurmukhi   => "Gurmukhi",
            Script::Telugu     => "Telugu",
            Script::Malayalam  => "Malayalam",
            Script::Oriya      => "Oriya",
            Script::Myanmar    => "Myanmar",
            Script::Sinhala    => "Sinhala",
            Script::Khmer      => "Khmer"
        }
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

type ScriptChecker = (Script, fn(char) -> bool);

/// Detect only a script by a given text
///
/// # Example
/// ```
/// use whatlang::{detect_script, Script};
/// let script = detect_script("Благодаря Эсперанто вы обрётете друзей по всему миру!").unwrap();
/// assert_eq!(script, Script::Cyrillic);
/// ```
pub fn detect_script(text: &str) -> Option<Script> {
    const SCRIPT_COUNTERS: &[ScriptChecker] = &[
        (Script::Latin      , is_latin     ),
        (Script::Cyrillic   , is_cyrillic  ),
        (Script::Arabic     , is_arabic    ),
        (Script::Mandarin   , is_mandarin  ),
        (Script::Devanagari , is_devanagari),
        (Script::Hebrew     , is_hebrew    ),
        (Script::Ethiopic   , is_ethiopic  ),
        (Script::Georgian   , is_georgian  ),
        (Script::Bengali    , is_bengali   ),
        (Script::Hangul     , is_hangul    ),
        (Script::Hiragana   , is_hiragana  ),
        (Script::Katakana   , is_katakana  ),
        (Script::Greek      , is_greek     ),
        (Script::Kannada    , is_kannada   ),
        (Script::Tamil      , is_tamil     ),
        (Script::Thai       , is_thai      ),
        (Script::Gujarati   , is_gujarati  ),
        (Script::Gurmukhi   , is_gurmukhi  ),
        (Script::Telugu     , is_telugu    ),
        (Script::Malayalam  , is_malayalam ),
        (Script::Oriya      , is_oriya     ),
        (Script::Myanmar    , is_myanmar   ),
        (Script::Sinhala    , is_sinhala   ),
        (Script::Khmer      , is_khmer     ),
    ];

    let half = text.chars().count() / 2;

    let counts = text.par_chars().filter(|&ch| !is_stop_char(ch)).filter_map(|ch| {
            SCRIPT_COUNTERS.par_iter().find_any(|&&(_, check_fn)| check_fn(ch)).map(|(script, _)| *script)
        })
        .try_fold(|| vec![0; SCRIPT_COUNTERS.len()], |mut counts, script| {
            // New scope needed until NLL lands
            {
                counts[script as usize] += 1;
                if counts[script as usize] > half {
                    // use Err as an early return
                    return Err(script);
                }
            }
            Ok(counts)
        })
        .try_reduce(|| vec![0; SCRIPT_COUNTERS.len()], |mut counts1, counts2| {
            for (i, (orig_count, &new_count)) in counts1.iter_mut().zip(counts2.iter()).enumerate() {
                *orig_count += new_count;
                if *orig_count > half {
                    return Err(SCRIPT_COUNTERS[i].0)
                }
            }
            Ok(counts1)
        });

    match counts {
        // Early return: A count reached > half
        Err(script) => Some(script),
        Ok(counts) => counts.into_iter().enumerate().max_by_key(|&(_, count)| count).map(|(i, _)| SCRIPT_COUNTERS[i].0),
    }
}

fn is_cyrillic(ch: char) -> bool {
   match ch {
       '\u{0400}'...'\u{0484}' |
       '\u{0487}'...'\u{052F}' |
       '\u{2DE0}'...'\u{2DFF}' |
       '\u{A640}'...'\u{A69D}' |
       '\u{1D2B}' |
       '\u{1D78}' |
       '\u{A69F}' => true,
       _ => false
   }
}

// https://en.wikipedia.org/wiki/Latin_script_in_Unicode
fn is_latin(ch : char) -> bool {
    match ch {
        'a'...'z' |
        'A'...'Z' |
        '\u{0080}'...'\u{00FF}' |
        '\u{0100}'...'\u{017F}' |
        '\u{0180}'...'\u{024F}' |
        '\u{0250}'...'\u{02AF}' |
        '\u{1D00}'...'\u{1D7F}' |
        '\u{1D80}'...'\u{1DBF}' |
        '\u{1E00}'...'\u{1EFF}' |
        '\u{2100}'...'\u{214F}' |
        '\u{2C60}'...'\u{2C7F}' |
        '\u{A720}'...'\u{A7FF}' |
        '\u{AB30}'...'\u{AB6F}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Arabic_script_in_Unicode
fn is_arabic(ch : char) -> bool {
    match ch {
        '\u{0600}'...'\u{06FF}' |
        '\u{0750}'...'\u{07FF}' |
        '\u{08A0}'...'\u{08FF}' |
        '\u{FB50}'...'\u{FDFF}' |
        '\u{FE70}'...'\u{FEFF}' |
        '\u{10E60}'...'\u{10E7F}' |
        '\u{1EE00}'...'\u{1EEFF}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Devanagari#Unicode
fn is_devanagari(ch : char) -> bool {
    match ch {
        '\u{0900}'...'\u{097F}' |
        '\u{A8E0}'...'\u{A8FF}' |
        '\u{1CD0}'...'\u{1CFF}' => true,
        _ => false
    }
}

// Based on https://www.key-shortcut.com/en/writing-systems/ethiopian-script/
fn is_ethiopic(ch : char) -> bool {
    match ch {
        '\u{1200}'...'\u{139F}' |
        '\u{2D80}'...'\u{2DDF}' |
        '\u{AB00}'...'\u{AB2F}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Hebrew_(Unicode_block)
fn is_hebrew(ch : char) -> bool {
    match ch {
        '\u{0590}'...'\u{05FF}' => true,
        _ => false
    }
}

fn is_georgian(ch : char) -> bool {
   match ch {
       '\u{10A0}'...'\u{10FF}' => true,
       _ => false
   }
}

fn is_mandarin(ch : char) -> bool {
    match ch {
        '\u{2E80}'...'\u{2E99}' |
        '\u{2E9B}'...'\u{2EF3}' |
        '\u{2F00}'...'\u{2FD5}' |
        '\u{3005}' |
        '\u{3007}' |
        '\u{3021}'...'\u{3029}' |
        '\u{3038}'...'\u{303B}' |
        '\u{3400}'...'\u{4DB5}' |
        '\u{4E00}'...'\u{9FCC}' |
        '\u{F900}'...'\u{FA6D}' |
        '\u{FA70}'...'\u{FAD9}' => true,
        _ => false
    }
}

fn is_bengali(ch : char) -> bool {
   match ch {
       '\u{0980}'...'\u{09FF}' => true,
       _ => false
   }
}

fn is_hiragana(ch : char) -> bool {
   match ch {
       '\u{3040}'...'\u{309F}' => true,
       _ => false
   }
}

fn is_katakana(ch : char) -> bool {
   match ch {
       '\u{30A0}'...'\u{30FF}' => true,
       _ => false
    }
}


// Hangul is Korean Alphabet. Unicode ranges are taken from: https://en.wikipedia.org/wiki/Hangul
fn is_hangul(ch : char) -> bool {
    match ch {
        '\u{AC00}'...'\u{D7AF}' |
        '\u{1100}'...'\u{11FF}' |
        '\u{3130}'...'\u{318F}' |
        '\u{3200}'...'\u{32FF}' |
        '\u{A960}'...'\u{A97F}' |
        '\u{D7B0}'...'\u{D7FF}' |
        '\u{FF00}'...'\u{FFEF}' => true,
        _ => false
    }
}

// Taken from: https://en.wikipedia.org/wiki/Greek_and_Coptic
fn is_greek(ch : char) -> bool {
    match ch {
        '\u{0370}'...'\u{03FF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Kannada_(Unicode_block)
fn is_kannada(ch : char) -> bool {
    match ch {
        '\u{0C80}'...'\u{0CFF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Tamil_(Unicode_block)
fn is_tamil(ch: char) -> bool {
    match ch {
        '\u{0B80}'...'\u{0BFF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Thai_(Unicode_block)
fn is_thai(ch: char) -> bool {
    match ch {
        '\u{0E00}'...'\u{0E7F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Gujarati_(Unicode_block)
fn is_gujarati(ch: char) -> bool {
    match ch {
        '\u{0A80}'...'\u{0AFF}' => true,
        _ => false
    }
}

// Gurmukhi is the script for Punjabi language.
// Based on: https://en.wikipedia.org/wiki/Gurmukhi_(Unicode_block)
fn is_gurmukhi(ch: char) -> bool {
    match ch {
        '\u{0A00}'...'\u{0A7F}' => true,
        _ => false
    }
}

fn is_telugu(ch: char) -> bool {
    match ch {
        '\u{0C00}'...'\u{0C7F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Malayalam_(Unicode_block)
fn is_malayalam(ch: char) -> bool {
    match ch {
        '\u{0D00}'...'\u{0D7F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Malayalam_(Unicode_block)
fn is_oriya(ch: char) -> bool {
    match ch {
        '\u{0B00}'...'\u{0B7F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Myanmar_(Unicode_block)
fn is_myanmar(ch: char) -> bool {
    match ch {
        '\u{1000}'...'\u{109F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Sinhala_(Unicode_block)
fn is_sinhala(ch: char) -> bool {
    match ch {
        '\u{0D80}'...'\u{0DFF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Khmer_alphabet
fn is_khmer(ch: char) -> bool {
    match ch {
        '\u{1780}'...'\u{17FF}' | '\u{19E0}'...'\u{19FF}' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_name() {
        assert_eq!(Script::Cyrillic.name(), "Cyrillic");
        assert_eq!(Script::Katakana.name(), "Katakana");
    }

    #[test]
    fn test_detect_script() {
        assert_eq!(detect_script(&"1234567890-,;!".to_string()), None);

        // One script
        assert_eq!(detect_script(&"Hello!".to_string()), Some(Script::Latin));
        assert_eq!(detect_script(&"Привет всем!".to_string()), Some(Script::Cyrillic));
        assert_eq!(detect_script(&"ქართული ენა მსოფლიო ".to_string()), Some(Script::Georgian));
        assert_eq!(detect_script(&"県見夜上温国阪題富販".to_string()), Some(Script::Mandarin));
        assert_eq!(detect_script(&" ككل حوالي 1.6، ومعظم الناس ".to_string()), Some(Script::Arabic));
        assert_eq!(detect_script(&"हिमालयी वन चिड़िया (जूथेरा सालिमअली) चिड़िया की एक प्रजाति है".to_string()), Some(Script::Devanagari));
        assert_eq!(detect_script(&"היסטוריה והתפתחות של האלפבית העברי".to_string()), Some(Script::Hebrew));
        assert_eq!(detect_script(&"የኢትዮጵያ ፌዴራላዊ ዴሞክራሲያዊሪፐብሊክ".to_string()), Some(Script::Ethiopic));

        // Mixed scripts
        assert_eq!(detect_script(&"Привет! Текст на русском with some English.".to_string()), Some(Script::Cyrillic));
        assert_eq!(detect_script(&"Russian word любовь means love.".to_string()), Some(Script::Latin));
    }

    #[test]
    fn test_is_latin() {
        assert_eq!(is_latin('z'), true);
        assert_eq!(is_latin('A'), true);
        assert_eq!(is_latin('č'), true);
        assert_eq!(is_latin('š'), true);
        assert_eq!(is_latin('Ĵ'), true);

        assert_eq!(is_latin('ж'), false);
    }

    #[test]
    fn test_is_cyrillic() {
        assert_eq!(is_cyrillic('а'), true);
        assert_eq!(is_cyrillic('Я'), true);
        assert_eq!(is_cyrillic('Ґ'), true);
        assert_eq!(is_cyrillic('ї'), true);
        assert_eq!(is_cyrillic('Ꙕ'), true);

        assert_eq!(is_cyrillic('L'), false);
    }

    #[test]
    fn test_is_ethiopic() {
        assert_eq!(is_ethiopic('ፚ'), true);
        assert_eq!(is_ethiopic('ᎀ'), true);

        assert_eq!(is_ethiopic('а'), false);
        assert_eq!(is_ethiopic('L'), false);
    }

    #[test]
    fn test_is_georgian() {
        assert_eq!(is_georgian('რ'), true);
        assert_eq!(is_georgian('ж'), false);
    }

    #[test]
    fn test_is_bengali() {
        assert_eq!(is_bengali('ই'), true);
        assert_eq!(is_bengali('z'), false);
    }

    #[test]
    fn test_is_katakana() {
        assert_eq!(is_katakana('カ'), true);
        assert_eq!(is_katakana('f'), false);
    }

    #[test]
    fn test_is_hiragana() {
        assert_eq!(is_hiragana('ひ'), true);
        assert_eq!(is_hiragana('a'), false);
    }

    #[test]
    fn test_is_hangul() {
        assert_eq!(is_hangul('ᄁ'), true);
        assert_eq!(is_hangul('t'), false);
    }

    #[test]
    fn test_is_greek() {
        assert_eq!(is_greek('φ'), true);
        assert_eq!(is_greek('ф'), false);
    }

    #[test]
    fn test_is_kannada() {
        assert_eq!(is_kannada('ಡ'), true);
        assert_eq!(is_kannada('S'), false);
    }

    #[test]
    fn test_is_tamil() {
        assert_eq!(is_tamil('ஐ'), true);
        assert_eq!(is_tamil('Ж'), false);
    }

    #[test]
    fn test_is_thai() {
        assert_eq!(is_thai('ก'), true);
        assert_eq!(is_thai('๛'), true);
        assert_eq!(is_thai('Ж'), false);
    }

    #[test]
    fn test_is_gujarati() {
        assert_eq!(is_gujarati('ઁ'), true);
        assert_eq!(is_gujarati('૱'), true);
        assert_eq!(is_gujarati('Ж'), false);
    }

    #[test]
    fn test_is_gurmukhi() {
        assert_eq!(is_gurmukhi('ਁ'), true);
        assert_eq!(is_gurmukhi('ੴ'), true);
        assert_eq!(is_gurmukhi('Ж'), false);
    }

    #[test]
    fn test_is_telugu() {
        assert_eq!(is_telugu('ఁ'), true);
        assert_eq!(is_telugu('౿'), true);
        assert_eq!(is_telugu('Ж'), false);
    }

    #[test]
    fn test_is_oriya() {
        assert_eq!(is_oriya('ଐ'), true);
        assert_eq!(is_oriya('୷'), true);
        assert_eq!(is_oriya('౿'), false);
    }
}
