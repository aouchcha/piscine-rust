pub fn capitalize_first(input: &str) -> String {
    let mut result : String = String::new();
    for (i,c) in input.chars().enumerate() {
        if i == 0 && c.is_lowercase() {
            result.push((c as u8 - 32) as char)
        }else {
            result.push(c)
        }
    }
    result
}

pub fn title_case(input: &str) -> String {
    let mut result : String = String::new();
    let mut index = 0;
    for c in input.chars() {
        if c.is_whitespace(){
            index = 0;
            result.push(c);
            continue
        }
        else if index == 0 && c.is_lowercase() {
            result.push((c as u8 - 32) as char)
        }else {
            result.push(c)
        }
        index += 1;
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result : String = String::new();
    for c in input.chars() {
        if c.is_lowercase() {
            result.push((c as u8 - 32) as char)
        }else if c.is_uppercase(){
            result.push((c as u8 + 32) as char)
        }else {
            result.push(c)
        }
    }
    result
}