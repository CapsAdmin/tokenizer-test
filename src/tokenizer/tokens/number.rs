
use crate::tokenizer::*;

pub struct Token {
    seen_dot: std::cell::Cell<bool>,
}

impl Token {
    pub fn new() -> Token {
        Token {
            seen_dot: std::cell::Cell::new(false)
        }
    }
}

impl TokenModel for Token {
    fn name(&self) -> &str {
        return "number";
    }
    fn capture(&self, tk: &Tokenizer) -> CaptureResult {
        if !syntax::is_number(tk.get_char()) && !(tk.get_char() == '.' && syntax::is_number(tk.get_char_offset(1))) {
            return Ok(false);
        }

        if tk.get_char() == '0' {
            if tk.get_char_offset(1) == 'b' || tk.get_char_offset(1) == 'B' {
                return self.capture_binary(tk);
            }

            if tk.get_char_offset(1) == 'x' || tk.get_char_offset(1) == 'X' {
                return self.capture_hex(tk);
            }

            if tk.get_char_offset(1) == 'o' || tk.get_char_offset(1) == 'O' {
                return self.capture_octal(tk);
            }
        }

        return self.capture_number(tk);
    }
}

impl Token {
    fn capture_number(&self, tk: &Tokenizer) -> CaptureResult {
        for _ in tk.iterate() {
            let c = tk.get_char();

            if c == '.' {
                if self.seen_dot.get() {
                    return Err(tokenizer_error(format!("unexpected dot in number"), tk.get_pos(), tk.get_pos() + 1));
                }
                self.seen_dot.set(true)
            }

            if c == '_' {
                tk.advance(1);
                continue;
            }

            if syntax::is_number(c) || c == '.' {
                tk.advance(1);
            } else if syntax::is_space(c) {
                return Ok(true);
            } else {
                return Err(tokenizer_error(format!("unexpected character in number"), tk.get_pos(), tk.get_pos() + 1));
            }
        }
        return Ok(true);
    }

    fn capture_binary(&self, tk: &Tokenizer) -> CaptureResult {
        tk.advance(2);
        
        for _ in tk.iterate() {
            let c = tk.get_char();

            if c == '_' {
                tk.advance(1);
                continue;
            }

            if c == '1' || c == '0' {
                tk.advance(1);
            } else if syntax::is_space(c) {
                return Ok(true);
            } else {
                return Err(tokenizer_error(format!("unexpected character in binary number"), tk.get_pos(), tk.get_pos() + 1));
            }
        }

        return Ok(true);
    }

    fn capture_octal(&self, tk: &Tokenizer) -> CaptureResult {
        tk.advance(2);
        
        for _ in tk.iterate() {
            let c = tk.get_char();

            if c == '_' {
                tk.advance(1);
                continue;
            }

            if syntax::is_number_within_range(c as u8, 0, 7) {
                tk.advance(1);
            } else if syntax::is_space(c) {
                return Ok(true);
            } else {
                return Err(tokenizer_error(format!("unexpected character in octal number"), tk.get_pos(), tk.get_pos() + 1));
            }
        }

        return Ok(true);
    }

    fn capture_hex(&self, tk: &Tokenizer) -> CaptureResult {
        tk.advance(2);
        
        for _ in tk.iterate() {
            let c = tk.get_char();

            if c == '_' {
                tk.advance(1);
                continue;
            }

            if 
                (c == 'a' || c == 'A') ||
                (c == 'b' || c == 'B') ||
                (c == 'c' || c == 'C') ||
                (c == 'd' || c == 'D') ||
                (c == 'e' || c == 'E') ||
                (c == 'f' || c == 'F') ||
                syntax::is_number(c)
            {
                tk.advance(1);
            } else if syntax::is_space(c) {
                return Ok(true);
            } else {
                return Err(tokenizer_error(format!("unexpected character in hex number"), tk.get_pos(), tk.get_pos() + 1));
            }
        }

        return Ok(true);
    }
}