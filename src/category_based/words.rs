use rand::{RngExt, prelude::IndexedRandom};
use crate::types::{Pattern, TextParams, TextGenerator};

impl TextGenerator {
    pub fn select_pattern(&self, index: u8, word_length: u8) -> &Pattern {
        let mut rng = rand::rng();
        let positions = Pattern::valid_positions(index, word_length);
        self.patterns
            .iter()
            .filter(|x| positions.contains(&x.position()))
            .collect::<Vec<&Pattern>>()
            .choose(&mut rng)
            .unwrap()
            .to_owned()
    }

    // TODO: rewrite aligning the syllable patterns first; should be easier to apply phonetic rules
    /// Generates a random word with a specific syllable count
    pub fn word(&self, syllables: u8) -> String {
        let mut rng = rand::rng();

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

    /// Generates text with a fixed size, where words have a semirandom 
    /// number of syllables based on the given TextParams
    pub fn text(&self, params: &TextParams) -> String {
        std::iter::repeat_with(|| { self.word(params.syllables()) })
            .take(params.text_size.into())
            .collect::<Vec<String>>()
            .join(" ")
    }

    // TODO: configurable pseudotext
    /// Generates text with some recognizable structure, for simulating real text with generated
    /// words
    pub fn pseudotext(&self, params: &TextParams) -> String {
        let mut rng = rand::rng();
        let mut pseudotext = String::new();
        let roots: Vec<String> = (0..Ord::min(params.text_size/2, 12)).map(|_| self.text(params)).collect();
        let particles: Vec<String> = (0..Ord::min(params.text_size, 10)).map(|_| self.word(Ord::min(params.min_syllables, 2))).collect();
      //let morpheme: Vec<String> = (0..Ord::max(length, 12)).map(|_| self.random_word(1)).collect();

        let mut last_type = "none"; 
        for i in 0..=params.text_size {
            let state = rng.random_range(0..2);
            pseudotext.push_str(
                match (last_type, state) {
                    ("none", _) => { last_type = "root"; roots.choose(&mut rng).unwrap() },
                    ("root", _) => { last_type = "part"; particles.choose(&mut rng).unwrap() },
                  //("none", 1) => { last_type = "root"; roots.choose(&mut rng).unwrap().clone() (morpheme.choose(&mut rng).unwrap()) },
                    ("part", 0) => { last_type = "root"; roots.choose(&mut rng).unwrap() },
                    ("part", 1) => { last_type = "part"; particles.choose(&mut rng).unwrap() },
                    (_, _) => unreachable!()
                }
            );
            if rng.random_bool((i as f64 % 8.0) / 8.0) { pseudotext.push_str([", ", ". ", "? ", "! "].choose(&mut rng).unwrap()) }
            else { pseudotext.push(' ') };
        };
        pseudotext
    }
}
