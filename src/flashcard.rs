struct Flashcard {
    front: String,
    back: String,
}

impl Flashcard {
    fn show_front(language_to_ask: LanguageToDisplay) {
        front = Data::get_question_word(language_to_ask);
    }
    
    fn show_back(language_for_answer: LanguageToDisplay) {
        back = Data::get_answer_word(language_for_answer);
    }
}