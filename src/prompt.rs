use serde_json::{Value, Map};

use crate::dao::Dao;

pub struct Prompt {
    pub category: String,
    pub translation: String,
}

impl Prompt {
    pub fn get_word_for_prompt(self, language: &String) -> String {
        let data = Dao::get_serde_value();

        let result = get_category(
            data, &self.category);

        return result;
    }
}

fn get_category(data: Map<String, Value>, desired_category: &str) -> String {
    for keys in data.keys() {
        if keys.eq(desired_category) {
            let result = keys.clone();
            return result
        }
    }
    String::from("Sorry, we could not find your category.")
}
