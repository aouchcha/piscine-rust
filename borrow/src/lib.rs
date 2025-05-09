pub fn str_len(s: &str) -> usize {
    let mut len : usize = 0;
    for _i in s.chars() {
        len+=1;
    }
    len
}