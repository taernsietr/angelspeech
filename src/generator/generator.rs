use std::collections::HashMap;
use crate::types::{Pattern, Rule, TextGenerator};

#[allow(dead_code)]
impl TextGenerator {
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn as_json(&self) -> String { serde_json::to_string(&self).unwrap() }

    pub fn new<N: Into<String>>(
        name: N,
        categories: HashMap<String, Vec<String>>,
        patterns: Vec<Pattern>,
        ruleset: Vec<Rule>
    ) -> TextGenerator {
        TextGenerator { name: name.into(), categories, patterns, ruleset }
    }
    
    pub fn new_empty<N: Into<String>>(name: N) -> TextGenerator {
        TextGenerator {
            name: name.into(), 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
            ruleset: Vec::<Rule>::new()
        }
    }
}

