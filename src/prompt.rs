use std::{fs, path::Path};

use serde_json::Value;

pub struct Prompt {}

impl Prompt {
    // todo: abstract logic into new file, possibly "dao.rs"
    pub fn get_word_for_prompt(self, language: &String) -> (String, String, String, String) {
        let path: &Path = Path::new("src/resources/data.json");

        let contents: String =
            fs::read_to_string(path).expect("Should have been able to read the file");
        let data: Value = serde_json::from_str(&contents).unwrap();

        let random_category = find_random_category(&data);
        let (category_string, value_of_remaining_json) = random_category;

        let random_word = find_random_word(value_of_remaining_json);
        let (word_string, value_of_remaining_json) = random_word;

        let prompt_word = find_word(value_of_remaining_json, language.to_string());

        return (category_string, word_string, prompt_word, language.to_string());
    }
}

fn find_random_category<'a>(data: &'a Value) -> (std::string::String, Option<&Value>) {
    let mut count = 0;

    let map = data.as_object().unwrap();
    let i = fastrand::usize(..map.len());

    for key in map.keys() {
        if count == i {
            return (String::from(key), map.get(key));
        }
        count += 1;
    }

    return (
        "Something went wrong".to_string(),
        std::option::Option::None,
    );
}

fn find_random_word(data: Option<&Value>) -> (std::string::String, Option<&Value>) {
    let mut count = 0;

    let temp = Option::expect(data, "Sorry, we couldn't find the value");
    let map = temp.as_object().unwrap();
    let i = fastrand::usize(..map.len());

    for key in map.keys() {
        if count == i {
            return (String::from(key), map.get(key));
        }
        count += 1;
    }

    return (
        "Something went wrong".to_string(),
        std::option::Option::None,
    );
}

fn find_word(data: Option<&Value>, type_of_translation: String) -> String {
    let temp = Option::expect(data, "Sorry, we couldn't find the value");

    let map = temp.as_object().unwrap();
    let mut prompt_word = Option::expect(map.get(&type_of_translation), "Sorry").as_str();
    return prompt_word.get_or_insert("I got nothing").to_string();
}
