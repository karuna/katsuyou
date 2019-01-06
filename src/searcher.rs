use super::word_entity::WordEntity;

pub enum Result<'a> {
    None,
    Single(&'a WordEntity<'a>),
    Many(&'a [WordEntity<'a>]),
}

pub fn search_word(word: &str) -> Result {
    Result::None
}
