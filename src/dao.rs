use std::{fs, path::Path};

use serde_json::Value;

pub struct Dao {}

impl Dao {
    pub fn get_serde_value() {
        
        let path: &Path = Path::new("src/resources/data.json");
        let contents: String =
            fs::read_to_string(path).expect("Should have been able to read the file");
        return serde_json::from_str(&contents).unwrap();
        // return &data;
    }
}