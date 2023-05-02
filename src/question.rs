use crate::{prompt::Prompt, answer::Answer, enums};
use crate::enums::StyleOfQuestion;

pub struct Question {
    prompt: Prompt,
    answer: Answer,
    style_of_question: StyleOfQuestion,
}

impl Question {
    pub fn generate_question(
        self,
        prompt: Prompt,
        answer: Answer,
        style_of_question: StyleOfQuestion,
    ) -> Question {
        let prompt = prompt;
        let answer = answer;
        let style_of_question = style_of_question;
        self
    }
    pub fn ask_question(question: Question) {
        match question.style_of_question {
            enums::StyleOfQuestion::Flashcard => println!("flashcard"),
            enums::StyleOfQuestion::QandA => println!("QnA"),
            enums::StyleOfQuestion::MultipleChoice => println!("MultipleChoice"),
        }
    }
}
