pub fn char_length(s: &str) -> usize {
    let mut size : usize = 0;
    for _ in s.chars() {
        size += 1;
    } 
    size
}