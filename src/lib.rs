//! Tools for generating conlang resources, such as random word 
//! lists and pseudotext, and more advanced options such as 
//! randomly generated phonologies.

#![allow(dead_code)]

pub(crate) mod category_based;
pub(crate) mod mora;
pub(crate) mod pattern;
pub(crate) mod rules;
pub(crate) mod utils;
pub(crate) mod types;
pub(crate) mod common;
pub(crate) mod traits;
pub(crate) mod params;

pub mod prelude {
    pub use crate::types::{
        TextParams,
        TextType,
        TextGenerator
    };
    pub use crate::traits::IntoTextGenerator;
}
