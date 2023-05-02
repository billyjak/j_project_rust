use std::{fs, path::Path};

use serde_json::{Map, Value};
pub struct Dao {}

impl Dao {
    pub fn get_map_using_serde() -> serde_json::Map<String, Value> {
        let path: &Path = Path::new("src/resources/data.json");

        let contents: String =
            fs::read_to_string(path).unwrap_or("Should have been able to read the file".to_string());

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&contents);

        return data
            .unwrap_or("We could not unwrap the data".into())
            .as_object()
            .unwrap_or(&Map::new())
            .clone();
    }
}
