mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべれば", Some("食べれば"))],
    )
    .run([|v| v.ba()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもれば", Some("守れば"))],
    )
    .run([|v| v.ba()]);

    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いけば", Some("行けば"))],
    )
    .run([|v| v.ba()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("くれば", Some("来れば"))],
    )
    .run([|v| v.ba()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("すれば", None)],
    )
    .run([|v| v.ba()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにすれば", Some("耳にすれば"))],
    )
    .run([|v| v.ba()]);
}
