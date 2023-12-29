use std::cmp::Ordering;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::generator::pattern::{Pattern, PatternPosition};

#[derive(Deserialize, Serialize, Debug)]
pub struct TextGenerator {
    pub name: String,
    pub categories: HashMap<String,Vec<String>>,
    pub patterns: Vec<Pattern>,
}

#[allow(dead_code)]
impl TextGenerator {
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn as_json(&self) -> String { serde_json::to_string(&self).unwrap() }

    pub fn new(
        name: String,
        categories: HashMap<String,Vec<String>>,
        patterns: Vec<Pattern>
    ) -> TextGenerator {

        TextGenerator { name, categories, patterns }
    }
    
    pub fn new_empty(name: String) -> TextGenerator {
        TextGenerator {
            name, 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
        }
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

    pub fn random_word(&self, word_length: u8) -> String {
        let mut rng = rand::thread_rng();
        let mut word = Vec::<String>::new();

        // Each syllable
        for index in 1..=word_length {
        /*  index  = 1     && len  =  1 -> any, non-medial
            index  = 1     && len >=  2 -> any, initial, non-final
            index  = len   && len  >  1 -> any, non-initial, final
        1 < index <= len-1 && len  >  2 -> any, non-final, medial, non-initial */
            let position_filters: Vec<PatternPosition> = match (index, word_length) {
                (1, 1) => {                                            vec!(PatternPosition::Any, PatternPosition::NonMedial)},
                (1, 2..) => {                                          vec!(PatternPosition::Any, PatternPosition::Initial, PatternPosition::NonMedial, PatternPosition::NonFinal)},
                (index, 2..) if index == word_length => {              vec!(PatternPosition::Any, PatternPosition::NonInitial, PatternPosition::NonMedial, PatternPosition::Final)},
                (index, 3..) if 1 < index && index < word_length => {  vec!(PatternPosition::Any, PatternPosition::NonFinal, PatternPosition::Medial, PatternPosition::NonInitial)},
                _ => unreachable!(),
            };

            /*
            // TODO: move this to a more adequate file
            pub enum WordMoraPattern {
                Any,
                Alternating,
                AlternatingDoubleHeavy,
                SingleHeavy,
                DoubleHeavy,
            }
            */

            /* 
            // TODO: Decide if implementing this is worth it
            let weight_filters: Vec<PatternWeight> = match (current_weight, word_mora_pattern) {
                ("L" || "H", WordMoraPattern::Any) => {},
                ("L", WordMoraPattern::Alternating) => {},
                ("H", WordMoraPattern::Alternating) => {},

            }
            */
            
            let syllable_pattern = &self.patterns
                .iter()
                .filter(|x| position_filters.contains(&x.position()))
                .collect::<Vec<&Pattern>>()
                .choose(&mut rng)
                .unwrap()
                .to_owned();

            for element in syllable_pattern.pattern().chars() {
                if element.is_uppercase() || element.is_numeric() { 
                    word.push(
                        self.categories
                            .get(&element.to_string())
                            .unwrap()
                            .choose(&mut rng)
                            .unwrap()
                            .clone()
                    );
                }
                else if element.is_lowercase() { 
                    word.push(element.to_string());
                }
                else {
                    panic!("Invalid character in syllable pattern: {}", element);
                }
            }
        }
        word.concat()
    }

    pub fn random_text(&self, min_syllables: u8, max_syllables: u8, bias: f32, text_size: u8) -> String {
        let text_size = if text_size < 1 {
            println!("[Angelspeech] Warning: text size must be at least 1. Defaulting to 1.");
            1
        }
        else { text_size };

        let mut text = Vec::<String>::new();

        for _ in 0..text_size {
            text.push(self.random_length_word(min_syllables, max_syllables, bias));
        }
        text.join(" ")
    }

    // TODO: configurable pseudotext, ie "command" gives a certain pattern of output different than
    // "wikipedia article"
    pub fn pseudotext(&self, min_syllables: u8, max_syllables: u8, bias: f32, text_size: u8) -> String {
        let mut rng = rand::thread_rng();
        let mut pseudotext = String::new();
        let roots: Vec<String> = (0..Ord::min(text_size/2, 12)).map(|_| self.random_length_word(min_syllables, max_syllables, bias)).collect();
        let particles: Vec<String> = (0..Ord::min(text_size, 10)).map(|_| self.random_word(Ord::min(min_syllables, 2))).collect();
      //let morpheme: Vec<String> = (0..Ord::max(length, 12)).map(|_| self.random_word(1)).collect();

        let mut last_type = "none"; 
        for i in 0..=text_size {
            let state = rng.gen_range(0..2);
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
            if rng.gen_bool((i as f64 % 8.0) / 8.0) { pseudotext.push_str([", ", ". ", "? ", "! "].choose(&mut rng).unwrap()) }
            else { pseudotext.push(' ') };
        };
        pseudotext
    }
}
