use rand::{Rng, prelude::SliceRandom};
use crate::types::{TextGenerator, TextParams};

impl TextGenerator {
    pub fn random_text(&self, params: &TextParams) -> String {
        (0..params.text_size).map(|_| { self.rlword(params) })
            .collect::<Vec<String>>()
            .join(" ")
    }

    // TODO: configurable pseudotext, ie "command" gives a certain pattern of output different than
    // "wikipedia article"
    pub fn pseudotext(&self, params: &TextParams) -> String {
        let mut rng = rand::thread_rng();
        let mut pseudotext = String::new();
        let roots: Vec<String> = (0..Ord::min(params.text_size/2, 12)).map(|_| self.rlword(params)).collect();
        let particles: Vec<String> = (0..Ord::min(params.text_size, 10)).map(|_| self.rword(Ord::min(params.min_syllables, 2))).collect();
      //let morpheme: Vec<String> = (0..Ord::max(length, 12)).map(|_| self.random_word(1)).collect();

        let mut last_type = "none"; 
        for i in 0..=params.text_size {
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
