pub fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_letter(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}