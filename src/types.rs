use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct TextGenerator {
    pub name: String,
    pub categories: HashMap<String,Vec<String>>,
    pub patterns: Vec<Pattern>,
    pub ruleset: Vec<Rule>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    pub context: String,
    pub result: String
}

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
    pub pattern: String,
    pub position: PatternPosition,
    pub weight: PatternWeight,
}

