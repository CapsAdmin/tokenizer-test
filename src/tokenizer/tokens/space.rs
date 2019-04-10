use crate::tokenizer::*;

pub struct Token;
impl TokenModel for Token {
    fn name(&self) -> &str {
        return "space";
    }
    fn capture(&self, tk: &Tokenizer) -> CaptureResult {
        if !syntax::is_space(tk.get_char()) {
            return Ok(false);
        }

        for _ in tk.iterate() {
            tk.advance(1);
   
            if !syntax::is_space(tk.get_char()) {
                return Ok(true);
            }
        }

        return Ok(true);
    }
}