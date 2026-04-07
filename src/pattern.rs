use std::str::FromStr;
use crate::types::{
    Pattern,
    PatternPosition,
    PatternWeight
};

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

    pub fn valid_positions(index: u8, word_length: u8) -> Vec<PatternPosition> {
        /*  index  = 1     && len  =  1 -> any, non-medial
            index  = 1     && len >=  2 -> any, initial, non-final
            index  = len   && len  >  1 -> any, non-initial, final
        1 < index <= len-1 && len  >  2 -> any, non-final, medial, non-initial */
        match (index, word_length) {
            (1, 1) => {
                vec!(PatternPosition::Any, PatternPosition::NonMedial)
            },
            (1, 2..) => {
                vec!(PatternPosition::Any, PatternPosition::Initial, PatternPosition::NonMedial, PatternPosition::NonFinal)
            },
            (index, 2..) if index == word_length => {
                vec!(PatternPosition::Any, PatternPosition::NonInitial, PatternPosition::NonMedial, PatternPosition::Final)
            },
            (index, 3..) if 1 < index && index < word_length => {
                vec!(PatternPosition::Any, PatternPosition::NonFinal, PatternPosition::Medial, PatternPosition::NonInitial)
            },
            _ => unreachable!(),
        }
    }
}
