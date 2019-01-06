use super::word_entity::WordEntity;
use console::style;

pub fn print_word(word: &WordEntity) {
    println!(
        "Conjugation for word: {}
is:",
        style(&word.dictionary_form).bold()
    )
}
