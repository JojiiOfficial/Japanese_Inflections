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
            AssertedResult::new("たべよう", Some("食べよう")),
            AssertedResult::new("たべましょう", Some("食べましょう")),
        ],
    )
    .run([
        |v| v.volitional(WordForm::Short),
        |v| v.volitional(WordForm::Long),
    ]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![
            AssertedResult::new("まもろう", Some("守ろう")),
            AssertedResult::new("まもりましょう", Some("守りましょう")),
        ],
    )
    .run([
        |v| v.volitional(WordForm::Short),
        |v| v.volitional(WordForm::Long),
    ]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![
            AssertedResult::new("こよう", Some("来よう")),
            AssertedResult::new("きましょう", Some("来ましょう")),
        ],
    )
    .run([
        |v| v.volitional(WordForm::Short),
        |v| v.volitional(WordForm::Long),
    ]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![
            AssertedResult::new("しよう", None),
            AssertedResult::new("しましょう", None),
        ],
    )
    .run([
        |v| v.volitional(WordForm::Short),
        |v| v.volitional(WordForm::Long),
    ]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![
            AssertedResult::new("みみにしよう", Some("耳にしよう")),
            AssertedResult::new("みみにしましょう", Some("耳にしましょう")),
        ],
    )
    .run([
        |v| v.volitional(WordForm::Short),
        |v| v.volitional(WordForm::Long),
    ]);
}
