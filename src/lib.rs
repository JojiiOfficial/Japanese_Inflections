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

    pub fn try_kana(&self, kana: bool) -> String {
        if kana {
            return self.kana.to_owned();
        }

        self.get_reading()
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
                .map(|i| i[..i.len() - kanji_bytes].to_owned()),
            kana: self.kana[..self.kana.len() - kana_bytes].to_owned(),
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
            WordForm::Short => self.nai_stem(),
            WordForm::Long => self.stem_long(),
        }
    }

    /// Returns the dictionary form of a word
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.dictionary(WordForm::Long).unwrap().kana, String::from("ならいます"));
    /// assert_eq!(verb.dictionary(WordForm::Long).unwrap().kanji.unwrap(), String::from("習います"));
    ///
    /// assert_eq!(verb.dictionary(WordForm::Short).unwrap().kana, String::from("ならう"));
    /// assert_eq!(verb.dictionary(WordForm::Short).unwrap().kanji.unwrap(), String::from("習う"));
    /// ```
    pub fn dictionary(&self, form: WordForm) -> JapaneseResult<Word> {
        Ok(match form {
            WordForm::Short => self.word.clone(),
            WordForm::Long => self.dictionary_polite()?,
        })
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
        if self.word.kana == "いらっしゃる" {
            return Ok(Word::new("いらして", None));
        }

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

    /// Returns the verb in the potential form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.potential(WordForm::Short).unwrap().kana, String::from("ならえる"));
    /// assert_eq!(verb.potential(WordForm::Short).unwrap().kanji.unwrap(), String::from("習える"));
    ///
    /// assert_eq!(verb.potential(WordForm::Long).unwrap().kana, String::from("ならえます"));
    /// assert_eq!(verb.potential(WordForm::Long).unwrap().kanji.unwrap(), String::from("習えます"));
    /// ```
    pub fn potential(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.potential_short(),
            WordForm::Long => self.potential_long(),
        }
    }

    /// Returns the verb in the negative potential form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative_potential(WordForm::Short).unwrap().kana, String::from("ならえない"));
    /// assert_eq!(verb.negative_potential(WordForm::Short).unwrap().kanji.unwrap(), String::from("習えない"));
    ///
    /// assert_eq!(verb.negative_potential(WordForm::Long).unwrap().kana, String::from("ならえません"));
    /// assert_eq!(verb.negative_potential(WordForm::Long).unwrap().kanji.unwrap(), String::from("習えません"));
    /// ```
    pub fn negative_potential(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.negative_potential_short(),
            WordForm::Long => self.negative_potential_long(),
        }
    }

    /// Returns the verb in the imperative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.imperative().unwrap().kana, String::from("たべろ"));
    /// assert_eq!(verb.imperative().unwrap().kanji.unwrap(), String::from("食べろ"));
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.imperative().unwrap().kana, String::from("ならえ"));
    /// assert_eq!(verb.imperative().unwrap().kanji.unwrap(), String::from("習え"));
    /// ```
    pub fn imperative(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            let mut stripped = self.word.clone().strip_end(1);
            stripped.push_str("ろ");
            return Ok(stripped);
        }

        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("しろ"),
                        kanji: Some(String::from("為ろ")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("しろ");
                return Ok(prefix);
            } else if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("こい"),
                    kanji: Some("来い".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        if self.is_polite() {
            return Ok(self.word.clone().strip_end(1).push_str("い").to_owned());
        }

        self.stem_potential()
    }

    /// Returns the verb in the negative imperative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.imperative_negative().unwrap().kana, String::from("たべるな"));
    /// assert_eq!(verb.imperative_negative().unwrap().kanji.unwrap(), String::from("食べるな"));
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.imperative_negative().unwrap().kana, String::from("ならうな"));
    /// assert_eq!(verb.imperative_negative().unwrap().kanji.unwrap(), String::from("習うな"));
    /// ```
    pub fn imperative_negative(&self) -> JapaneseResult<Word> {
        let mut stripped = self.word.clone();
        stripped.push_str("な");
        return Ok(stripped);
    }

    /// Returns the verb in the causative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.causative().unwrap().kana, String::from("たべさせる"));
    /// assert_eq!(verb.causative().unwrap().kanji.unwrap(), String::from("食べさせる"));
    /// ```
    pub fn causative(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            let mut stripped = self.word.clone().strip_end(1);
            stripped.push_str("させる");
            return Ok(stripped);
        }

        if self.is_exception() {
            if self.word.ends_with("する", Some("為る")) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("させる"),
                        kanji: Some(String::from("為せる")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("させる");
                return Ok(prefix);
            } else if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("こさせる"),
                    kanji: Some("来させる".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        let mut short_stem = self.nai_stem()?;
        short_stem.push_str("せる");
        Ok(short_stem)
    }

    /// Returns the verb in the passive-causative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.causative_passive().unwrap().kana, String::from("たべさせられる"));
    /// assert_eq!(verb.causative_passive().unwrap().kanji.unwrap(), String::from("食べさせられる"));
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.causative_passive().unwrap().kanji.unwrap(), String::from("習わされる"));
    /// ```
    pub fn causative_passive(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            let mut stripped = self.word.clone().strip_end(1);
            stripped.push_str("させられる");
            return Ok(stripped);
        }

        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("させられる"),
                        kanji: Some(String::from("為せられる")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("させられる");
                return Ok(prefix);
            } else if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("こさせられる"),
                    kanji: Some("来させられる".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        let mut short_stem = self.nai_stem()?;
        short_stem.push_str("される");
        Ok(short_stem)
    }

    /// Returns the verb in the negative passive-causative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.negative_causative_passive().unwrap().kana, String::from("たべさせられない"));
    /// assert_eq!(verb.negative_causative_passive().unwrap().kanji.unwrap(), String::from("食べさせられない"));
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative_causative_passive().unwrap().kanji.unwrap(), String::from("習わされない"));
    /// ```
    pub fn negative_causative_passive(&self) -> JapaneseResult<Word> {
        let mut causative_passive = self.causative_passive()?.strip_end(1);
        causative_passive.push_str("ない");
        Ok(causative_passive)
    }

    /// Returns the verb in the negative causative form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.negative_causative().unwrap().kana, String::from("たべさせない"));
    /// assert_eq!(verb.negative_causative().unwrap().kanji.unwrap(), String::from("食べさせない"));
    /// ```
    pub fn negative_causative(&self) -> JapaneseResult<Word> {
        let causative = self.causative()?;
        let mut negative_causative = causative.strip_end(1);
        negative_causative.push_str("ない");
        Ok(negative_causative)
    }

    /// Returns the verb in the passive form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.passive().unwrap().kana, String::from("たべられる"));
    /// assert_eq!(verb.passive().unwrap().kanji.unwrap(), String::from("食べられる"));
    /// ```
    pub fn passive(&self) -> JapaneseResult<Word> {
        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("される"),
                        kanji: Some(String::from("為れる")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("される");
                return Ok(prefix);
            } else if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("こられる"),
                    kanji: Some("来られる".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        let mut short_stem = self.nai_stem()?;
        if self.verb_type == VerbType::Ichidan {
            short_stem.push('ら');
        }
        short_stem.push_str("れる");
        Ok(short_stem)
    }

    /// Returns the verb in the negative passive form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.negative_passive().unwrap().kana, String::from("たべられない"));
    /// assert_eq!(verb.negative_passive().unwrap().kanji.unwrap(), String::from("食べられない"));
    /// ```
    pub fn negative_passive(&self) -> JapaneseResult<Word> {
        let passive = self.passive()?;
        let mut negative_passive = passive.strip_end(1);
        negative_passive.push_str("ない");
        Ok(negative_passive)
    }

    /// Returns the verb in the tara form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.tara().unwrap().kana, String::from("たべたら"));
    /// assert_eq!(verb.tara().unwrap().kanji.unwrap(), String::from("食べたら"));
    /// ```
    pub fn tara(&self) -> JapaneseResult<Word> {
        let mut ta_form = self.past(WordForm::Short)?;
        ta_form.push_str("ら");
        Ok(ta_form)
    }

    /// Returns the verb in the negative tara form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.negative_tara().unwrap().kana, String::from("たべなかったら"));
    /// assert_eq!(verb.negative_tara().unwrap().kanji.unwrap(), String::from("食べなかったら"));
    /// ```
    pub fn negative_tara(&self) -> JapaneseResult<Word> {
        let mut ta_form = self.negative_past(WordForm::Short)?;
        ta_form.push_str("ら");
        Ok(ta_form)
    }

    /// Returns the verb in the ba form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.ba().unwrap().kana, String::from("たべれば"));
    /// assert_eq!(verb.ba().unwrap().kanji.unwrap(), String::from("食べれば"));
    /// ```
    pub fn ba(&self) -> JapaneseResult<Word> {
        let mut e_stem = self.ba_stem()?;
        e_stem.push_str("ば");
        Ok(e_stem)
    }

    /// Returns the verb in the negative ba form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("たべる", Some("食べる")).into_verb(VerbType::Ichidan).unwrap();
    /// assert_eq!(verb.negative_ba().unwrap().kana, String::from("たべなければ"));
    /// assert_eq!(verb.negative_ba().unwrap().kanji.unwrap(), String::from("食べなければ"));
    /// ```
    pub fn negative_ba(&self) -> JapaneseResult<Word> {
        let mut negative = self.negative(WordForm::Short)?.strip_end(1);
        negative.push_str("ければ");
        Ok(negative)
    }

    /// Returns the verb in the volitional form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.volitional(WordForm::Short).unwrap().kana, String::from("ならおう"));
    /// assert_eq!(verb.volitional(WordForm::Short).unwrap().kanji.unwrap(), String::from("習おう"));
    ///
    /// assert_eq!(verb.volitional(WordForm::Long).unwrap().kana, String::from("ならいましょう"));
    /// assert_eq!(verb.volitional(WordForm::Long).unwrap().kanji.unwrap(),
    /// String::from("習いましょう"));
    /// ```
    pub fn volitional(&self, form: WordForm) -> JapaneseResult<Word> {
        match form {
            WordForm::Short => self.volitional_short(),
            WordForm::Long => self.volitional_long(),
        }
    }

    /// Returns the verb in the negative volitional form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.negative_volitional().unwrap().kana, String::from("ならうまい"));
    /// assert_eq!(verb.negative_volitional().unwrap().kanji.unwrap(), String::from("習うまい"));
    /// ```
    pub fn negative_volitional(&self) -> JapaneseResult<Word> {
        let mut word = self.word.clone();
        word.push_str("まい");
        Ok(word)
    }

    /// Returns the verb in the zu form
    ///
    /// # Example
    /// ```
    /// use jp_inflections::{Word, VerbType, WordForm};
    ///
    /// let verb = Word::new("ならう", Some("習う")).into_verb(VerbType::Godan).unwrap();
    /// assert_eq!(verb.zu().unwrap().kana, String::from("ならわず"));
    /// assert_eq!(verb.zu().unwrap().kanji.unwrap(), String::from("習わず"));
    /// ```
    pub fn zu(&self) -> JapaneseResult<Word> {
        if self.word.ends_with("する", None) {
            if self.word.kana == "する" {
                return Ok(Word {
                    kana: String::from("せず"),
                    kanji: Some(String::from("為ず")),
                    inflections: vec![],
                });
            }

            let mut word = self.word.clone().strip_end(2);
            word.push_str("せず");
            return Ok(word);
        }
        let mut word = self.negative(WordForm::Short)?.strip_end(2);
        word.push_str("ず");
        Ok(word)
    }

    /// Returns the short negative potential form of the verb
    fn negative_potential_short(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_potential()?;
        stem.push_str("ない");
        Ok(stem)
    }

    /// Returns the long negative potential form of the verb
    fn negative_potential_long(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_potential()?;
        stem.push_str("ません");
        Ok(stem)
    }

    /// Returns the short potential form of the verb
    fn potential_short(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_potential()?;
        stem.push('る');
        Ok(stem)
    }

    /// Returns the long potential form of the verb
    fn potential_long(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_potential()?;
        stem.push_str("ます");
        Ok(stem)
    }

    /// Returns the polite present form of the word
    fn dictionary_polite(&self) -> JapaneseResult<Word> {
        let mut stem = self.stem_long()?;
        stem.push_str("ます");
        Ok(stem)
    }

    /// Returns the verb in the negative short past form
    fn negative_past_short(&self) -> JapaneseResult<Word> {
        if self.word.has_reading("ある", None) {
            return Ok(Word::new("なかった", None));
        }

        let mut negative_past = self.nai_stem()?;
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

        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    let kanji = format!("為{}", to_append);
                    return Ok(Word::new(format!("し{}", to_append), Some(kanji)));
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str(format!("し{}", to_append).as_str());
                return Ok(prefix);
            }

            if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word::new(
                    format!("き{}", to_append),
                    Some(format!("来{}", to_append)),
                ));
            }
        }

        if self.word.ends_with("ある", None) {
            return Ok(Word::new(format!("あっ{}", to_append), None));
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

        let mut negative = self.nai_stem()?;
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
    fn nai_stem(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            return Ok(self.word.clone().strip_end(1));
        }

        // Handle exception: 来る
        if self.is_exception() {
            if self.word.has_reading("くる", Some("来る")) {
                return Ok(Word {
                    kanji: Some(String::from("来")),
                    kana: String::from("こ"),
                    inflections: Vec::new(),
                });
            } else if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kanji: Some(String::from("為")),
                        kana: String::from("し"),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("し");
                return Ok(prefix);
            }
        }

        self.mapped_stem(&[
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

        if self.is_exception() {
            if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kanji: Some(String::from("来")),
                    kana: String::from("き"),
                    inflections: Vec::new(),
                });
            } else if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kanji: Some(String::from("為")),
                        kana: String::from("し"),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("し");
                return Ok(prefix);
            }
        }

        if self.is_polite() {
            return Ok(self.word.clone().strip_end(1).push_str("い").to_owned());
        }

        self.mapped_stem(&[
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

    /// Returns the potential stem of the verb
    fn stem_potential(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            return Ok(self.word.clone().strip_end(1).push_str("られ").to_owned());
        }

        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("でき"),
                        kanji: Some(String::from("出来")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("でき");
                return Ok(prefix);
            }

            if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("こられ"),
                    kanji: Some("来られ".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        self.mapped_stem(&[
            ('す', 'せ'),
            ('く', 'け'),
            ('ぐ', 'げ'),
            ('む', 'め'),
            ('ぶ', 'べ'),
            ('ぬ', 'ね'),
            ('る', 'れ'),
            ('う', 'え'),
            ('つ', 'て'),
        ])
    }

    /// Returns the ba stem of the verb
    fn ba_stem(&self) -> JapaneseResult<Word> {
        if self.verb_type == VerbType::Ichidan {
            return Ok(self.word.clone().strip_end(1).push_str("れ").to_owned());
        }

        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("すれ"),
                        kanji: Some(String::from("為れ")),
                        inflections: Vec::new(),
                    });
                }

                let mut prefix = self.word.clone().strip_end(2);
                prefix.push_str("すれ");
                return Ok(prefix);
            }

            if self.word.ends_with("くる", Some("来る")) {
                return Ok(Word {
                    kana: String::from("くれ"),
                    kanji: Some("来れ".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        self.mapped_stem(&[
            ('す', 'せ'),
            ('く', 'け'),
            ('ぐ', 'げ'),
            ('む', 'め'),
            ('ぶ', 'べ'),
            ('ぬ', 'ね'),
            ('る', 'れ'),
            ('う', 'え'),
            ('つ', 'て'),
        ])
    }

    /// Returns the word in the short volitional form
    fn volitional_short(&self) -> JapaneseResult<Word> {
        let mut stem = self.volitional_stem()?;
        stem.push_str("う");
        Ok(stem)
    }

    /// Returns the word in the long volitional form
    fn volitional_long(&self) -> JapaneseResult<Word> {
        let mut stem = self.dictionary(WordForm::Long)?.strip_end(1);
        stem.push_str("しょう");
        Ok(stem)
    }

    /// Returns the volitional stem of the verb
    fn volitional_stem(&self) -> JapaneseResult<Word> {
        if self.is_exception() {
            if self.word.ends_with("する", None) {
                if self.word.kana == "する" {
                    return Ok(Word {
                        kana: String::from("しよ"),
                        kanji: Some("為よ".to_owned()),
                        inflections: Vec::new(),
                    });
                }
                let mut word = self.word.clone().strip_end(2);
                word.push_str("しよ");
                return Ok(word);
            }
            if self.word.ends_with("くる", None) {
                return Ok(Word {
                    kana: String::from("こよ"),
                    kanji: Some("来よ".to_owned()),
                    inflections: Vec::new(),
                });
            }
        }

        if self.verb_type == VerbType::Ichidan {
            let mut word = self.word.clone().strip_end(1);
            word.push_str("よ");
            return Ok(word);
        }

        self.mapped_stem(&[
            ('す', 'そ'),
            ('く', 'こ'),
            ('ぐ', 'ご'),
            ('む', 'も'),
            ('ぶ', 'ぼ'),
            ('ぬ', 'の'),
            ('る', 'ろ'),
            ('う', 'お'),
            ('つ', 'と'),
        ])
    }

    /// Returns the stem of a word using [`mappings`]
    fn mapped_stem(&self, mappings: &[(char, char)]) -> JapaneseResult<Word> {
        let word = &self.word.kana;

        if word.ends_with("する") && self.is_exception() {
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

    /// Returuns `true` if verb_type is exception
    fn is_exception(&self) -> bool {
        self.verb_type == VerbType::Exception
    }

    /// Returns `true` if the verb is one of the 5 polite verbs
    fn is_polite(&self) -> bool {
        match self.word.kana.as_str() {
            "いらっしゃる" => true,
            "おっしゃる" => true,
            "くださる" => true,
            "ござる" => true,
            "なさる" => true,
            _ => false,
        }
    }
}
