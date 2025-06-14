use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map = HashMap::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            let _count = map.entry(c).or_insert(1);
        }
    }
    map.len() == 26
}