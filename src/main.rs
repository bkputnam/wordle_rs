// use wordle_rs::compare::compare;

use wordle_rs::{wordlist::WORD_LIST, next_guess::get_next_guess, word_set::WordSet};

fn main() {
    // let mut remaining_words:HashSet<String> = HashSet::new();
    let mut remaining_words = WordSet::new();
    for word in WORD_LIST.iter().copied() {
        remaining_words.insert(word.to_string());
    }
    let first_guess = get_next_guess(&remaining_words);
    println!("First guess: {first_guess}");
}
