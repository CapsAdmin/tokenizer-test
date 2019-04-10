pub mod space;
pub mod identifier;
pub mod number;
pub mod keyword;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! register {
        ($name: ident) => {
            pub struct Token;
            impl TokenModel for Token {
                fn name(&self) -> String {
                    return String::from(stringify!($name));
                }
                fn capture_internal(&self, tk: &Tokenizer) -> bool {
                    return self.capture(tk);
                }
            }
        }
    }
}