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
            AssertedResult::new("たべなかった", Some("食べなかった")),
            AssertedResult::new("たべませんでした", Some("食べませんでした")),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもらなかった", Some("守らなかった")),
            AssertedResult::new("まもりませんでした", Some("守りませんでした")),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![
            AssertedResult::new("いかなかった", Some("行かなかった")),
            AssertedResult::new("いきませんでした", Some("行きませんでした")),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);

    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("こなかった", Some("来なかった")),
            AssertedResult::new("きませんでした", Some("来ませんでした")),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("しなかった", None),
            AssertedResult::new("しませんでした", None),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![
            AssertedResult::new("みみにしなかった", Some("耳にしなかった")),
            AssertedResult::new("みみにしませんでした", Some("耳にしませんでした")),
        ],
    )
    .run([
        |v| v.negative_past(WordForm::Short),
        |v| v.negative_past(WordForm::Long),
    ]);
}
