pub fn mean(list: &[i32]) -> f64 {
    let mut result : f64 = 0.0;
    for num in list {
        result += *num as f64;
    }
    result /= list.len() as f64;
    result
}

pub fn median(list: &[i32]) -> i32 {
    let mut hh = list.to_vec(); 
    hh.sort();
    if hh.len() % 2 != 0 {
        return hh[hh.len()/2]
    }
    (hh[hh.len()/2]+hh[hh.len()/2 -1]) / 2
}
use std::collections::HashMap;
pub fn mode(list: &[i32]) -> i32 {
    let mut result = 0;
    let mut temp = 0; 
    let mut map = HashMap::new();
    for num in list {
        let count = map.entry(*num).or_insert(0);
        *count += 1
    }
    for (key, value) in &map {
        if *value > temp {
            temp = *value;
            result = *key;
        }
    }
    result
}