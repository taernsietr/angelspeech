use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::types::{Categories, Patterns, Ruleset, Pattern, Rule};

impl Deref for Categories {
    type Target = HashMap<String, Vec<String>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Categories {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target { &mut self.0 }
}

impl Deref for Patterns {
    type Target = Vec<Pattern>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Patterns {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target { &mut self.0 }
}

impl Deref for Ruleset {
    type Target = Vec<Rule>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Ruleset {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target { &mut self.0 }
}

