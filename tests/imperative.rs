mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべろ", Some("食べろ"))],
    )
    .run([|v| v.imperative()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもれ", Some("守れ"))],
    )
    .run([|v| v.imperative()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こい", Some("来い"))],
    )
    .run([|v| v.imperative()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("しろ", None)],
    )
    .run([|v| v.imperative()]);

    VerbTest::new(
        "いらっしゃる",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("いらっしゃい", None)],
    )
    .run([|v| v.imperative()]);
}
