pub fn sum(a: &[i32]) -> i32 {
    let mut r : i32 = 0;
    for num in a {
        r += num;
    }
    r
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10;32]
}