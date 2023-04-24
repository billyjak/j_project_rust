use std::fmt;

#[derive(Debug)]
pub enum Variant {
    Hiragana,
    Katakana,
}

pub enum StyleOfQuestion {
    Flashcard,
    QandA,
    MultipleChoice,
}

#[derive(Debug)]
pub enum PromptLanguage {
    English,
    Kanji,
    Roman,
    Kana(Variant),
}

pub enum AnswerLanguage {
    English,
    Kanji,
    Roman,
    Kana(Variant),
}

impl fmt::Display for PromptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}