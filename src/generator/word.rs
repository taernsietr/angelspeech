struct Word {
    language: String,
    phonemes: Vec<Phoneme>
}
 
impl Word {
    pub fn realization(&self) -> Vec<Phone> { unimplemented!() }
    pub fn derive(&self, morpheme: Morpheme) -> Word { unimplemented!() }
}
