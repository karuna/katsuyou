#[macro_use]
extern crate human_panic;

use katsuyou;

fn main() {
    setup_panic!();
    katsuyou::run();
}
