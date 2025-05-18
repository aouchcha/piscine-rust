pub fn stars(n: u32) -> String {
    let mut r : String = String::new();
    let end = (2 as i32).pow(n);
    for _ in 0..end {
        r.push_str("*")
    }
    r
}