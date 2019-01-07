extern crate clap;
extern crate console;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

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
                dictionary_form: String::from("食べる"),
                translation: String::from("to eat"),
                word_type: WordType::VerbIchidan,
            };
            printer::print_word(&sample)
        }
        Result::Single(_result) => {}
        Result::Many(_results) => {}
    }
}
