use rand::prelude::SliceRandom;
use crate::types::{Pattern, TextParams, TextGenerator};

impl TextGenerator {
    pub fn select_pattern(&self, index: u8, word_length: u8) -> &Pattern {
        let mut rng = rand::thread_rng();
        let positions = Pattern::valid_positions(index, word_length);
        self.patterns
            .iter()
            .filter(|x| positions.contains(&x.position()))
            .collect::<Vec<&Pattern>>()
            .choose(&mut rng)
            .unwrap()
            .to_owned()
    }

    /// Generates a random word with a specific syllable count
    pub fn rword(&self, syllables: u8) -> String {
        let mut rng = rand::thread_rng();

        (1..=syllables).map(|index| {
            self.select_pattern(index, syllables).pattern().chars().map(|element| {
                match element {
                    c if c.is_uppercase() || c.is_numeric() => self.categories
                        .get(&c.to_string())
                        .unwrap()
                        .choose(&mut rng)
                        .unwrap()
                        .clone(),
                    c if c.is_lowercase() => c.to_string(),
                    _ => panic!("Invalid character in syllable pattern: {}", element)
                }
            }).collect::<String>()
        }).collect::<Vec<_>>().concat()
    }

    /// Generates a word with a random number os syllables based on the given TextParams
    pub fn rlword(&self, params: &TextParams) -> String {
        let mut rng = rand::thread_rng();
        let syllables = params.syllables(&mut rng);
        self.rword(syllables)
    }
}
