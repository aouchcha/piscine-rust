pub fn arrange_phrase(phrase: &str) -> String {
    let slice: Vec<&str> = phrase.split(" ").collect();
    let mut holder: Vec<&str> = vec![""; slice.len()];
    let mut res = String::new();
    for s in slice {
        for c in s.chars() {
            if c.is_numeric() {
                holder[((c as u8 - 48) - 1) as usize] = s;
                continue;
            }
        }
    }
    for i in 0..holder.len() {
        for c in holder[i].chars() {
            if !c.is_numeric() {
                res.push(c);
            }
        }
        if i < holder.len()-1 {
            res.push_str(" ");
        }
    }
    res
}
