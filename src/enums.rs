enum Variant {
    Hiragana,
    Katakana,
}

enum StyleOfQuestion {
    Flashcard,
    QandA,
    MultipleChoice,
}

enum PromptLanguage {
    English,
    Kanji,
    Roman,
    Kana(Variant),
}

enum AnswerLanguage {
    English,
    Kanji,
    Roman,
    Kana(Variant),
}