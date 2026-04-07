use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use thiserror::Error;
use crate::types::{Pattern, Rule, TextGenerator};

#[derive(Error, Debug)]
pub enum TextGeneratorError {
    #[error("Generator name must not be empty.")]
    EmptyGeneratorName,
    #[error("There must be at least a single category.")]
    EmptyCategories,
}

impl TextGenerator {
    /// Returns the name for the generator setting
    pub fn get_name(&self) -> String { self.name.clone() }
    
    /// Set a new name for the generator setting
    pub fn set_name<S: Into<String>>(&mut self, name: S) { self.name = name.into() }

    /// Creates a new generator setting
    pub fn new<N: Into<String>>(
        name: N,
        categories: HashMap<String, Vec<String>>,
        patterns: Vec<Pattern>,
        ruleset: Vec<Rule>
    ) -> TextGenerator {
        TextGenerator { name: name.into(), categories, patterns, ruleset }
    }
    
    /// Creates an empty generator setting
    pub fn empty<N: Into<String>>(name: N) -> TextGenerator {
        TextGenerator {
            name: name.into(), 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
            ruleset: Vec::<Rule>::new()
        }
    }

    /// Loads a generator setting from a JSON string
    pub fn from_json<T: Into<String>>(str: T) -> Result<TextGenerator, serde_json::Error> {
        serde_json::from_str::<TextGenerator>(&str.into())
    }

    /// Loads a generator setting from a JSON file
    pub fn from_file(file: PathBuf) -> anyhow::Result<TextGenerator> {
        let data = std::fs::read_to_string(&file)?;
        Ok(serde_json::from_str::<TextGenerator>(&data)?)
    }
   
    /// Saves a generator setting to a JSON file
    pub fn save_to_file(
        &self,
        settings_folder: PathBuf
    ) -> Result<(), std::io::Error> {
        let mut file = settings_folder.join(&self.name);
        file.set_extension("json");

        let inner = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e)
        };
        std::fs::write(file, inner)
    }
}

