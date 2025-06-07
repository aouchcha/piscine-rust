pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(),(c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();
    for c in a.chars() {
        if c.is_numeric() {
            result.push_str( &((c as i8 - 48 ) as f64 ).exp().to_string())
        }else {
            result.push(c);
        }
    }
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res : Vec<f64> = Vec::new();
    for num in &b {
        res.push((num.abs() as f64).ln());
    }
    (b, res)
}