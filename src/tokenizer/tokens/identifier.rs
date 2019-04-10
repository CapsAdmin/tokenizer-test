use std::string::String;
use crate::tokenizer::{ Tokenizer, TokenModel, syntax };

pub struct Identifier;
impl TokenModel for Identifier {
    fn name(&self) -> String {
        return String::from("identifier");
    }
    fn capture(&self, tk: &Tokenizer) -> bool {
        if !syntax::is_letter(tk.get_char()) {
            false;
        }

        for _ in tk.iterate() {
            tk.advance(1);
            
            if !syntax::is_letter(tk.get_char()) && !syntax::is_number(tk.get_char()) {
                return true;
            }
        }

        return true;
    }
}