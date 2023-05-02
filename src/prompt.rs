use serde_json::{Map, Value};

use crate::dao::Dao;
use crate::enums::PromptLanguage;

pub struct Prompt {
    pub category: String,
    prompt_language: PromptLanguage,
}

impl Prompt {
    pub fn generate_prompt(category: String, prompt_language: PromptLanguage) {}



    pub fn get_word_for_prompt(&self) -> String {
        let data = Dao::get_map_using_serde();

        let result = get_category(data, &self.category);

        return result;
    }
}

fn get_category_map(data: Map(String, Value) -> Map<String, Value>) {
    let mut mappy = Map::new();
    for keys in data.keys() {
        mappy.insert(keys, )
    }
}
fn get_category(data: Map<String, Value>, desired_category: &str) -> String {
    for keys in data.keys() {
        if keys.eq(desired_category) {
            let result = keys.clone();
            return result;
        }
    }
    String::from("Sorry, we could not find your category.")
}
