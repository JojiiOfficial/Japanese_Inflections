use crate::{alphabet, umlaut::Umlaut};

/// One single syllable within the a kana alphabet
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Syllable(char);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Info {
    pub row: Row,
    pub umlaut: Umlaut,
}

/// A kana row
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Row {
    /// あ,え,い,お,う
    Umlauts,
    /// ん
    NSpecial,
    K,
    G,
    S,
    Z,
    T,
    D,
    N,
    H,
    B,
    P,
    M,
    R,
    Y,
    W,
}

impl From<char> for Syllable {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl From<&str> for Syllable {
    fn from(s: &str) -> Self {
        s.chars().next().unwrap().into()
    }
}

impl Syllable {
    /// Returns `true` if the syllable ends with or equals a given Umlaut
    ///
    /// # Examples
    /// ```
    /// use jp_inflections::syllable::Syllable;
    /// use jp_inflections::umlaut::Umlaut;
    ///
    /// assert!(Syllable::from("が").ends_with(Umlaut::A));
    /// assert!(!Syllable::from('ぬ').ends_with(Umlaut::A));
    /// ```
    pub fn ends_with<U: Into<Umlaut>>(&self, u: U) -> bool {
        self.get_info()
            .map(|i| i.umlaut == u.into())
            .unwrap_or_default()
    }

    /// Returns an `Some(Info)` based on the syllable, or None if its not a valid syllable
    /// # Examples
    /// ```
    /// use jp_inflections::syllable::{Syllable, Info, Row};
    /// use jp_inflections::umlaut::Umlaut;
    ///
    /// let s = Syllable::from('が');
    /// assert_eq!(s.get_info(), Some(Info { row: Row::G, umlaut: Umlaut::A }));
    /// ```
    pub fn get_info(&self) -> Option<Info> {
        let c = self.0;

        for (row, letters) in alphabet::HIRAGANA_SYLLABLES {
            for (character, umlaut) in *letters {
                if *character == c {
                    return Some(Info {
                        umlaut: *umlaut,
                        row: *row,
                    });
                }
            }
        }

        None
    }

    /// Returns the charactor hold by [`self`]
    pub fn get_char(&self) -> char {
        self.0
    }

    /// Returns true if the syllable is a valid (hiragana) character
    pub fn is_valid(&self) -> bool {
        self.get_info().is_none()
    }
}
