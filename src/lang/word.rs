use super::phonology::Phoneme;

enum WordClass {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Particle
}

enum PhonologicalProcess {
    Prefixation,
    Infixation,
    Suffixation,
    Circumfixation,
    Metathesis,
}

pub struct Word {
    class: WordClass,
    phonemes: Vec<Phoneme>,
}

pub struct Morpheme {
    form: Vec<Phoneme>,
    applies_to: WordClass,
    by: PhonologicalProcess,
    meaning: String,
}
 
impl Word {
    pub fn get_anagrams(&self) -> Vec<String> { todo!() }
    pub fn is_anagram_of(&self, word: impl Into<String>) -> bool { todo!() }
}
