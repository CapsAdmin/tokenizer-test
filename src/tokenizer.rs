use std::cell::Cell;
use std::string::String;

mod tokens;
use tokens::*;

pub trait TokenModel {
    fn capture(&self, tk: &Tokenizer) -> bool;
    fn name(&self) -> String;
}

pub struct CapturedToken {
    pub start: u64,
    pub stop: u64,
}
impl CapturedToken {
    pub fn new(start: u64, stop: u64) -> CapturedToken {
        CapturedToken { start, stop }
    }
}

pub struct Tokenizer<'a> {
    pos: Cell<usize>,
    code: &'a str,
    token_classes: Vec<Box<TokenModel>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &str) -> Tokenizer {
        let mut classes: Vec<Box<TokenModel>> = Vec::new();

        classes.push(Box::new(tokens::space::Identifier));
        classes.push(Box::new(tokens::identifier::Identifier));

        Tokenizer {
            pos: Cell::new(0),
            code: code,
            token_classes: classes,
        }
    }
    pub fn read_all(&self) {
        for _ in self.iterate() {
            let mut found = false;
            
            for tk in &self.token_classes {
                let pos = self.pos.get();
                if tk.capture(&self) {
                    let token = CapturedToken::new(pos as u64, self.pos.get() as u64);  
                    println!("{}⸢{}⸥", tk.name(), self.code[(token.start as usize) .. (token.stop as usize)].to_string()); 
                    found = true;
                }
            }

            if !found {
                println!("unhandled character {}\n", self.get_char());
                self.advance(1);
            }

            if self.the_end() {
                break
            }
        }
    }
    pub fn get_char(&self) -> char {
        return self.code.char_indices().nth(self.pos.get()).unwrap().1;
    }
    pub fn get_char_offset(&self, offset: isize) -> char {
        return self.code.char_indices().nth(self.pos.get() + (offset as usize)).unwrap().1;
    }
    pub fn iterate(&self) -> std::ops::Range<usize> {
        self.pos.get()..self.code.char_indices().count()-1
    }
    pub fn advance(&self, num: isize) {
        let pos = if num < 0 {
            self.code.char_indices().nth(self.pos.get() - num as usize)
        } else {
            self.code.char_indices().nth(self.pos.get() + num as usize)
        };

        if pos.is_some() {
            return self.pos.set(pos.unwrap().0);
        }

        panic!("reached end of code {}", self.pos.get() + 1);
    }

    pub fn the_end(&self) -> bool {
        self.pos.get() >= self.code.char_indices().count()-1
    }
}