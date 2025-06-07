pub fn first_subword(s: String) -> String {
    for (i,c) in s.chars().enumerate() {
        if (c.is_uppercase() || c == '_') && i != 0 {
            return s[0..i].to_string()
        }
    }
    s
}