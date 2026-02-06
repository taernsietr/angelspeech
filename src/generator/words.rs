use std::cmp::Ordering;
use rand::{Rng, prelude::SliceRandom};
use crate::types::{Pattern, TextGenerator};

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

    pub fn random_word(&self, word_length: u8) -> String {
        let mut rng = rand::thread_rng();
        (1..=word_length).map(|index| {
            self.select_pattern(index, word_length).pattern().chars().map(|element| {
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

    pub fn random_length_word(&self, min_syllables: u8, max_syllables: u8, bias: f32) -> String {
        let mut rng = rand::thread_rng();

        let word_length = match min_syllables.cmp(&max_syllables) {
            Ordering::Less => {

                // used logic from https://stackoverflow.com/questions/29325069/how-to-generate-random-numbers-biased-towards-one-value-in-a-range
                let mut bias_t = (max_syllables - min_syllables) / 2;
                if bias > 0.0 { bias_t = max_syllables };
                if bias < 0.0 { bias_t = min_syllables };
                let mix = bias.abs() * rng.gen::<f32>();

                (rng.gen_range({
                    if min_syllables > 0 { min_syllables }
                    else { 1 }
                }..=max_syllables) as f32 * (1.0 - mix) + bias_t as f32 * mix) as u8
            },
            Ordering::Equal => {
                println!("[Angelspeech] Warning: Minimum syllable value has to be greater than zero. Defaulting to 1.");
                if min_syllables > 0 { min_syllables } else { 1 }
            },
            Ordering::Greater => {
                println!("[Angelspeech] Warning: Minimum syllable value has to be equal to or less than maximum syllables. Defaulting to the minimum given value ({}).",
                &min_syllables);
                min_syllables
            },
        };
        self.random_word(word_length)
    }
}
