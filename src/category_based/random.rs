use std::collections::HashMap;
use rand::prelude::{RngExt, IteratorRandom, IndexedRandom};

use crate::types::{TextGenerator,Pattern, Rule, TextParams, TextType};
use crate::resources::*;

impl TextGenerator {
    // Optionally take parameters such as consonant/vowel ratio, inventory size, etc
    // This has to be rewritten more adequately
    pub fn random() -> TextGenerator {
        let mut rng = rand::rng();

        let ratio: f32 = rng.random_range(0.2..=1.0);
        let size: u8 = rng.random_range(5..=30);

        let consonants = PULMONIC_CONSONANTS
            .sample(&mut rng, ((ratio * size as f32) / (ratio + 1.0)).round().max(3.0) as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let vowels = VOWELS
            .sample(&mut rng, (size as f32 / (ratio + 1.0)).round().max(3.0) as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let no_consonant_cats = rng.random_range(1..=(consonants.len() as f32 / 3.0).round() as usize);
        let no_vowel_cats = rng.random_range(1..=(vowels.len() as f32 / 3.0).round() as usize);
        let no_pats = rng.random_range(1..5);

        let mut categories = HashMap::<String, Vec<String>>::new(); 
        let mut patterns = Vec::<Pattern>::new();

        for i in 0..=no_consonant_cats {
            let category_size = rng.random_range(1..=consonants.len());
            categories.insert(
                i.to_string(),
                consonants
                    .sample(&mut rng, category_size)
                    .map(String::from)
                    .collect::<Vec<String>>()
            );
        }
        
        for j in no_consonant_cats..=no_vowel_cats+no_consonant_cats {
            let category_size = rng.random_range(1..=vowels.len());
            categories.insert(
                j.to_string(),
                vowels
                    .sample(&mut rng, category_size)
                    .map(String::from)
                    .collect::<Vec<String>>()
            );
        }
        
        for _ in 0..=no_pats {
            let pattern_size = rng.random_range(1..=4);
            patterns.push(
                Pattern::new(
                    categories
                        .clone()
                        .into_keys()
                        .sample(&mut rng, pattern_size)
                        .iter()
                        .map(String::from)
                        .collect::<String>(),
                    "any".to_string(),
                    "default".to_string()
                )
            );
        }

        let mut temp = TextGenerator::new("temp".to_string(), categories.clone(), patterns.clone(), Vec::<Rule>::new());
        let params = TextParams::new(1, 4, 0.0, 1, TextType::GenericWord);
        let name = temp.text(&params);
        temp.set_name(name);
        temp
    }
}
