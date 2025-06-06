pub fn rev_str(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars().rev() {
        res.push(c);
    }
    res
}