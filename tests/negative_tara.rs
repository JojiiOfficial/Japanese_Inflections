mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new(
            "たべなかったら",
            Some("食べなかったら"),
        )],
    )
    .run([|v| v.negative_tara()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new(
            "まもらなかったら",
            Some("守らなかったら"),
        )],
    )
    .run([|v| v.negative_tara()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こなかったら", Some("来なかったら"))],
    )
    .run([|v| v.negative_tara()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("しなかったら", None)],
    )
    .run([|v| v.negative_tara()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new(
            "みみにしなかったら",
            Some("耳にしなかったら"),
        )],
    )
    .run([|v| v.negative_tara()]);
}
