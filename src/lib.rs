pub mod alphabet;
pub mod error;
pub mod inflection;
pub mod syllable;
pub mod umlaut;

use error::Error;
use inflection::Inflection;
use syllable::Syllable;
use umlaut::Umlaut;

type JapaneseResult<T> = Result<T, Error>;

/// Represents a japanese word
#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub kana: String,
    pub kanji: Option<String>,
    pub inflections: Vec<Inflection>,
}

/// Represents a Japanese verb
#[derive(Debug, Clone, PartialEq)]
pub struct Verb {
    pub word: Word,
    pub verb_type: VerbType,
}

/// Represents a type of verb
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerbType {
    Godan,
    Ichidan,
    /// する,来る,...
    Exception,
}

/// The form of a word.
///
/// Example:
/// [`Short`]: しない
/// [`Long`]: しません
#[derive(Debug, Clone, Copy, PartialEq)]
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

    /// Returns the last syllable of the word
    fn ending_syllable(&self) -> Option<Syllable> {
        self.kana.chars().last().map(Syllable::from)
    }

    /// Remove last n characters from [`self`]
    fn strip_end(self, n: usize) -> Word {
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
                .map(|i| i[..i.len() - kana_bytes].to_owned()),
            kana: self.kana[..self.kana.len() - kanji_bytes].to_owned(),
        }
    }

    /// Pushes a &str onto the end of the kana and kanji word
    fn push_str(&mut self, s: &str) -> &mut Word {
        self.kana.push_str(s);
        if let Some(ref mut kanji) = self.kanji {
            kanji.push_str(s);
        }
        self
    }

    /// Pushes a char onto the end of the kana and kanji word
    fn push(&mut self, c: char) -> &mut Word {
        self.kana.push(c);
        if let Some(ref mut kanji) = self.kanji {
            kanji.push(c);
        }
        self
    }

    /// Retuns a `Error::NotAVerb` error if self is not a verb
    fn require_verb(&self) -> JapaneseResult<()> {
        Ok(self.is_verb().then(|| ()).ok_or(Error::NotAVerb)?)
    }
}

impl Verb {
    /// Returns a new verb
    pub fn new(word: Word, verb_type: VerbType) -> Self {
        Self { word, verb_type }
    }

    /// Same as Word::get_reading(&self)
    pub fn get_reading(&self) -> String {
        self.word.get_reading()
    }

    /// Returns true if [`self`] is in dictionary form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert!(verb.is_dict_form());
    /// ```
    pub fn is_dict_form(&self) -> bool {
        self.word.inflections.is_empty()
    }

