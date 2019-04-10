mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let tk = Tokenizer::new("test123 0b10101 function 0xdeadbeef 0o21312 dawdw 0.0.2  
    
    d ad wasd ");

    tk.read_all();
}