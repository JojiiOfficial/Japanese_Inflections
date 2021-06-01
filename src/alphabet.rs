use crate::{syllable::Row, umlaut::Umlaut};

/// All (single) hiragana syllables
pub const HIRAGANA_SYLLABLES: &[(Row, &[(char, Umlaut)])] = &[
    (
        Row::Umlauts,
        &[
            ('あ', Umlaut::A),
            ('え', Umlaut::E),
            ('い', Umlaut::I),
            ('お', Umlaut::O),
            ('う', Umlaut::U),
        ],
    ),
    (
        Row::K,
        &[
            ('か', Umlaut::A),
            ('け', Umlaut::E),
            ('き', Umlaut::I),
            ('こ', Umlaut::O),
            ('く', Umlaut::U),
        ],
    ),
    (
        Row::G,
        &[
            ('が', Umlaut::A),
            ('げ', Umlaut::E),
            ('ぎ', Umlaut::I),
            ('ご', Umlaut::O),
            ('ぐ', Umlaut::U),
        ],
    ),
    (
        Row::S,
        &[
            ('さ', Umlaut::A),
            ('せ', Umlaut::E),
            ('し', Umlaut::I),
            ('そ', Umlaut::O),
            ('す', Umlaut::U),
        ],
    ),
    (
        Row::Z,
        &[
            ('ざ', Umlaut::A),
            ('ぜ', Umlaut::E),
            ('じ', Umlaut::I),
            ('ぞ', Umlaut::O),
            ('ず', Umlaut::U),
        ],
    ),
    (
        Row::T,
        &[
            ('た', Umlaut::A),
            ('て', Umlaut::E),
            ('ち', Umlaut::I),
            ('と', Umlaut::O),
            ('つ', Umlaut::U),
        ],
    ),
    (
        Row::D,
        &[
            ('だ', Umlaut::A),
            ('で', Umlaut::E),
            ('ぢ', Umlaut::I),
            ('ど', Umlaut::O),
            ('づ', Umlaut::U),
        ],
    ),
    (
        Row::N,
        &[
            ('な', Umlaut::A),
            ('ね', Umlaut::E),
            ('に', Umlaut::I),
            ('の', Umlaut::O),
            ('ぬ', Umlaut::U),
        ],
    ),
    (
        Row::H,
        &[
            ('は', Umlaut::A),
            ('へ', Umlaut::E),
            ('ひ', Umlaut::I),
            ('ほ', Umlaut::O),
            ('ふ', Umlaut::U),
        ],
    ),
    (
        Row::B,
        &[
            ('ば', Umlaut::A),
            ('べ', Umlaut::E),
            ('び', Umlaut::I),
            ('ぼ', Umlaut::O),
            ('ぶ', Umlaut::U),
        ],
    ),
    (
        Row::P,
        &[
            ('ぱ', Umlaut::A),
            ('ぺ', Umlaut::E),
            ('ぴ', Umlaut::I),
            ('ぽ', Umlaut::O),
            ('ぷ', Umlaut::U),
        ],
    ),
    (
        Row::M,
        &[
            ('ま', Umlaut::A),
            ('め', Umlaut::E),
            ('み', Umlaut::I),
            ('も', Umlaut::O),
            ('む', Umlaut::U),
        ],
    ),
    (
        Row::R,
        &[
            ('ら', Umlaut::A),
            ('れ', Umlaut::E),
            ('り', Umlaut::I),
            ('ろ', Umlaut::O),
            ('る', Umlaut::U),
        ],
    ),
    (
        Row::Y,
        &[('や', Umlaut::A), ('よ', Umlaut::O), ('ゆ', Umlaut::U)],
    ),
    (Row::W, &[('わ', Umlaut::A), ('わ', Umlaut::O)]),
];
