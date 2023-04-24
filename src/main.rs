mod quiz;

use std::fs;
use std::path::Path;

use serde_json::Value;

fn main() {
    let path: &Path = Path::new("src/resources/data.json");

    let contents: String =
        fs::read_to_string(path).expect("Should have been able to read the file");
    let data: Value = serde_json::from_str(&contents).unwrap();

    let random_category = find_random_category(&data);
    let (category_string, value_of_remaining_json) = random_category;
    println!("The category is: {}", category_string);

    let random_word = find_random_word(value_of_remaining_json);
    let (word_string, value_of_remaining_json) = random_word;
    println!("The English word(s) is: {}", word_string);

    let answer = find_roman(value_of_remaining_json, "roman");
    println!("The japenese pronunciation is: {}", answer);
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

fn find_roman(data: Option<&Value>, type_of_translation: &str) -> String {
    let temp = Option::expect(data, "Sorry, we couldn't find the value");

    let map = temp.as_object().unwrap();
    let mut answer = Option::expect(map.get(type_of_translation), "Sorry").as_str();
    return answer.get_or_insert("I got nothing").to_string();
}
