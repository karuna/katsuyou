extern crate clap;
extern crate console;

mod cli;
mod printer;
mod searcher;
mod word_entity;

use self::searcher::{search_word, Result};
use self::word_entity::{WordEntity, WordType};

pub fn run() {
    let args = cli::app_arguments();
    let word = args.value_of(cli::WORD).unwrap();
    match search_word(word) {
        Result::None => {
            let sample = WordEntity {
                dictionary_form: "食べる",
                translation: "to eat",
                word_type: WordType::VerbIchidan,
            };
            printer::print_word(&sample)
        }
        Result::Single(_result) => {}
        Result::Many(_results) => {}
    }
}
