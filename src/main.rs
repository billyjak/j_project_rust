use crate::answer::Answer;
use crate::prompt::Prompt;
use crate::dao::Dao;

mod answer;
mod prompt;
mod dao;

fn main() {
    let p = Prompt {};
    let (category, word, style, language) = 
            p.get_word_for_prompt(&String::from("kana"));

    let a = Answer {};

    // call answer with some of this info (and possibly more) to get answer
}
