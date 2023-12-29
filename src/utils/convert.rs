use regex::Regex;

use crate::resources::ipa::CONVERSION_TABLE;

#[allow(dead_code)]
pub fn escape_regex(regex: &str) -> String {
    let re = Regex::new(r"([\\/\{\}\[\]\(\)\.\*\?])").unwrap();
    re.replace_all(regex, r"\$1").to_string()
}

// Still compiles regex on every call
// .iter() returns elements in arbitrary order
pub fn xsampa_to_ipa(input: String) -> String {
    let mut result = input.clone();
    let mut regexes = Vec::<(Regex, String)>::new();
    
    for (left, right) in CONVERSION_TABLE.iter() {
        regexes.push(
            (
                Regex::new(escape_regex(left).as_str()).unwrap(),
                right.to_string()
            )
        );
    }

    for each in regexes {
        result = each.0.replace_all(&result, each.1).to_string();
    }

    result
}

pub fn ipa_to_xsampa(input: String) -> String {
    let mut result = String::new();

    for ipa in input.chars() {
        for (left, right) in CONVERSION_TABLE.iter() {
            if &ipa.to_string() == right { result.push_str(left) }
            else if ipa == ' ' { result.push(' ') }
        }
    }

    result
}
