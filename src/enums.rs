enum Variant {
    Hiragana,
    Katakana,
}

enum LanguageToDisplay {
    English,
    Kanji,
    Roman,
    Kana(Variant),
}

enum StyleOfQuestion {
    Flashcard,
    QandA,
    MultipleChoice,
}