use std::collections::HashMap;
use rand::{Rng, prelude::{IteratorRandom, SliceRandom}};

use crate::types::{TextGenerator,Pattern, Rule};

// pub const CONSONANTS: [&str; 63] = ["p", "b", "t", "d", "t`", "d`", "c", r"J\\", "k", "g", "q", r"G\\", "?", "m", "F", "n", "n`", "J", "N", r"N\\", r"B\\", "r", r"R\\", "4", "r`", r"p\\", "B", "f", "v", "T", "D", "s", "z", "S", "Z", "s`", "z`", "C", r"j\\", "x", "G", "X", "R", r"X\\", r"?\\", r"h\\", "K", r"K\\", r"v\\", r"r\\", "r\\`", "j", r"M\\", "l", "l`", "L", r"L\\", "W", "w", "H", r"s\\", r"z\\", r"x\\"];
// pub const VOWELS: [&str; 34] = ["i", "y", "1", "}", "M", "u", "I", "Y", r"I\\", r"U\\", "U", "e", "2", r"@\\", "8", "7", "o", "e_o", "2_o", "@", "o_o", "E", "9", "3", r"3\\", "V", "O", "{", "6", "a", "&", "a_", "A", "Q"];
pub const CONVERSION_TABLE: [(&str, &str); 103] = [("h\\", "ɦ"), ("j\\", "ʝ"), ("l\\", "ɺ"), ("p\\", "ɸ"), ("r\\`", "ɻ"), ("r\\", "ɹ"), ("s\\", "ɕ"), ("x\\", "ɧ"), ("z\\", "ʑ"), ("B\\", "ʙ"), ("G\\_<", "ʛ"), ("G\\", "ɢ"), ("H\\", "ʜ"), ("J\\_<", "ʄ"), ("J\\", "ɟ"), ("K\\", "ɮ"), ("L\\", "ʟ"), ("N\\", "ɴ"), ("M\\", "ɰ"), ("R\\", "ʀ"), ("X\\", "ħ"), ("@\\", "ɚ"), ("3\\", "ɞ"), ("?\\", "ʕ"), ("<\\", "ʢ"), (">\\", "ʡ"), ("b_<", "ɓ"), ("g_<", "ɠ"), ("d_<", "ɗ"), ("d`", "ɖ"), ("l`", "ɭ"), ("n`", "ɳ"), ("r`", "ɽ"), ("s`", "ʂ"), ("t`", "ʈ"), ("z`", "ʐ"), ("@`", "ə"), ("a", "a"), ("b", "b"), ("c", "c"), ("d", "d"), ("e", "e"), ("f", "f"), ("g", "g"), ("h", "h"), ("i", "i"), ("j", "j"), ("k", "k"), ("l", "l"), ("m", "m"), ("n", "n"), ("o", "o"), ("p", "p"), ("q", "q"), ("r", "r"), ("s", "s"), ("t", "t"), ("u", "u"), ("v", "v"), ("x", "x"), ("w", "w"), ("y", "y"), ("z", "z"), ("A", "ɑ"), ("B", "β"), ("C", "ç"), ("D", "ð"), ("E", "ɛ"), ("F", "ɱ"), ("G", "ɣ"), ("H", "ɥ"), ("I", "ɪ"), ("J", "ɲ"), ("K", "ɬ"), ("L", "ʎ"), ("M", "ɯ"), ("N", "ŋ"), ("O", "ɔ"), ("P", "ʋ"), ("Q", "ɒ"), ("R", "ʁ"), ("S", "ʃ"), ("T", "θ"), ("U", "ʊ"), ("V", "ʌ"), ("X", "χ"), ("W", "ʍ"), ("Y", "ʏ"), ("Z", "ʒ"), ("@", "ə"), ("{", "ɐ"), ("}", "ʉ"), ("1", "ɨ"), ("2", "ø"), ("3", "ɜ"), ("4", "ɾ"), ("5", "ɫ"), ("6", "ɶ"), ("7", "ɵ"), ("8", "ɵ"), ("9", "œ̞"), ("&", "ɶ̝"), ("?", "ʔ")];

