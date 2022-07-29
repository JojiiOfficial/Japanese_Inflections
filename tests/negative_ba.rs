mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべなければ", Some("食べなければ"))],
    )
    .run([|v| v.negative_ba()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもらなければ", Some("守らなければ"))],
    )
    .run([|v| v.negative_ba()]);

    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いかなければ", Some("行かなければ"))],
    )
    .run([|v| v.negative_ba()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こなければ", Some("来なければ"))],
    )
    .run([|v| v.negative_ba()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("しなければ", None)],
    )
    .run([|v| v.negative_ba()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new(
            "みみにしなければ",
            Some("耳にしなければ"),
        )],
    )
    .run([|v| v.negative_ba()]);
}
