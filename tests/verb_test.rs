use jp_inflections::{self, error, Verb, VerbType, Word};

#[derive(Debug, Clone)]
pub struct VerbTest<'a> {
    kana: &'a str,
    kanji: Option<&'a str>,
    verb_type: VerbType,
    results: Vec<AssertedResult<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct AssertedResult<'a> {
    kana: &'a str,
    kanji: Option<&'a str>,
}

impl<'a> AssertedResult<'a> {
    pub fn new(kana: &'a str, kanji: Option<&'a str>) -> AssertedResult<'a> {
        AssertedResult { kana, kanji }
    }
}

impl<'a> VerbTest<'a> {
    pub fn new(
        kana: &'a str,
        kanji: Option<&'a str>,
        verb_type: VerbType,
        results: Vec<AssertedResult<'a>>,
    ) -> VerbTest<'a> {
        VerbTest {
            kana,
            kanji,
            verb_type,
            results,
        }
    }

    pub fn run<const N: usize>(self, f: [fn(&Verb) -> Result<Word, error::Error>; N]) {
        let verb = self.get_verb();

        for (exp_result, fun) in self.results.into_iter().zip(f.iter()) {
            let result = fun(&verb).expect("Conjugation failed");

            assert_eq!(result.kana, exp_result.kana);
            if let Some(kanji) = exp_result.kanji {
                assert!(result.kanji.is_some());
                assert_eq!(result.kanji.unwrap(), kanji);
            }
        }
    }

    fn get_verb(&self) -> Verb {
        get_verb(self.kana, self.kanji, self.verb_type)
    }
}

fn get_verb(kana: &str, kanji: Option<&str>, verb_type: VerbType) -> Verb {
    Word::new(kana, kanji)
        .into_verb(verb_type)
        .expect("Verb conversion failed")
}
