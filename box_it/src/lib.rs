pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let holder : Vec<&str> = s.split_whitespace().collect();
    let mut res : Vec<u32> = Vec::new();
    let mut temp = String::new();
    let mut end_k = false;
    for num_s in holder {
        if num_s.to_string().ends_with("k") {
            temp = num_s[..num_s.len()-1].to_string();
            end_k = true

        }else {
            temp = num_s.to_string();
            end_k = false;
        }

        let mut n : f64 = temp.parse().unwrap();
        if end_k {
            n *= 1000.0
        }
        res.push(n as u32)
    }
    Box::new(res)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}