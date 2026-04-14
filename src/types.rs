use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct TextGenerator {
    pub name: String,
    pub categories: Categories,
    pub patterns: Vec<Pattern>,
    pub ruleset: Vec<Rule>
} 

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
pub struct Categories(pub HashMap<String, Vec<String>>);

#[derive(sqlx::Type, Deserialize, Serialize, Debug)]
pub struct Rule {
    pub context: String,
    pub result: String
}

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternPosition {
    Any,
    Initial,
    Medial,
    Final,
    NonInitial,
    NonMedial,
    NonFinal,
}

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Copy, Debug)]
pub enum PatternWeight {
    Default,
    Light,
    Heavy,
}

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
pub struct Pattern {
    pub pattern: String,
    pub position: PatternPosition,
    pub weight: PatternWeight,
}

#[derive(sqlx::FromRow)]
pub struct TextParams {
    pub min_syllables: u8, 
    pub max_syllables: u8, 
    pub bias: f32, 
    pub text_size: u8,
    pub text_type: TextType
}

#[derive(sqlx::Type, Deserialize, Clone)]
pub enum TextType {
    GenericWord,
    GenericPseudotext
}

#[derive(Debug, PartialEq, Eq)]
pub struct TextTypeParseError;

