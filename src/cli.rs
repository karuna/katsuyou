use clap::{App, Arg, ArgMatches};

pub fn app_arguments<'a>() -> ArgMatches<'a> {
    App::new("katsuyou")
        .version("0.0.1")
        .author("Karuna Murti <karuna.murti@gmail.com>")
        .about("katsuyou is a command line to show all conjugation form of a Japanese verb or adjective.")
                .arg(Arg::with_name("WORD")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Word (verb or adjective) to conjugate. Can be romaji, kanji, or kana.")
                ).get_matches()
}
