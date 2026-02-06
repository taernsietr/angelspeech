use std::path::PathBuf;

use crate::types::TextGenerator;

impl TextGenerator {
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
