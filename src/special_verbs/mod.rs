pub mod kuru;

use crate::{inflection::Inflection, Word, WordForm};

pub type KanaKanjiPair<'s> = (&'s str, Option<&'s str>);

pub trait SpecialVerb {
    fn dict() -> KanaKanjiPair<'static>;

    fn to_strip_suffix() -> KanaKanjiPair<'static> {
        Self::dict()
    }

    fn stem_suf(wf: WordForm) -> KanaKanjiPair<'static>;
    fn stem_ba_suf() -> KanaKanjiPair<'static>;
    fn stem_potential_suf() -> KanaKanjiPair<'static>;
    fn te_form_suf() -> KanaKanjiPair<'static>;
    fn past_suf() -> KanaKanjiPair<'static>;
    fn negative_suf(wf: WordForm) -> KanaKanjiPair<'static>;
    fn passive_suf() -> KanaKanjiPair<'static>;
    fn causative_suf() -> KanaKanjiPair<'static>;
    fn causative_passive_suf() -> KanaKanjiPair<'static>;
    fn imperative_suf() -> KanaKanjiPair<'static>;

    fn format_verb(word: &Word, inflection: Inflection, wf: WordForm) -> Option<Word> {
        let fmt = |word: &Word, replace: KanaKanjiPair| -> Option<Word> {
            let (kana_s, kanji_s) = Self::to_strip_suffix();
            word.new_with_suffix_replaced(kana_s, kanji_s, replace.0, replace.1)
        };

        match inflection {
            Inflection::Stem => fmt(word, Self::stem_suf(wf)),
            Inflection::StemPotential => fmt(word, Self::stem_potential_suf()),
            Inflection::Te => fmt(word, Self::te_form_suf()),
            Inflection::Past => fmt(word, Self::past_suf()),
            Inflection::Passive => fmt(word, Self::passive_suf()),
            Inflection::Causative => fmt(word, Self::causative_suf()),
            Inflection::CausativePassive => fmt(word, Self::causative_passive_suf()),
            Inflection::Negative => fmt(word, Self::negative_suf(wf)),
            Inflection::Imperative => fmt(word, Self::imperative_suf()),
            _ => None,
        }
    }
}
