mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let tk = Tokenizer::new("test123   dawdw  d ad w");

    tk.read_all();
}