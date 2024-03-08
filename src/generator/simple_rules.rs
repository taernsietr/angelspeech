use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
//    context: regex::Regex,
    context: String,
    result: String
}

impl Rule {
    pub fn new<N: Into<String>>(ctx: N, res: N) -> Rule {
        // let context = regex::Regex::new(ctx).unwrap();
        let context = ctx.into();
        let result = res.into();
        Rule { context, result }
    }

    pub fn apply(&self, input: &str) -> String {
        let context = regex::Regex::new(&self.context).unwrap();
        context.replacen(input, 0, &self.result).to_string()
//        self.context.replacen(input, 0, &self.result).to_string()
    }
}

