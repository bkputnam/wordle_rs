use crate::word_set::Word;

static WORD_LIST_STR: &str = include_str!("wordlist.txt");

lazy_static! {
    pub static ref WORD_LIST: Vec<Word> = WORD_LIST_STR
        .split('\n')
        .map(|word: &str| Word::from_str(word))
        .collect();
}
