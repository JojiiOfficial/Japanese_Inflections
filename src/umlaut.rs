#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Umlaut {
    A,
    E,
    I,
    O,
    U,
}

impl Into<Umlaut> for char {
    fn into(self) -> Umlaut {
        match self {
            'a' => Umlaut::A,
            'e' => Umlaut::E,
            'i' => Umlaut::I,
            'o' => Umlaut::O,
            'u' => Umlaut::U,
            _ => panic!("Not an umlaut"),
        }
    }
}
