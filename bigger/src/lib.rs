use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut big = 0;
    for val in h.values() {
        if *val > big {
            big = *val;
        }
    }
    big
}