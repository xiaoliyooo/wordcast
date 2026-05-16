use crate::config::VoiceConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LangHint {
    Korean,
    Japanese,
    English,
    Other,
}

impl LangHint {
    pub fn detect(text: &str) -> Self {
        let mut has_hangul = false;
        let mut has_kana = false;
        let mut has_latin_alpha = false;

        for c in text.chars() {
            let cp = c as u32;
            if (0xAC00..=0xD7AF).contains(&cp) || (0x1100..=0x11FF).contains(&cp) {
                has_hangul = true;
            } else if (0x3040..=0x309F).contains(&cp)
                || (0x30A0..=0x30FF).contains(&cp)
                || (0xFF65..=0xFF9F).contains(&cp)
            {
                has_kana = true;
            } else if c.is_ascii_alphabetic() {
                has_latin_alpha = true;
            }
        }

        if has_hangul {
            Self::Korean
        } else if has_kana {
            Self::Japanese
        } else if has_latin_alpha {
            Self::English
        } else {
            Self::Other
        }
    }

    pub fn voice_for(&self, cfg: &VoiceConfig) -> String {
        match self {
            Self::Korean => cfg.korean.clone(),
            Self::Japanese => cfg.japanese.clone(),
            Self::English => cfg.english.clone(),
            Self::Other => cfg.default.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LangHint;

    #[test]
    fn hangul_is_korean() {
        assert_eq!(LangHint::detect("안녕하세요"), LangHint::Korean);
    }

    #[test]
    fn hiragana_is_japanese() {
        assert_eq!(LangHint::detect("ありがとう"), LangHint::Japanese);
    }

    #[test]
    fn katakana_is_japanese() {
        assert_eq!(LangHint::detect("カタカナ"), LangHint::Japanese);
    }

    #[test]
    fn plain_latin_is_english() {
        assert_eq!(LangHint::detect("Hello world"), LangHint::English);
    }

    #[test]
    fn pure_cjk_falls_to_other() {
        assert_eq!(LangHint::detect("你好世界"), LangHint::Other);
    }

    #[test]
    fn hangul_beats_english_loanword() {
        assert_eq!(LangHint::detect("iPhone 13 안녕"), LangHint::Korean);
    }

    #[test]
    fn kana_beats_english_loanword() {
        assert_eq!(LangHint::detect("PCで作業する"), LangHint::Japanese);
    }

    #[test]
    fn hangul_beats_cjk_hanja() {
        assert_eq!(LangHint::detect("안녕 韓國"), LangHint::Korean);
    }

    #[test]
    fn latin_with_cjk_is_english() {
        assert_eq!(LangHint::detect("Hello 你好"), LangHint::English);
    }

    #[test]
    fn diacritics_with_latin_is_english() {
        assert_eq!(LangHint::detect("café naïve"), LangHint::English);
    }

    #[test]
    fn empty_is_other() {
        assert_eq!(LangHint::detect(""), LangHint::Other);
    }

    #[test]
    fn numbers_only_is_other() {
        assert_eq!(LangHint::detect("123 456"), LangHint::Other);
    }
}
