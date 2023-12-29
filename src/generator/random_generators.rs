use std::collections::HashMap;
use rand::Rng;
use rand::prelude::{IteratorRandom, SliceRandom};

use crate::generator::{
    text_generator::TextGenerator,
    pattern::Pattern
};

use crate::resources::ipa::{
    PULMONIC_CONSONANTS,
    VOWELS
};

impl TextGenerator {

    // Optionally take parameters such as consonant/vowel ratio, inventory size, etc
    // This has to be rewritten more adequately
    pub fn random() -> TextGenerator {
        let mut rng = rand::thread_rng();

        let ratio: f32 = rng.gen_range(0.2..=1.0);
        let size: u8 = rng.gen_range(5..=30);

        let consonants = PULMONIC_CONSONANTS
            .choose_multiple(&mut rng, ((ratio * size as f32) / (ratio + 1.0)).round() as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let vowels = VOWELS
            .choose_multiple(&mut rng, (size as f32 / (ratio + 1.0)).round() as usize)
            .map(|x| x.to_owned().to_string()).collect::<Vec<String>>();

        let no_consonant_cats = rng.gen_range(1..=(consonants.len() as f32 / 2.0).round() as usize);
        let no_vowel_cats = rng.gen_range(1..=(vowels.len() as f32 / 2.0).round() as usize);
        let no_pats = rng.gen_range(1..5);

        let mut categories = HashMap::<String, Vec<String>>::new(); 
        let mut patterns = Vec::<Pattern>::new();

        for i in 0..=no_consonant_cats {
            let category_size = rng.gen_range(1..=consonants.len());
            categories.insert(
                i.to_string(),
                consonants
                    .choose_multiple(&mut rng, category_size)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            );
        }
        
        for j in no_consonant_cats..=no_vowel_cats+no_consonant_cats {
            let category_size = rng.gen_range(1..=vowels.len());
            categories.insert(
                j.to_string(),
                vowels
                    .choose_multiple(&mut rng, category_size)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            );
        }
        
        for _ in 0..=no_pats {
            let pattern_size = rng.gen_range(1..=4);
            patterns.push(
                Pattern::new(
                    categories
                        .clone()
                        .into_keys()
                        .choose_multiple(&mut rng, pattern_size)
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<String>(),
                    "any".to_string(),
                    "default".to_string()
                )
            );
        }

        let temp = TextGenerator::new("temp".to_string(), categories.clone(), patterns.clone());
        TextGenerator::new(temp.random_length_word(1, 4, 0.0), categories, patterns)
    }
}
