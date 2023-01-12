#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inflection {
    Stem,
    StemPotential,
    StemBa,
    Positive,
    Negative,
    Past,
    Present,
    Polite,
    Te,
    Passive,
    Causative,
    CausativePassive,
    Imperative,
}
