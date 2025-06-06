pub fn str_len(s: &str) -> usize {
    let mut size = 0 as usize;
    for _ in s.chars() {
        size += 1;
    }
    size
}