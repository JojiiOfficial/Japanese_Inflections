mod verb_test;

use jp_inflections::VerbType;
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべたくない", Some("食べたくない"))],
    )
    .run([|v| v.negative_desiderative()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもりたくない", Some("守りたくない"))],
    )
    .run([|v| v.negative_desiderative()]);
}
