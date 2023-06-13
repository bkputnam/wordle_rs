static WORD_LIST_STR: &str = include_str!("wordlist.txt");

lazy_static! {
    pub static ref WORD_LIST: Vec<&'static str> = WORD_LIST_STR.split('\n').collect();
}
