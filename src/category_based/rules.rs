use crate::types::Rule;

impl Rule {
    pub fn new<N: Into<String>>(ctx: N, res: N) -> Rule {
        let context = ctx.into();
        let result = res.into();
        Rule { context, result }
    }

    pub fn apply(&self, input: &str) -> String {
        let context = regex::Regex::new(&self.context).unwrap();
        context.replacen(input, 0, &self.result).to_string()
    }
}

