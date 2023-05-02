use crate::prompt::Prompt;

mod answer;
mod prompt;
mod dao;
mod question;
mod enums;

fn main() {
    let p = Prompt { category: "times".to_string() };

    println!("{}", p.get_word_for_prompt());
}
