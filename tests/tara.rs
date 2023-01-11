mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべたら", Some("食べたら"))],
    )
    .run([|v| v.tara()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもったら", Some("守ったら"))],
    )
    .run([|v| v.tara()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いったら", Some("行ったら"))],
    )
    .run([|v| v.tara()]);

    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("きたら", Some("来たら"))],
    )
    .run([|v| v.tara()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("したら", None)],
    )
    .run([|v| v.tara()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにしたら", Some("耳にしたら"))],
    )
    .run([|v| v.tara()]);

    VerbTest::new(
        "いらっしゃる",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("いらっしゃったら", None)],
    )
    .run([|v| v.tara()]);
}
