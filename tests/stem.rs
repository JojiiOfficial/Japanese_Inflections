mod verb_test;

use jp_inflections::{self, VerbType, WordForm};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn exceptions() {
    // 行く
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いき", Some("行き"))],
    )
    .run([|v| v.get_stem(WordForm::Long)]);

    // する
    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("し", None)],
    )
    .run([|v| v.get_stem(WordForm::Long)]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにし", Some("耳にし"))],
    )
    .run([|v| v.get_stem(WordForm::Long)]);

    // 来る
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("き", Some("来"))],
    )
    .run([|v| v.get_stem(WordForm::Long)]);

    VerbTest::new(
        "あそびにくる",
        Some("遊びに来る"),
        VerbType::Exception,
        vec![AssertedResult::new("あそびにき", Some("遊びに来"))],
    )
    .run([|v| v.get_stem(WordForm::Long)]);
}
