mod tokenizer;

fn main() {
    let tk = tokenizer::Tokenizer::new("test123   dawdw  d ad w");

    tk.read_all();
}