pub const PULMONIC_CONSONANTS: [&str; 109] = ["m̥", "m", "ɱ̊", "ɱ", "n̼", "n̥", "n", "ɳ̊", "ɳ", "ɲ̊", "ɲ", "ŋ̊", "ŋ", "ɴ̥", "ɴ", "p", "b", "p̪", "b̪", "t̼", "d̼", "t", "d", "ʈ", "ɖ", "c", "ɟ", "k", "ɡ", "q", "ɢ", "ʡ", "ʔ", "s", "z", "ʃ", "ʒ", "ʂ", "ʐ", "ɕ", "ʑ", "ɸ", "β", "f", "v", "θ̼", "ð̼", "θ", "ð", "θ̠", "ð̠", "ɹ̠̊˔", "ɹ̠˔", "ɻ̊˔", "ɻ˔", "ç", "ʝ", "x", "ɣ", "χ", "ʁ", "ħ", "ʕ", "h", "ɦ", "ʋ", "ɹ", "ɻ", "j", "ɰ", "ʔ̞", "ⱱ̟", "ⱱ", "ɾ̼", "ɾ̥", "ɾ", "ɽ̊", "ɽ", "ɢ̆", "ʡ̆", "ʙ̥", "ʙ", "r̥", "r", "ɽ̊r̥", "ɽr", "ʀ̥", "ʀ", "ʜ", "ʢ", "ɬ", "ɮ", "ꞎ", "ʎ̝", "ʟ̝", "l", "ɭ", "ʎ", "ʟ", "ʟ̠", "ɺ̥", "ɺ", "ʎ̆", "ʟ̆", "ɥ̊", "ɥ", "ʍ", "w", "ɫ"];
pub const EJECTIVE_CONSONANTS: [&str; 17] = ["pʼ", "tʼ", "ʈʼ", "cʼ", "kʼ", "qʼ", "ʡʼ", "ɸʼ", "fʼ", "θʼ", "sʼ", "ʃʼ", "ʂʼ", "ɕʼ", "xʼ", "χʼ", "ɬʼ"];
pub const CLICK_CONSONANTS: [&str; 30] = ["kʘ", "qʘ", "kǀ", "qǀ", "kǃ", "qǃ", "kǂ", "qǂ", "ɡʘ", "ɢʘ", "ɡǀ", "ɢǀ", "ɡǃ", "ɢǃ", "ɡǂ", "ɢǂ", "ŋʘ", "ɴʘ", "ŋǀ", "ɴǀ", "ŋǃ", "ɴǃ", "ŋǂ", "ɴǂ", "kǁ", "qǁ", "ɡǁ", "ɢǁ", "ŋǁ", "ɴǁ"];
pub const IMPLOSIVE_CONSONANTS: [&str; 12] = ["ɓ", "ɗ", "ᶑ", "ʄ", "ɠ", "ʛ", "ɓ̥", "ɗ̥", "ᶑ̊", "ʄ̊", "ɠ̊", "ʛ̥"];
pub const PULMONIC_AFFRICATES: [&str; 33] = ["ts", "dz", "t̠ʃ", "d̠ʒ", "tʂ", "dʐ", "tɕ", "dʑ", "pɸ", "bβ", "p̪f", "b̪v", "t̪θ", "d̪ð", "tɹ̝̊", "dɹ̝", "t̠ɹ̠̊˔", "d̠ɹ̠˔", "cç", "ɟʝ", "kx", "ɡɣ", "qχ", "ɢʁ", "ʡʜ", "ʡʢ", "ʔh", "tɬ", "dɮ", "tꞎ", "dɭ˔", "ɟʎ̝", "ɡʟ̝"];
pub const EJECTIVE_AFFRICATES: [&str; 7] = ["t̪θʼ", "tsʼ", "t̠ʃʼ", "tʂʼ", "kxʼ", "qχʼ", "tɬʼ"];
pub const VOWELS: [&str; 33] = ["i", "y", "ɨ", "ʉ", "ɯ", "u", "ɪ", "ʏ", "ʊ", "e", "ø", "ɘ", "ɵ", "ɤ", "o", "e̞", "ø̞", "ə", "ɤ̞", "o̞", "ɛ", "œ", "ɜ", "ɞ", "ʌ", "ɔ", "æ", "ɐ", "a", "ɶ", "ä", "ɑ", "ɒ"];

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

        let no_consonant_cats = rng.gen_range(1..=(consonants.len() as f32 / 3.0).round() as usize);
        let no_vowel_cats = rng.gen_range(1..=(vowels.len() as f32 / 3.0).round() as usize);
        let no_pats = rng.gen_range(1..5);

        let mut categories = HashMap::<String, Vec<String>>::new(); 
        let mut patterns = Vec::<Pattern>::new();

        for i in 0..=no_consonant_cats {
            let category_size = rng.gen_range(1..=consonants.len());
            categories.insert(
                i.to_string(),
                consonants
                    .choose_multiple(&mut rng, category_size)
                    .map(String::from)
                    .collect::<Vec<String>>()
            );
        }
        
        for j in no_consonant_cats..=no_vowel_cats+no_consonant_cats {
            let category_size = rng.gen_range(1..=vowels.len());
            categories.insert(
                j.to_string(),
                vowels
                    .choose_multiple(&mut rng, category_size)
                    .map(String::from)
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
                        .map(String::from)
                        .collect::<String>(),
                    "any".to_string(),
                    "default".to_string()
                )
            );
        }

        let temp = TextGenerator::new("temp".to_string(), categories.clone(), patterns.clone(), Vec::<Rule>::new());
        TextGenerator::new(temp.random_length_word(1, 4, 0.0), categories, patterns, Vec::<Rule>::new())
    }
}
