# Japanese_Inflections
A small rust library to conjugate Japanese words

## Example
```rust
use jp_inflections::{error::Error, VerbType, Word, WordForm};

fn main() -> Result<(), Error> {
    // Word must be in the dictionary form. Kanji reading is optional
    let word = Word::new("しる", Some("知る"));
    let verb = word.into_verb(VerbType::Godan)?; // Can throw an error since only verbs can be conjugated (correctly)

    // Dictionary
    assert_eq!(verb.dictionary(WordForm::Short)?.kanji.unwrap(), "知る");
    assert_eq!(verb.dictionary(WordForm::Long)?.kanji.unwrap(), "知ります");

    // Stem
    assert_eq!(verb.get_stem(WordForm::Short)?.kanji.unwrap(), "知ら");
    assert_eq!(verb.get_stem(WordForm::Long)?.kanji.unwrap(), "知り");

    // Negative
    assert_eq!(verb.negative(WordForm::Short)?.kanji.unwrap(), "知らない");
    assert_eq!(verb.negative(WordForm::Long)?.kanji.unwrap(), "知りません");

    // Past
    assert_eq!(verb.past(WordForm::Short)?.kanji.unwrap(), "知った");
    assert_eq!(verb.past(WordForm::Long)?.kanji.unwrap(), "知りました");

    // Negative past
    assert_eq!(
        verb.negative_past(WordForm::Short)?.kanji.unwrap(),
        "知らなかった"
    );
    assert_eq!(
        verb.negative_past(WordForm::Long)?.kanji.unwrap(),
        "知りませんでした"
    );

    // Te form
    assert_eq!(verb.te_form()?.kanji.unwrap(), "知って");

    // Negative Te form
    assert_eq!(verb.negative_te_form()?.kanji.unwrap(), "知らなくて");

    // Potential
    assert_eq!(verb.potential(WordForm::Short)?.kanji.unwrap(), "知れる");
    assert_eq!(verb.potential(WordForm::Long)?.kanji.unwrap(), "知れます");

    // Potential Negative
    assert_eq!(verb.negative_potential(WordForm::Short)?.kanji.unwrap(), "知れない");
    assert_eq!(verb.negative_potential(WordForm::Long)?.kanji.unwrap(), "知れません");

    Ok(())
}
```
