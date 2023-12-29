use crate::resources::ipa::*;

pub fn get_ipa() -> Vec<String> {
    PULMONIC_CONSONANTS.into_iter().map(String::from).collect()
}

