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
            AssertedResult::new("たべた", Some("食べた")),
            AssertedResult::new("たべました", Some("食べました")),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもった", Some("守った")),
            AssertedResult::new("まもりました", Some("守りました")),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![
            AssertedResult::new("いった", Some("行った")),
            AssertedResult::new("いきました", Some("行きました")),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);

    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("きた", Some("来た")),
            AssertedResult::new("きました", Some("来ました")),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("した", None),
            AssertedResult::new("しました", None),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);

    VerbTest::new(
        "いらっしゃる",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("いらっしゃった", None),
            AssertedResult::new("いらっしゃいました", None),
        ],
    )
    .run([|v| v.past(WordForm::Short), |v| v.past(WordForm::Long)]);
}
