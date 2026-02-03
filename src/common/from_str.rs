use std::str::FromStr;
use crate::types::{PatternWeight,PatternPosition};

impl FromStr for PatternPosition {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternPosition, Self::Err> {

        match input.to_uppercase().as_ref() {
            "ANY" => Ok(PatternPosition::Any),
            "INITIAL" => Ok(PatternPosition::Initial),
            "MEDIAL" => Ok(PatternPosition::Medial),
            "FINAL" => Ok(PatternPosition::Final),
            "NONINITIAL" => Ok(PatternPosition::NonInitial),
            "NONMEDIAL" => Ok(PatternPosition::NonMedial),
            "NONFINAL" => Ok(PatternPosition::NonFinal),
            _ => Ok(PatternPosition::Any),
        }
    }
}

impl FromStr for PatternWeight {
    type Err = ();

    fn from_str(input: &str) -> Result<PatternWeight, Self::Err> {

        match input {
            "DEFAULT" => Ok(PatternWeight::Default),
            "LIGHT" => Ok(PatternWeight::Light),
            "HEAVY" => Ok(PatternWeight::Heavy),
            _ => Ok(PatternWeight::Default),
        }
    }
}

