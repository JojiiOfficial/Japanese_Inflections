mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべさせる", Some("食べさせる"))],
    )
    .run([|v| v.causative()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもらせる", Some("守らせる"))],
    )
    .run([|v| v.causative()]);
}

#[test]
fn exceptions() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こさせる", Some("来させる"))],
    )
    .run([|v| v.causative()]);

    VerbTest::new(
        "する",
        Some("為る"),
        VerbType::Exception,
        vec![AssertedResult::new("させる", Some("為せる"))],
    )
    .run([|v| v.causative()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにさせる", Some("耳にさせる"))],
    )
    .run([|v| v.causative()]);
}
