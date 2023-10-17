pub struct Phone {
    ipa: String,
    xsampa: String
}

pub struct Phoneme {
    realization: Vec<(Phone, String)> // where String => condition
}

pub struct PhonologicalRule {
    input: Vec<Phoneme>,
    output: Vec<Phoneme>,
    env: String,
}
