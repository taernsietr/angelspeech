use rand::{Rng, RngExt, distr::{Distribution, StandardUniform}};
use crate::types::{PatternWeight,PatternPosition};

impl Distribution<PatternWeight> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PatternWeight {
        match rng.random_range(1..100) {
            1..=70 => PatternWeight::Default,
            71..=85 => PatternWeight::Light,
            86..=100 => PatternWeight::Heavy,
            _ => unreachable!()
        }
    }
}

impl Distribution<PatternPosition> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PatternPosition {
        match rng.random_range(1..100) {
            1..=70 => PatternPosition::Any,
            71..=75 => PatternPosition::Initial,
            76..=80 => PatternPosition::Medial,
            81..=85 => PatternPosition::Final,
            86..=90 => PatternPosition::NonInitial,
            91..=95 => PatternPosition::NonMedial,
            96..=100 => PatternPosition::NonFinal,
            _ => unreachable!()
        }
    }
}

