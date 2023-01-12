pub mod alphabet;
pub mod error;
pub mod inflection;
pub mod special_verbs;
pub mod syllable;
pub mod umlaut;
pub mod verb;
pub mod word;

pub use verb::Verb;
pub use verb::VerbType;
pub use word::Word;
pub use word::WordForm;

pub type JapaneseResult<T> = Result<T, error::Error>;
