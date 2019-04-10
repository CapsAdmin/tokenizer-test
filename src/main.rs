mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let tk = Tokenizer::new("function ");

    for token in tk.read_all() {
        println!("{}⸢{}⸥", token.type_name, tk.get_chars(token.start as usize, token.stop as usize));
    }

    
    Tokenizer::new("  \na\ndawd").assert_types(vec!["space", "identifier", "space", "identifier"]);
    //Tokenizer::new("  \na\ndawd1").assert_values(vec!["   \n", "a", "\n", "dawd1"]);

    if Tokenizer::new("function ").read_all()[1].type_name == "space" {
        println!("OK!");
    }
}