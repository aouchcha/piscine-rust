pub fn first_subword(mut s: String) -> String {
    let mut index = 0;
    for (i, item) in s.chars().enumerate() {
        if (item > 'z' || item < 'a'){
            if i == 0 {
                continue;
            }
            index = i;
            break;
        }
    }
    if index != 0 {
        return s[..index].to_string();
    }
    s[..].to_string()
}