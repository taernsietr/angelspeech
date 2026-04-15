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

impl<N> From<Vec<(N,N,N)>> for Patterns
where N: Into<String> {
    fn from(params: Vec<(N,N,N)>) -> Self {
        Patterns(
            params
                .into_iter()
                .map(|(pat,pos,wgt)| Pattern::new(pat, pos, wgt))
                .collect()
        )
    }
}

impl From<Vec<Rule>> for Ruleset {
    fn from(params: Vec<Rule>) -> Self {
        Ruleset(params)
    }
}

impl<N> From<Vec<(N,N)>> for Ruleset 
where N: Into<String> {
    fn from(params: Vec<(N,N)>) -> Self {
        Ruleset(
            params
                .into_iter()
                .map(|(ctx,rst)| Rule::new(ctx, rst))
                .collect()
        )
    }
}

