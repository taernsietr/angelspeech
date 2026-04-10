use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::types::Categories;

impl Deref for Categories {
    type Target = HashMap<String, Vec<String>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Categories {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target { &mut self.0 }
}

