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
            AssertedResult::new("たべられない", Some("食べられない")),
            AssertedResult::new("たべられません", Some("食べられません")),
        ],
    )
    .run([
        |v| v.negative_potential(WordForm::Short),
        |v| v.negative_potential(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもれない", Some("守れない")),
            AssertedResult::new("まもれません", Some("守れません")),
        ],
    )
    .run([
        |v| v.negative_potential(WordForm::Short),
        |v| v.negative_potential(WordForm::Long),
    ]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("こられない", Some("来られない")),
            AssertedResult::new("こられません", Some("来られません")),
        ],
    )
    .run([
        |v| v.negative_potential(WordForm::Short),
        |v| v.negative_potential(WordForm::Long),
    ]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("できない", None),
            AssertedResult::new("できません", None),
        ],
    )
    .run([
        |v| v.negative_potential(WordForm::Short),
        |v| v.negative_potential(WordForm::Long),
    ]);
}
