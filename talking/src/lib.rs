pub fn talking(text: &str) -> &str {
    if text.trim().len() == 0 {
        return "Just say something!"
    }

    let holder : Vec<char> = text.chars().collect();  
    if is_uppercase(text) {
        if  holder[holder.len() - 1] != '?' {
            return "There is no need to yell, calm down!"
        } else {
            return "Quiet, I am thinking!"
        }
    }

    if holder[holder.len() - 1] == '?' {
        return "Sure."
    }

   

    "Interesting"
}

fn is_uppercase(s: &str) -> bool {
    let mut is_number = false;
    for c in s.chars() {
        if c.is_numeric() {
            is_number = true;
        }else if c.is_alphabetic() {
            is_number = false;
        }
        if c.is_ascii_lowercase() {
            return false
        }
    }
    if is_number {
        return !is_number;
    }
    true
}