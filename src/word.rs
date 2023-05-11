use crate::{
    error::Error,
    inflection::Inflection,
    syllable::Syllable,
    umlaut::Umlaut,
    verb::{Verb, VerbType},
    JapaneseResult,
};

/// Represents a japanese word
#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub kana: String,
    pub kanji: Option<String>,
    pub inflections: Vec<Inflection>,
}

/// The form of a word.
///
/// Example:
/// [`Short`]: しない
/// [`Long`]: しません
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordForm {
    Short,
    Long,
}

impl Word {
    /// Creates a new [`Word`] value of a kana and optionally kanji word. Requires both words to be
    /// in the dictionary form
    pub fn new<S: AsRef<str>>(kana: S, kanji: Option<S>) -> Word {
        Word {
            kana: kana.as_ref().to_owned(),
            kanji: kanji.map(|i| i.as_ref().to_owned()),
            inflections: Vec::new(),
        }
    }

    #[inline]
    pub fn set_kana(&mut self, kana: String) {
        self.kana = kana;
    }

    #[inline]
    pub fn set_kanji(&mut self, kanji: Option<String>) {
        self.kanji = kanji;
    }

    /// Returns `true` if the [`Word`] is a verb
    ///
    /// # Example
    /// ```
    /// use jp_inflections::Word;
    ///
    /// assert!(Word::new("ならう", Some("習う")).is_verb());
    /// assert!(!Word::new("えいご", Some("英語")).is_verb());
    /// ```
    pub fn is_verb(&self) -> bool {
        self.kana
            .chars()
            .last()
            .map(|i| Syllable::from(i).ends_with(Umlaut::U))
            .unwrap_or_default()
    }

    /// Returns a verb from the word. Requires the word to be a verb in the dictionary form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType};
    ///
    /// assert!(Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).is_ok());
    pub fn into_verb(self, verb_type: VerbType) -> JapaneseResult<Verb> {
        self.require_verb()?;

        Ok(Verb::new(self, verb_type))
    }

    /// Returns true if [`self`] has the passed readings. If kanji is none, but the word has a
    /// kanji reading the output represents only a kana match
    pub fn has_reading(&self, kana: &str, kanji: Option<&str>) -> bool {
        if self.kana == kana {
            return true;
        }

        if let Some(ref word_kanji) = self.kanji {
            if let Some(ref search_kanji) = kanji {
                if word_kanji == search_kanji {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the kana and optionally kanji prefix with the given readings stripped or `None` if
    /// the word doesn't end with the given suffix
    pub fn strip_suffix(&self, kana: &str, kanji: Option<&str>) -> Option<(&str, Option<&str>)> {
        let kana_prefix = self.kana.strip_suffix(kana)?;

        let kanji_prefix = match (kanji, &self.kanji) {
            (Some(_), None) => None,
            (Some(suffix), Some(kanji_reading)) => Some(kanji_reading.strip_suffix(suffix)?),
            _ => return None,
        };

        Some((kana_prefix, kanji_prefix))
    }

    /// Tries to strip the given kana and kanji readings from the word and replaces them with the
    /// given new kanji and kana suffixes. Returns `None` if the word doesn't have the given kana, kanji or both suffixes
    pub fn new_with_suffix_replaced(
        &self,
        kana_suffix: impl AsRef<str>,
        kanji_suffix: Option<impl AsRef<str>>,
        new_kana_suffix: impl AsRef<str>,
        new_kanji_suffix: Option<impl AsRef<str>>,
    ) -> Option<Word> {
        let kana = kana_suffix.as_ref();
        let kanji = kanji_suffix.as_ref().map(|i| i.as_ref());

        let (skana, skanji) = self.strip_suffix(kana, kanji)?;

        let new_kana = format!("{skana}{}", new_kana_suffix.as_ref());
        let new_kanji = skanji.and_then(|i| Some(format!("{i}{}", new_kanji_suffix?.as_ref())));

        Some(Word {
            kana: new_kana,
            kanji: new_kanji,
            inflections: Vec::new(),
        })
    }

    /// Returns true if the words readings end with the passed strings. If the kanji is none, but
    /// the word has a kanji reading the output represents only a kana match
    pub fn ends_with(&self, kana: &str, kanji: Option<&str>) -> bool {
        if self.kana.ends_with(kana) {
            return true;
        }

        if let Some(ref word_kanji) = self.kanji {
            if let Some(ref search_kanji) = kanji {
                if word_kanji.ends_with(search_kanji) {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the kanji reading if possible, otherwise the kana reading gets returned
    pub fn get_reading(&self) -> String {
        if let Some(ref kanji) = self.kanji {
            kanji.to_owned()
        } else {
            self.kana.to_owned()
        }
    }

    pub fn try_kana(&self, kana: bool) -> String {
        if kana {
            return self.kana.to_owned();
        }

        self.get_reading()
    }

    /// Returns the last syllable of the word
    pub fn ending_syllable(&self) -> Option<Syllable> {
        self.kana.chars().last().map(Syllable::from)
    }

    /// Remove last n characters from [`self`]
    pub fn strip_end(self, n: usize) -> Word {
        let kana_bytes: usize = self.kana.chars().rev().take(n).map(|i| i.len_utf8()).sum();
        let kanji_bytes: usize = self
            .kanji
            .as_ref()
            .map(|kanji| kanji.chars().rev().take(n).map(|i| i.len_utf8()).sum())
            .unwrap_or_default();

        Word {
            inflections: self.inflections.clone(),
            kanji: self
                .kanji
                .as_ref()
                .map(|i| i[..i.len() - kanji_bytes].to_owned()),
            kana: self.kana[..self.kana.len() - kana_bytes].to_owned(),
        }
    }

    /// Pushes a &str onto the end of the kana and kanji word
    pub fn push_str(&mut self, s: &str) -> &mut Word {
        self.kana.push_str(s);
        if let Some(ref mut kanji) = self.kanji {
            kanji.push_str(s);
        }
        self
    }

    /// Pushes a char onto the end of the kana and kanji word
    pub fn push(&mut self, c: char) -> &mut Word {
        self.kana.push(c);
        if let Some(ref mut kanji) = self.kanji {
            kanji.push(c);
        }
        self
    }

    /// Retuns a `Error::NotAVerb` error if self is not a verb
    pub fn require_verb(&self) -> JapaneseResult<()> {
        self.is_verb().then_some(()).ok_or(Error::NotAVerb)
    }
}
