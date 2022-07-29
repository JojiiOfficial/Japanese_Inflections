mod verb_test;

use jp_inflections::{self, Verb, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "でかける",
        Some("出かける"),
        VerbType::Ichidan,
        vec![AssertedResult::new("でかけるまい", Some("出かけるまい"))],
    )
    .run([|v: &Verb| v.negative_volitional()]);
}

#[test]
fn godan() {
    VerbTest::new(
        "ならう",
        Some("習う"),
        VerbType::Godan,
        vec![AssertedResult::new("ならうまい", Some("習うまい"))],
    )
    .run([|v: &Verb| v.negative_volitional()]);
}

#[test]
fn suru() {
    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("するまい", None)],
    )
    .run([|v: &Verb| v.negative_volitional()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにするまい", Some("耳にするまい"))],
    )
    .run([|v: &Verb| v.negative_volitional()]);
}

#[test]
fn kuru() {
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("くるまい", Some("来るまい"))],
    )
    .run([|v: &Verb| v.negative_volitional()]);
}
