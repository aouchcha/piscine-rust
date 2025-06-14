mod edit_distance;
use crate::edit_distance::*;

pub fn expected_variable(source: &str, target: &str) -> Option<String> {
    if !is_camel(source) && !is_snake(source) {
        return None
    }
    let num = edit_distance(&source.to_lowercase(), &target.to_lowercase());
    let l = target.len();
    let like = 100 - ((num as u32 * 100) / l as u32);
    if like < 50 {
        return None
    }
    Some(format!("{}%", like))
}

fn is_camel(s: &str) -> bool {
    let holder : Vec<char>  = s.chars().collect();
    for i in 1..holder.len() {
        if holder[i].is_uppercase() {
            return true
        }
        if holder[i].is_whitespace() {
            return false
        }
    }
    false
}

fn is_snake(s: &str) -> bool {
    s.contains("_")
}