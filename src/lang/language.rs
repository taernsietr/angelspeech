use super::{
    word::Word,
    dict::DictEntry,
    phonology::{Phoneme, PhonologicalRule},
    morphology::Morpheme,
};

struct Language {
    name: String,
    phonemes: Vec<Phoneme>,
    word_level_ruleset: Vec<PhonologicalRule>,
    morphemes: Vec<Morpheme>,
    lexicon: Vec<(Word, Option<DictEntry>)>,
}
