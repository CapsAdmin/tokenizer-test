use std::string::String;
use crate::tokenizer::*;

pub struct Token;
impl TokenModel for Token {
    fn name(&self) -> String {
        return String::from("space");
    }
    fn capture(&self, tk: &Tokenizer) -> bool {
        if !syntax::is_space(tk.get_char()) {
            return false;
        }

        for _ in tk.iterate() {
            tk.advance(1);
            
            if !syntax::is_space(tk.get_char()) {
                return true;
            }
        }

        return false;
    }
}