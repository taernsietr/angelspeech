use std::path::PathBuf;

use crate::generator::text_generator::TextGenerator;

impl TextGenerator {

    // TODO: Refactor this entire function, this is somewhat disgusting
    pub fn load_local(file: PathBuf) -> TextGenerator {

        let data = std::fs::read_to_string(&file).expect("Failed to load generator settings file");
        
        let generator: TextGenerator = serde_json::from_str::<TextGenerator>(&data)
            .unwrap_or_else(|_| panic!("Failed to read JSON data in {}", &file.to_str().unwrap()));
        
        /*
         * Error checking:
         * patterns must not have any symbol that isn't assigned to a category;
         * pattern symbols must be capital letters or numbers;
         * pattern symbols must be unique within a generator - using a symbol more than once will
         * overwrite the previous categories upon loading the generator;
         * ideally, the program should not panic when encountering these errors, but an elegant
         * solution has to be found first.
         */

        let result = {
            // TODO: make this check more than just the length
            let defined_symbols = generator.categories.keys();
            let mut used_symbols = Vec::<String>::new();

            for i in &generator.patterns {
                for j in i.pattern().chars() {
                    if j.is_uppercase() || j.is_numeric() {
                        used_symbols.push(j.to_string());
                    }
                }
            }
            used_symbols.sort();
            used_symbols.dedup();
            used_symbols.len() <= defined_symbols.len()
        }; 
        if result { generator }
        else { panic!("Mismatch between number of defined and used pattern symbols"); }
    }
   
    pub fn save_local(&self, settings_folder: PathBuf) {
        let file = settings_folder.join(&self.name);
        std::fs::write(file, self.as_json()).unwrap();
    }
}
