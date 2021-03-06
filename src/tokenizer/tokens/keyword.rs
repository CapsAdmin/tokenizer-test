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
    fn name(&self) -> &str {
        return "keyword";
    }
    fn capture(&self, tk: &Tokenizer) -> CaptureResult {
        for &keyword in &self.array {
            if keyword == tk.get_chars(tk.get_pos(), tk.get_pos() + keyword.char_indices().count()) {
                tk.advance(keyword.char_indices().count() as isize);
                return Ok(true);
            }
        }

        return Ok(false);
    }
}