    /// Returns the stem of a word
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.get_stem(WordForm::Long).unwrap().kana, String::from("ならい"));
    /// assert_eq!(verb.get_stem(WordForm::Long).unwrap().kanji.unwrap(), String::from("習い"));
    ///
    /// assert_eq!(verb.get_stem(WordForm::Short).unwrap().kana, String::from("ならわ"));
    /// assert_eq!(verb.get_stem(WordForm::Short).unwrap().kanji.unwrap(), String::from("習わ"));
    /// ```
    pub fn get_stem(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.stem_short(),
            WordForm::Long => self.stem_long(),
        }
    }

    /// Returns the negative form of a verb
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative(WordForm::Short).unwrap().kana, String::from("ならわない"));
    /// assert_eq!(verb.negative(WordForm::Short).unwrap().kanji.unwrap(), String::from("習わない"));
    ///
    /// assert_eq!(verb.negative(WordForm::Long).unwrap().kana, String::from("ならいません"));
    /// assert_eq!(verb.negative(WordForm::Long).unwrap().kanji.unwrap(), String::from("習いません"));
    /// ```
    pub fn negative(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.negative_short(),
            WordForm::Long => self.negative_long(),
        }
    }

    /// Returns the verb in its て form.
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.te_form().unwrap().kana, String::from("ならって"));
    /// assert_eq!(verb.te_form().unwrap().kanji.unwrap(), String::from("習って"));
    /// ```
    pub fn te_form(&self) -> JapaneseResult<Word> {
        self.te_rule(Syllable::from('て'))
    }

    /// Returns the verb in its negative て form.
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative_te_form().unwrap().kana, String::from("ならわなくて"));
    /// assert_eq!(verb.negative_te_form().unwrap().kanji.unwrap(), String::from("習わなくて"));
    /// ```
    pub fn negative_te_form(&self) -> JapaneseResult<Word> {
        let mut negated_short = self.negative_short()?.strip_end(1);
        negated_short.push_str("くて");
        Ok(negated_short)
    }

    /// Returns the verb in the past form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.past(WordForm::Short).unwrap().kana, String::from("ならった"));
    /// assert_eq!(verb.past(WordForm::Short).unwrap().kanji.unwrap(), String::from("習った"));
    ///
    /// assert_eq!(verb.past(WordForm::Long).unwrap().kana, String::from("ならいました"));
    /// assert_eq!(verb.past(WordForm::Long).unwrap().kanji.unwrap(), String::from("習いました"));
    /// ```
    pub fn past(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.past_short(),
            WordForm::Long => self.past_long(),
        }
    }

    /// Returns the verb in the negative past form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative_past(WordForm::Short).unwrap().kana, String::from("ならわなかった"));
    /// assert_eq!(verb.negative_past(WordForm::Short).unwrap().kanji.unwrap(), String::from("習わなかった"));
    ///
    /// assert_eq!(verb.negative_past(WordForm::Long).unwrap().kana, String::from("ならいませんでした"));
    /// assert_eq!(verb.negative_past(WordForm::Long).unwrap().kanji.unwrap(), String::from("習いませんでした"));
    /// ```
    pub fn negative_past(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.negative_past_short(),
            WordForm::Long => self.negative_past_long(),
        }
    }

    /// Returns the verb in the negative short past form
    fn negative_past_short(&self) -> JapaneseResult<Word> {
        if self.word.has_reading("ある", None) {
            return Ok(Word::new("なかった", None));
        }

        let mut negative_past = self.stem_short()?;
        negative_past.push_str("なかった");
        Ok(negative_past)
    }

    /// Returns the verb in the negative long past form
    fn negative_past_long(&self) -> JapaneseResult<Word> {
        let mut negative_past = self.stem_long()?;
        negative_past.push_str("ませんでした");
        Ok(negative_past)
    }

    /// Returns a word conjungated like て from but with a custom character instead of て
    fn te_rule(&self, to_append: Syllable) -> JapaneseResult<Word> {
        if self.word.ends_with("いく", Some("行く")) {
            return Ok(Word::new(
                format!("いっ{}", to_append),
                Some(format!("行っ{}", to_append)),
            ));
        }

        if self.word.ends_with("する", None) {
            return Ok(Word::new(format!("し{}", to_append), None));
        }

        if self.word.ends_with("ある", None) {
            return Ok(Word::new(format!("あっ{}", to_append), None));
        }

        if self.word.ends_with("くる", Some("来る")) {
            return Ok(Word::new(
                format!("き{}", to_append),
                Some(format!("来{}", to_append)),
            ));
        }

        match self.verb_type {
            VerbType::Ichidan => Ok(self.te_rule_ichidan(to_append)),
            VerbType::Godan | VerbType::Exception => Ok(self.te_rule_godan(to_append)?),
        }
    }

    /// Applies the て rule for an ichidan verb
    fn te_rule_ichidan(&self, to_append: Syllable) -> Word {
        let mut w = self.word.clone().strip_end(1);
        w.push(to_append.into());
        w
    }

    /// Applies the て rule for a godan verb
    fn te_rule_godan(&self, to_append: Syllable) -> JapaneseResult<Word> {
        let mut new = self.map_ending(&[
            ('す', 'し'),
            ('く', 'い'),
            ('ぐ', 'い'),
            ('む', 'ん'),
            ('ぶ', 'ん'),
            ('ぬ', 'ん'),
            ('る', 'っ'),
            ('う', 'っ'),
            ('つ', 'っ'),
        ])?;

        let mut to_append = to_append;

        // Change `to_append` to だ/で
        let ending = self.word.ending_syllable().unwrap().get_char();
        if matches!(ending, 'ぐ' | 'む' | 'ぶ' | 'ぬ') {
            to_append = to_append.to_dakuten();
        }

        new.push(to_append.into());
        Ok(new)
    }

    /// Returns the verb in the short past form
    fn past_short(&self) -> JapaneseResult<Word> {
        self.te_rule(Syllable::from('た'))
    }

    /// Returns the verb in the long past form
    fn past_long(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_long()?;
        stem.push_str("ました");
        Ok(stem)
    }

    /// Returns the word in the short negative form
    fn negative_short(&self) -> JapaneseResult<Word> {
        if self.word.kana == "ある" {
            return Ok(Word::new("ない", None));
        }

        let mut negative = self.stem_short()?;
        negative.push_str("ない");
        Ok(negative)
    }

    /// Returns the word in the long negative form
    fn negative_long(&self) -> JapaneseResult<Word> {
        let mut negative = self.stem_long()?;
        negative.push_str("ません");
        Ok(negative)
    }

    /// Returns the short stem of a the verb
    fn stem_short(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            return Ok(self.word.clone().strip_end(1));
        }

        // Handle exception: 来る
        if self.word.has_reading("くる", Some("来る")) {
            return Ok(Word {
                kanji: Some(String::from("来")),
                kana: String::from("こ"),
                inflections: Vec::new(),
            });
        }

        self.stem(&[
            ('す', 'さ'),
            ('く', 'か'),
            ('ぐ', 'が'),
            ('む', 'ま'),
            ('ぶ', 'ば'),
            ('ぬ', 'な'),
            ('る', 'ら'),
            ('う', 'わ'),
            ('つ', 'た'),
        ])
    }

    /// Returns the long stem of the verb
    fn stem_long(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            return Ok(self.word.clone().strip_end(1));
        }

        self.stem(&[
            ('す', 'し'),
            ('く', 'き'),
            ('ぐ', 'ぎ'),
            ('む', 'み'),
            ('ぶ', 'び'),
            ('ぬ', 'に'),
            ('る', 'り'),
            ('う', 'い'),
            ('つ', 'ち'),
        ])
    }

    /// Returns the stem of a word using [`mappings`]
    fn stem(&self, mappings: &[(char, char)]) -> JapaneseResult<Word> {
        let word = &self.word.kana;

        if word.ends_with("する") {
            return Ok(self.word.clone().strip_end(2).push('し').to_owned());
        }

        Ok(self.map_ending(mappings)?)
    }

    /// Maps the last `char` of the verb using [`mappings`]
    fn map_ending(&self, mappings: &[(char, char)]) -> JapaneseResult<Word> {
        let ending = self.word.ending_syllable().ok_or(Error::UnexpectedEnding)?;
        let mut new_word = self.word.clone().strip_end(1);

        for (src, dst) in mappings {
            if ending.get_char() == *src {
                new_word.push(*dst);
                return Ok(new_word);
            }
        }

        Err(Error::UnexpectedEnding)
    }
}
