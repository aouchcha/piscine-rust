pub fn to_url(s: &str) -> String {
    let mut result  = String::new();
    for i in s.chars() {
        if i != ' ' {
            result.push(i);
        }else {
            result.push_str("%20");
        }
    }
    result
}