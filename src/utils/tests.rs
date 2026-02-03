#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::generator::{
        pattern::Pattern,
        text_generator::TextGenerator,
        simple_rules::Rule
    };

    #[test]
    fn parse_generator() {
        let cats = [
            ("C".to_string(), ["p", "t", "k", "b", "d", "g", "m", "n", "s", "h"].map(String::from).to_vec()),
        ];

        let pats = [
            ["CV".to_string(), "Any".to_string(), "Default".to_string()],
        ];

        let rules = Rule::new("mp", "mb");

        let generator = TextGenerator::new(
            "testing",
            HashMap::<String, Vec<String>>::from(cats),
            pats.map(|pat| Pattern::new(&pat[0], &pat[1], &pat[2])).to_vec(),
            vec!(rules)
        );
    
        assert_eq!(
            &generator.as_json(),
            "{\"name\":\"testing\",\"categories\":{\"C\":[\"p\",\"t\",\"k\",\"b\",\"d\",\"g\",\"m\",\"n\",\"s\",\"h\"]},\"patterns\":[{\"pattern\":\"CV\",\"position\":\"Any\",\"weight\":\"Default\"}],\"ruleset\":[{\"context\":\"mp\",\"result\":\"mb\"}]}"
        );
    }

    #[test]
    fn apply_rule() {
        let rule = Rule::new(r#"cl"#, "ch");
        assert_eq!(
           rule.apply("clave"),
           "chave"
       );
    }
}

