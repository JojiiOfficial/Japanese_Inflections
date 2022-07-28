mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべさせない", Some("食べさせない"))],
    )
    .run([|v| v.negative_causative()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもらせない", Some("守らせない"))],
    )
    .run([|v| v.negative_causative()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こさせない", Some("来させない"))],
    )
    .run([|v| v.negative_causative()]);

    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("させない", None)],
    )
    .run([|v| v.negative_causative()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにさせない", Some("耳にさせない"))],
    )
    .run([|v| v.negative_causative()]);
}
