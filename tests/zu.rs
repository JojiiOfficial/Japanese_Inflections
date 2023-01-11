mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべず", Some("食べず"))],
    )
    .run([|v| v.zu()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもらず", Some("守らず"))],
    )
    .run([|v| v.zu()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こず", Some("来ず"))],
    )
    .run([|v| v.zu()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("せず", None)],
    )
    .run([|v| v.zu()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにせず", Some("耳にせず"))],
    )
    .run([|v| v.zu()]);
}
