use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map = HashMap::new();
    let ss = s.to_lowercase();
    for c in ss.chars() {
        if c>= 'a' && c <='z' {
            println!("{c}");
            let count = map.entry(c).or_insert(1);
            *count += 1;
        }
    }
    map.len() == 26
}