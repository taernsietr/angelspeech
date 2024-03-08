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

struct PhonologicalRule { input: Vec<Phoneme>, output: Vec<Phoneme>, env: String }
struct Phone { ipa: String, xsampa: String }
struct Phoneme { realization: Vec<(Phone, String)> } // where String => condition
struct Morpheme { form: Vec<Phoneme>, applies_to: WordClass, by: PhonologicalProcess, meaning: String }
struct Word { class: WordClass, phonemes: Vec<Phoneme> }
struct DictEntry;
struct Language {
    name: String,
    phonemes: Vec<Phoneme>,
    word_level_ruleset: Vec<PhonologicalRule>,
    morphemes: Vec<Morpheme>,
    lexicon: Vec<(Word, Option<DictEntry>)>,
}

impl Phoneme {
    fn primary(&self) -> String {
        self.realization[0].1.clone()
    }
}

impl Word {
    pub fn get_anagrams(&self) -> Vec<String> { todo!() }
    pub fn is_anagram_of(&self, word: impl Into<String>) -> bool { todo!() }

}

impl Morpheme {
    pub fn derive(&self, word: Word) -> Word {
        match self.by {
            PhonologicalProcess::Prefixation => { },
            PhonologicalProcess::Infixation => { },
            PhonologicalProcess::Suffixation => { },
            PhonologicalProcess::Circumfixation => { },
            PhonologicalProcess::Metathesis => { }
        }
    }
}
