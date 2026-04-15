use std::collections::HashMap;
use std::convert::Into;
use crate::types::*;

impl From<HashMap<String,Vec<String>>> for Categories {
    fn from(params: HashMap<String,Vec<String>>) -> Self {
       Categories(params)
    }
}

impl<N> From<Vec<(N, Vec<N>)>> for Categories
where N: Into<String> {
    fn from(params: Vec<(N, Vec<N>)>) -> Self {
        Categories(
        params
            .into_iter()
            .map(|(k,v)| (
                k.into(),
                v.into_iter().map(Into::into).collect::<Vec<String>>()
            ))
            .collect()
        )
    }
}


impl From<Vec<Pattern>> for Patterns {
    fn from(params: Vec<Pattern>) -> Self {
        Patterns(params)
    }
}

impl From<Vec<Rule>> for Ruleset {
    fn from(params: Vec<Rule>) -> Self {
        Ruleset(params)
    }
}

