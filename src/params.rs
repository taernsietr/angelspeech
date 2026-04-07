use std::cmp::max;
use rand::{
    rngs::ThreadRng,
    Rng
};

use crate::types::TextParams;

impl TextParams {
    /// Returns a new set of parameters for generating words or text
    /// This method is infallible; invalid given values will be reset to defaults
    pub fn new(
        min_syllables: u8,
        max_syllables: u8,
        bias: f32,
        text_size: u8,
    ) -> Self {
        if text_size < 1 {
            println!("[Angelspeech] Warning: text size must be at least 1. Defaulting to 1.");
        }
        if min_syllables <= 0 || max_syllables <= 0 {
            println!("[Angelspeech] Warning: Syllable values have to be greater than zero. Defaulting to 1.");
        }
        if min_syllables > max_syllables {
            println!("[Angelspeech] Warning: Minimum syllable value has to be equal to or less than maximum syllables. Defaulting to the minimum given value.")
        }

        let text_size = max(1, text_size);
        let max_syllables = max(1, max_syllables);
        let min_syllables = min_syllables.clamp(1, max_syllables);

        TextParams {
            min_syllables,
            max_syllables,
            bias,
            text_size
        }
    }

    /// Generates a random syllable count based on its minimum and maximum values
    pub fn syllables(&self, rng: &mut ThreadRng) -> u8 {
        // used logic from https://stackoverflow.com/questions/29325069/how-to-generate-random-numbers-biased-towards-one-value-in-a-range
        let mix = self.bias.abs() * rng.gen::<f32>();
        let bias_t = {
            if self.bias > 0.0 { self.max_syllables } 
            else if self.bias < 0.0 { self.min_syllables }
            else { 0 }
        };

        let rnd = rng.gen_range(self.min_syllables..=self.max_syllables) as f32;
        (rnd * (1.0 - mix) + bias_t as f32 * mix) as u8
    }
}
