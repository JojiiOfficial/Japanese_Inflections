mod verb_test;

use jp_inflections::{self, Verb, VerbType, WordForm};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "でかける",
        Some("出かける"),
        VerbType::Ichidan,
        vec![
            AssertedResult::new("でかける", Some("出かける")),
            AssertedResult::new("でかけます", Some("出かけます")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "ならう",
        Some("習う"),
        VerbType::Godan,
        vec![
            AssertedResult::new("ならう", Some("習う")),
            AssertedResult::new("ならいます", Some("習います")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);
}

#[test]
fn suru() {
    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("する", None),
            AssertedResult::new("します", None),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);
}

#[test]
fn kuru() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("くる", Some("来る")),
            AssertedResult::new("きます", Some("来ます")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);
}

#[test]
fn polite_verbs() {
    VerbTest::new(
        "いらっしゃる",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("いらっしゃる", None),
            AssertedResult::new("いらっしゃいます", None),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);

    VerbTest::new(
        "おっしゃる",
        Some("仰る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("おっしゃる", Some("仰る")),
            AssertedResult::new("おっしゃいます", Some("仰います")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);

    VerbTest::new(
        "くださる",
        Some("下さる"),
        VerbType::Exception,
        vec![
            AssertedResult::new("くださる", Some("下さる")),
            AssertedResult::new("くださいます", Some("下さいます")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);

    VerbTest::new(
        "ござる",
        Some("御座る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("ござる", Some("御座る")),
            AssertedResult::new("ございます", Some("御座います")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);

    VerbTest::new(
        "なさる",
        Some("為さる"),
        VerbType::Exception,
        vec![
            AssertedResult::new("なさる", Some("為さる")),
            AssertedResult::new("なさいます", Some("為さいます")),
        ],
    )
    .run([
        |v: &Verb| v.dictionary(WordForm::Short),
        |v: &Verb| v.dictionary(WordForm::Long),
    ]);
}
