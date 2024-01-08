use std::str::FromStr;
use rand::{Rng, distributions::{Distribution, Standard}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternPosition {
    Any,
    Initial,
    Medial,
    Final,
    NonInitial,
    NonMedial,
    NonFinal,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum PatternWeight {
    Default,
    Light,
    Heavy,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pattern {
    pattern: String,
    position: PatternPosition,
    weight: PatternWeight,
}

impl FromStr for PatternPosition {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternPosition, Self::Err> {

        match input {
            "Any" | "any" => Ok(PatternPosition::Any),
            "Initial" | "initial" => Ok(PatternPosition::Initial),
            "Medial" | "medial" => Ok(PatternPosition::Medial),
            "Final" | "final" => Ok(PatternPosition::Final),
            "NonInitial" | "noninitial" => Ok(PatternPosition::NonInitial),
            "NonMedial" | "nonmedial" => Ok(PatternPosition::NonMedial),
            "NonFinal" | "nonfinal" => Ok(PatternPosition::NonFinal),
            _ => Ok(PatternPosition::Any),
        }
    }
}

impl FromStr for PatternWeight {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternWeight, Self::Err> {

        match input {
            "Default" | "default" => Ok(PatternWeight::Default),
            "Light" | "light" => Ok(PatternWeight::Light),
            "Heavy" | "heavy" => Ok(PatternWeight::Heavy),
            _ => Ok(PatternWeight::Default),
        }
    }
}

impl Distribution<PatternWeight> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PatternWeight {
        match rng.gen_range(1..100) {
            1..=70 => PatternWeight::Default,
            71..=85 => PatternWeight::Light,
            86..=100 => PatternWeight::Heavy,
            _ => unreachable!()
        }
    }
}

impl Distribution<PatternPosition> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PatternPosition {
        match rng.gen_range(1..100) {
            1..=70 => PatternPosition::Any,
            71..=75 => PatternPosition::Initial,
            76..=80 => PatternPosition::Medial,
            81..=85 => PatternPosition::Final,
            86..=90 => PatternPosition::NonInitial,
            91..=95 => PatternPosition::NonMedial,
            96..=100 => PatternPosition::NonFinal,
            _ => unreachable!()
        }
    }
}

#[allow(dead_code)]
impl Pattern {
    pub fn pattern(&self) -> String { self.pattern.clone() }
    pub fn position(&self) -> PatternPosition { self.position }
    pub fn weight(&self) -> PatternWeight { self.weight }
    pub fn new<S: Into<String>>(pattern: S, position: S, weight: S) -> Pattern {
        Pattern { 
            pattern: pattern.into(), 
            position: PatternPosition::from_str(&position.into()).unwrap(), 
            weight: PatternWeight::from_str(&weight.into()).unwrap(), 
        }
    }
}
