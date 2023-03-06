#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Umlaut {
    A,
    E,
    I,
    O,
    U,
}

impl From<char> for Umlaut {
    fn from(value: char) -> Self {
        match value {
            'a' => Umlaut::A,
            'e' => Umlaut::E,
            'i' => Umlaut::I,
            'o' => Umlaut::O,
            'u' => Umlaut::U,
            _ => panic!("Not an umlaut"),
        }
    }
}
