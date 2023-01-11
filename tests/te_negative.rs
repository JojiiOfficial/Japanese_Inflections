mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべなくて", Some("食べなくて"))],
    )
    .run([|v| v.negative_te_form()]);
}

#[test]
fn godan_ru() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもらなくて", Some("守らなくて"))],
    )
    .run([|v| v.negative_te_form()]);
}

#[test]
fn godan_su() {
    VerbTest::new(
        "はなす",
        Some("話す"),
        VerbType::Godan,
        vec![AssertedResult::new("はなさなくて", Some("話さなくて"))],
    )
    .run([|v| v.negative_te_form()]);
}

#[test]
fn godan_ku() {
    VerbTest::new(
        "かく",
        Some("書く"),
        VerbType::Godan,
        vec![AssertedResult::new("かかなくて", Some("書かなくて"))],
    )
    .run([|v| v.negative_te_form()]);
}

#[test]
fn godan_gu() {
    VerbTest::new(
        "およぐ",
        Some("泳ぐ"),
        VerbType::Godan,
        vec![AssertedResult::new("およがなくて", Some("泳がなくて"))],
    )
    .run([|v| v.negative_te_form()]);
}

#[test]
fn exceptions() {
    // 行く
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いかなくて", Some("行かなくて"))],
    )
    .run([|v| v.negative_te_form()]);

    // する
    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("しなくて", None)],
    )
    .run([|v| v.negative_te_form()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにしなくて", Some("耳にしなくて"))],
    )
    .run([|v| v.negative_te_form()]);

    // 来る
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("こなくて", Some("来なくて"))],
    )
    .run([|v| v.negative_te_form()]);
}
