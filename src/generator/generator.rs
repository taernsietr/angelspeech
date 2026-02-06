use std::collections::HashMap;
use crate::types::{Pattern, Rule, TextGenerator};

#[allow(dead_code)]
impl TextGenerator {
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn as_json(&self) -> String { serde_json::to_string(&self).unwrap() }

    pub fn new<N: Into<String>>(
        name: N,
        categories: HashMap<String, Vec<String>>,
        patterns: Vec<Pattern>,
        ruleset: Vec<Rule>
    ) -> TextGenerator {
        TextGenerator { name: name.into(), categories, patterns, ruleset }
    }
    
    pub fn new_empty<N: Into<String>>(name: N) -> TextGenerator {
        TextGenerator {
            name: name.into(), 
            categories: HashMap::new(),
            patterns: Vec::<Pattern>::new(),
            ruleset: Vec::<Rule>::new()
        }
    }

    pub fn load(file: PathBuf) -> TextGenerator {
        let data = std::fs::read_to_string(&file).expect("Failed to load generator settings file");
        serde_json::from_str::<TextGenerator>(&data)
            .unwrap_or_else(|_| panic!("Failed to read JSON data in {}", &file.to_str().unwrap()))
    }
   
    pub fn save(&self, settings_folder: PathBuf) {
        let mut file = settings_folder.join(&self.name);
        file.set_extension("json");
        std::fs::write(file, self.as_json()).unwrap();
    }
}

