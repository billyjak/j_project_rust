use crate::answer::Answer;
use crate::prompt::Prompt;

mod answer;
mod prompt;

fn main() {
    let p = Prompt {};
    let (category, word, style, language) = 
            p.get_word_for_prompt(&String::from("kana"));

    let a = Answer {};

    // call answer with some of this info (and possibly more) to get answer
}
