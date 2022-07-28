mod verb_test;

use jp_inflections::{self, VerbType};
use verb_test::{AssertedResult, VerbTest};

#[test]
fn ichidan() {
    VerbTest::new(
        "たべる",
        Some("食べる"),
        VerbType::Ichidan,
        vec![AssertedResult::new("たべて", Some("食べて"))],
    )
    .run([|v| v.te_form()]);
}

#[test]
fn godan_ru() {
    VerbTest::new(
        "まもる",
        Some("守る"),
        VerbType::Godan,
        vec![AssertedResult::new("まもって", Some("守って"))],
    )
    .run([|v| v.te_form()]);
}

#[test]
fn godan_su() {
    VerbTest::new(
        "はなす",
        Some("話す"),
        VerbType::Godan,
        vec![AssertedResult::new("はなして", Some("話して"))],
    )
    .run([|v| v.te_form()]);
}

#[test]
fn godan_ku() {
    VerbTest::new(
        "かく",
        Some("書く"),
        VerbType::Godan,
        vec![AssertedResult::new("かいて", Some("書いて"))],
    )
    .run([|v| v.te_form()]);
}

#[test]
fn godan_gu() {
    VerbTest::new(
        "およぐ",
        Some("泳ぐ"),
        VerbType::Godan,
        vec![AssertedResult::new("およいで", Some("泳いで"))],
    )
    .run([|v| v.te_form()]);
}

#[test]
fn exceptions() {
    // 行く
    VerbTest::new(
        "いく",
        Some("行く"),
        VerbType::Exception,
        vec![AssertedResult::new("いって", Some("行って"))],
    )
    .run([|v| v.te_form()]);

    // する
    VerbTest::new(
        "する",
        None,
        VerbType::Exception,
        vec![AssertedResult::new("して", None)],
    )
    .run([|v| v.te_form()]);

    VerbTest::new(
        "みみにする",
        Some("耳にする"),
        VerbType::Exception,
        vec![AssertedResult::new("みみにして", Some("耳にして"))],
    )
    .run([|v| v.te_form()]);

    // 来る
    VerbTest::new(
        "くる",
        Some("来る"),
        VerbType::Exception,
        vec![AssertedResult::new("きて", Some("来て"))],
    )
    .run([|v| v.te_form()]);

    // いらっしゃる
    VerbTest::new(
        "いらっしゃる",
        None,
        VerbType::Godan,
        vec![AssertedResult::new("いらして", None)],
    )
    .run([|v| v.te_form()]);
}
