mod verb_test;

use jp_inflections::VerbType;
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべたい", Some("食べたい"))],
    )
    .run([|v| v.desiderative()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもりたい", Some("守りたい"))],
    )
    .run([|v| v.desiderative()]);
}
