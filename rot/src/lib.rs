pub fn rotate(input: &str, key: i8) -> String {
   rot_n(input, key)
}

fn rot_n(text: &str, n: i8) -> String {
    let mut temp = n;
    if n < 0 {
        temp = 26 + n
    }
    let mut result = String::new();
    
    for c in text.chars() {
        if c >= 'a' && c <= 'z' {
            let shifted = ((c as u8 - 97) as i8 + temp) % 26;
            let new_char = (97 + shifted as u8) as char;
            result.push(new_char);
        } else if c >= 'A' && c <= 'Z' {
            let shifted = ((c as u8 - 65) as i8 + temp) % 26;
            let new_char = (65 + shifted as u8) as char;
            result.push(new_char);
        } else {
            result.push(c);
        }
    }
    
    result
}