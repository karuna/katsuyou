use super::word_entity::WordEntity;

pub enum Result {
    None,
    Single(WordEntity),
    Many(Vec<WordEntity>),
}

pub fn search_word(word: &str) -> Result {
    Result::None
}
