use std::string::String;
use crate::tokenizer::*;

pub struct Token<'a> {
    array: Vec<&'a str>,
}

impl<'a> Token<'a> {
    pub fn new() -> Token<'a> {
        let mut list = vec![
            "then",
            "end",
            "do",
            "==",
            ">=",
            "<=",
            "function",
            "=",
            "=>",
        ];

        list.sort_by(|a, b| b.char_indices().count().cmp(&a.char_indices().count()));

        Token {
            array: list,
        }
    }
}
impl<'a> TokenModel for Token<'a> {
    fn name(&self) -> String {
        return String::from("keyword");
    }
    fn capture(&self, tk: &Tokenizer) -> bool {
        for &keyword in &self.array {
            if keyword == tk.get_chars(tk.get_pos(), tk.get_pos() + keyword.char_indices().count()) {
                tk.advance(keyword.char_indices().count() as isize);
                return true;
            }
        }

        return false;
    }
}