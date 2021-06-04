mod verb_test;

use jp_inflections::{self, VerbType, WordForm};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![
            AssertedResult::new("たべない", Some("食べない")),
            AssertedResult::new("たべません", Some("食べません")),
        ],
    )
    .run([
        |v| v.negative(WordForm::Short),
        |v| v.negative(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもらない", Some("守らない")),
            AssertedResult::new("まもりません", Some("守りません")),
        ],
    )
    .run([
        |v| v.negative(WordForm::Short),
        |v| v.negative(WordForm::Long),
    ]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![
            AssertedResult::new("いかない", Some("行かない")),
            AssertedResult::new("いきません", Some("行きません")),
        ],
    )
    .run([
        |v| v.negative(WordForm::Short),
        |v| v.negative(WordForm::Long),
    ]);

    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("こない", Some("来ない")),
            AssertedResult::new("きません", Some("来ません")),
        ],
    )
    .run([
        |v| v.negative(WordForm::Short),
        |v| v.negative(WordForm::Long),
    ]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("しない", None),
            AssertedResult::new("しません", None),
        ],
    )
    .run([
        |v| v.negative(WordForm::Short),
        |v| v.negative(WordForm::Long),
    ]);
}
