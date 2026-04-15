use std::collections::HashMap;
use crate::types::*;

impl Categories {
    pub fn empty() -> Self {
        Categories(HashMap::new())
    }
}

impl Patterns {
    pub fn empty() -> Self {
        Patterns(Vec::new())
    }
}

impl Ruleset {
    pub fn empty() -> Self {
        Ruleset(Vec::new())
    }
}

