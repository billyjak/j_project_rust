struct Quiz {
    question: Question,
    answer: Answer,
}

impl Quiz {
    fn form_question (
        style_of_question: StyleQuestion, language_to_display: LanguageToDisplay) {

        let style = match style_of_question {
            Flashcard => Flashcard::show_front(language_to_display),
            QandA => QandA::show_question(language_to_display),
            MultipleChoice => MultipleChoice::show_question(language_to_display),
        };
    }

    fn evaluate_answer() {}
}