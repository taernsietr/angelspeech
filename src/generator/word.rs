struct Word {
    language: String,
    phonemes: Vec<Phoneme>
}
 
impl Word {
    pub fn realization(&self) -> Vec<Phone> { unimplemented!() }
    pub fn derive(&self, morpheme: Morpheme) -> Word { unimplemented!() }

    pub fn is_anagram_of(&self, word: String) -> bool { unimplemented!() }
    pub fn get_anagrams(&self) -> Vec<String> { unimplemented!() }
}
