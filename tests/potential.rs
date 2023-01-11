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
            AssertedResult::new("たべられる", Some("食べられる")),
            AssertedResult::new("たべられます", Some("食べられます")),
        ],
    )
    .run([
        |v| v.potential(WordForm::Short),
        |v| v.potential(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもれる", Some("守れる")),
            AssertedResult::new("まもれます", Some("守れます")),
        ],
    )
    .run([
        |v| v.potential(WordForm::Short),
        |v| v.potential(WordForm::Long),
    ]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("こられる", Some("来られる")),
            AssertedResult::new("こられます", Some("来られます")),
        ],
    )
    .run([
        |v| v.potential(WordForm::Short),
        |v| v.potential(WordForm::Long),
    ]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("できる", None),
            AssertedResult::new("できます", None),
        ],
    )
    .run([
        |v| v.potential(WordForm::Short),
        |v| v.potential(WordForm::Long),
    ]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![
            AssertedResult::new("みみにできる", Some("耳にできる")),
            AssertedResult::new("みみにできます", Some("耳にできます")),
        ],
    )
    .run([
        |v| v.potential(WordForm::Short),
        |v| v.potential(WordForm::Long),
    ]);
}
