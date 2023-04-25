use std::{fs, path::Path};

use serde_json::{Value, Map};

pub struct Dao {}

impl Dao {
    pub fn get_serde_value() -> serde_json::Map<std::string::String, Value> {
        
        let path: &Path = Path::new("src/resources/data.json");
        let contents: String =
            fs::read_to_string(path).expect("Should have been able to read the file");
        let data: Result<Value, serde_json::Error> = serde_json::from_str(&contents);
        let result: Result<String, Value> = match data {
            Ok(d) => {
                let binding = d.as_object().unwrap();
                return binding.clone()
            },
            Err(e) => {
                let mut binding = Map::new();
                binding["Error"] = "Sorry, we could not map the data".into();
                return binding
            }
        };
    }
}