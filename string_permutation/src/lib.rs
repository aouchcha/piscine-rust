use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false}
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for c in s1.chars() {
        let count = map1.entry(c).or_insert(0);
        *count += 1;
    }

    for c in s2.chars() {
        let count = map2.entry(c).or_insert(0);
        *count += 1;
    }

    for (key, val) in &map1 {
        let value = map2.get(key).copied().unwrap_or(0);
        if value != *val {
            return false
        } 
    }
    true
}