pub fn talking(text: &str) -> &str {
    println!("{:?} {:?} {:?}", text, is_uppercase(text), text.contains("?"));
    if is_uppercase(text) && text.contains("?") {
        return "Quiet, I am thinking!"
    }else if !is_uppercase(text) && text.chars().nth(text.len()-1).unwrap() == '?' {
        return "Sure."
    }else if text.trim().len() == 0 {
        return "Just say something!"
    }else if is_uppercase(text) {
        return  "There is no need to yell, calm down!"
    }
    "Interesting"
}

pub fn is_uppercase(s: &str) -> bool {
    for c in s.chars() {
        if c >= 'a' && c <= 'z' || c >= '0' && c <= '9'{
            return false
        }
    }
    true
}