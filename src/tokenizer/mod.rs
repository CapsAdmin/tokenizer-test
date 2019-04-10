#[macro_use]

pub mod syntax;

use std::cell::Cell;

pub mod tokens;

fn register_tokens(mut list: Vec<Box<TokenModel>>) -> Vec<Box<TokenModel>> {
    use tokens::*;

    list.push(Box::new(space::Token));
    list.push(Box::new(keyword::Token::new()));
    list.push(Box::new(identifier::Token));
    list.push(Box::new(number::Token::new()));

    return list;
}

pub struct TokenizerError {
    msg: String,
    start: usize,
    stop: usize,
}

pub fn tokenizer_error(msg: String, start: usize, stop: usize) -> TokenizerError {
    TokenizerError {
        msg, start, stop
    }
}

pub type CaptureResult = Result<bool, TokenizerError>;

pub trait TokenModel {
    fn capture(&self, tk: &Tokenizer) -> CaptureResult;
    fn name(&self) -> &str;
}

pub struct CapturedToken<'a> {
    pub start: u64,
    pub stop: u64,
    pub type_name: &'a str,
}
impl<'a> CapturedToken<'a> {
    pub fn new(start: u64, mut stop: u64, type_name: &str) -> CapturedToken {
        if start == stop {
            stop += 1;
        }
        CapturedToken { start, stop, type_name }
    }
}

pub struct Tokenizer<'a> {
    pos: Cell<usize>,
    code: &'a str,
    tokens: Vec<Box<TokenModel>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &str) -> Tokenizer {
        Tokenizer {
            pos: Cell::new(0),
            code: code,
            tokens: register_tokens(Vec::new()),
        }
    }
    pub fn read_all(&self) -> Vec<CapturedToken> {
        let mut captured: Vec<CapturedToken> = vec!();

        for _ in self.iterate() {
            let mut found = false;
            
            for tk in &self.tokens {
                let pos = self.get_pos();
                match tk.capture(&self) {
                    Ok(b) => {
                        if b {
                            let token = CapturedToken::new(pos as u64, self.get_pos() as u64, tk.name());
                            captured.push(token);
                            found = true;
                        }
                    }
                    Err(e) => {
                        println!("!!{}: ⸢{}⸥", e.msg, self.get_chars(e.start as usize, e.stop as usize));
                    }
                }
            }

            if !found {
                println!("unhandled character ⸢{}⸥\n", self.get_char());
                self.advance(1);
            }

            if self.the_end() {
                break;
            }
        }

        return captured;
    }
    pub fn get_char(&self) -> char {
        if self.get_pos() >= self.length() {
            return self.code.char_indices().last().unwrap().1;
        }
        return self.code.char_indices().nth(self.get_pos()).unwrap().1;
    }
    pub fn get_char_offset(&self, offset: isize) -> char {
        return self.code.char_indices().nth(self.get_pos() + (offset as usize)).unwrap().1;
    }
    pub fn iterate(&self) -> std::ops::Range<usize> {
        self.get_pos()..self.length()
    }
    pub fn advance(&self, num: isize) {
        let pos = if num < 0 {
            self.code.char_indices().nth(self.get_pos() - num as usize)
        } else {
            self.code.char_indices().nth(self.get_pos() + num as usize)
        };

        if pos.is_some() {
            self.pos.set(pos.unwrap().0);
        } else {
            self.pos.set(self.length());
            //panic!("reached end of code {}", self.get_pos() + 1);
        }
    }

    pub fn the_end(&self) -> bool {
        return self.get_pos() >= self.length();
    }

    pub fn length(&self) -> usize {
        return self.code.char_indices().count();
    }

    pub fn get_chars(&self, mut start: usize, mut stop: usize) -> &str {
        if stop > self.length() {
            stop = self.length();
        }
        if start > self.length() {
            start = self.length();
        }
        return &self.code[start .. stop];
    }
 
    pub fn get_pos(&self) -> usize {
        return self.pos.get();
    }

    pub fn assert_types(&self, type_list: Vec<&str>) {
        let tokens = self.read_all();

        if tokens.len() != type_list.len() {
            panic!("token list is not the same as type list");
        }
            

        let mut i = 0;
        for t in tokens {
            if t.type_name != type_list[i] {
                panic!("fail!")
            }
            i += 1;
        }
    }

    /*pub fn assert_values(&self, list: Vec<&str>) {
        let tokens = self.read_all();

        if (tokens.len() != list.len()) {
            panic!("token list is not the same as type list");
        }
            
        let mut i = 0;
        for t in tokens {
            if t.value != list[i] {
                panic!("fail!")
            }
            i += 1;
        }
    }*/
}