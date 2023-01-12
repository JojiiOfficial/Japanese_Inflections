use super::{KanaKanjiPair, SpecialVerb};
use crate::WordForm;

// 来る
pub struct SpecialKuru;
impl SpecialVerb for SpecialKuru {
    #[inline]
    fn dict() -> KanaKanjiPair<'static> {
        ("くる", Some("来る"))
    }

    #[inline]
    fn stem_potential_suf() -> KanaKanjiPair<'static> {
        ("こられ", Some("来られ"))
    }

    #[inline]
    fn stem_ba_suf() -> KanaKanjiPair<'static> {
        ("これ", Some("来れ"))
    }

    #[inline]
    fn stem_suf(wf: WordForm) -> KanaKanjiPair<'static> {
        match wf {
            WordForm::Short => ("こ", Some("来")),
            WordForm::Long => ("き", Some("来")),
        }
    }

    #[inline]
    fn te_form_suf() -> KanaKanjiPair<'static> {
        ("きて", Some("来て"))
    }

    #[inline]
    fn past_suf() -> KanaKanjiPair<'static> {
        ("きた", Some("来た"))
    }

    #[inline]
    fn negative_suf(wf: WordForm) -> KanaKanjiPair<'static> {
        match wf {
            WordForm::Short => ("こない", Some("来ない")),
            WordForm::Long => ("きません", Some("来ません")),
        }
    }

    #[inline]
    fn causative_passive_suf() -> KanaKanjiPair<'static> {
        ("こさせられる", Some("来させられる"))
    }

    #[inline]
    fn causative_suf() -> KanaKanjiPair<'static> {
        ("こさせる", Some("来させる"))
    }

    #[inline]
    fn passive_suf() -> KanaKanjiPair<'static> {
        ("こられる", Some("来られる"))
    }

    fn imperative_suf() -> KanaKanjiPair<'static> {
        ("こい", Some("来い"))
    }
}

#[cfg(test)]
mod test {
    use super::SpecialKuru;
    use super::SpecialVerb;
    use crate::inflection::Inflection;
    use crate::Word;
    use crate::WordForm;

    #[test]
    fn test_special_kuru() {
        let word_kuru = Word::new("くる", Some("来る"))
            .into_verb(crate::VerbType::Exception)
            .unwrap();

        let past = SpecialKuru::format_verb(&word_kuru, Inflection::Past, WordForm::Long).unwrap();
        assert_eq!(past.kana, "きた");
        assert_eq!(past.kanji, Some("来た".to_string()));

        let neg =
            SpecialKuru::format_verb(&word_kuru, Inflection::Negative, WordForm::Short).unwrap();
        assert_eq!(neg.kana, "こない");
        assert_eq!(neg.kanji, Some("来ない".to_string()));
    }

    #[test]
    fn test_special_kuru_ending() {
        let word_kuru = Word::new("よせくる", Some("寄せ来る"))
            .into_verb(crate::VerbType::Exception)
            .unwrap();

        let past = SpecialKuru::format_verb(&word_kuru, Inflection::Past, WordForm::Long).unwrap();
        assert_eq!(past.kana, "よせきた");
        assert_eq!(past.kanji, Some("寄せ来た".to_string()));

        let neg =
            SpecialKuru::format_verb(&word_kuru, Inflection::Negative, WordForm::Short).unwrap();
        assert_eq!(neg.kana, "よせこない");
        assert_eq!(neg.kanji, Some("寄せ来ない".to_string()));
    }
}
