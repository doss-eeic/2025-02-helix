use spellbook::{Dictionary, ParseDictionaryError};

pub struct SpellChecker {
    dict: Dictionary,
}

impl SpellChecker {
    pub fn new(aff: &str, dic: &str) -> Result<Self, ParseDictionaryError> {
        let dict = Dictionary::new(aff, dic)?;
        Ok(Self { dict })
    }

    pub fn check_text(&self, text: &str) -> Vec<(usize, usize, String)> {
        let mut out = Vec::new();
        let mut suggs = Vec::new();

        for (start, end) in byte_word_ranges(text) {
            let word = &text[start..end];
            if !self.dict.check(word) {
                suggs.clear();
                self.dict.suggest(word, &mut suggs);
                let first = suggs.first().cloned().unwrap_or_default();
                out.push((start, end, first));
            }
        }
        out
    }
}

fn byte_word_ranges(s: &str) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut in_word = false;
    let mut start = 0usize;
    for (i, ch) in s.char_indices() {
        let is_word = ch.is_ascii_alphabetic() || ch == '\'' || ch == '-';
        if is_word {
            if !in_word {
                in_word = true;
                start = i;
            }
        } else if in_word {
            res.push((start, i));
            in_word = false;
        }
    }
    if in_word {
        res.push((start, s.len()));
    }
    res
}
