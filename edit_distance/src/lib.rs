use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let s1 : Vec<char> = source.chars().collect();
    let s2 : Vec<char> = target.chars().collect();
    let n = source.len();
    let m = target.len();
    return edit_distance_recu(s1, s2, n, m)
}

fn edit_distance_recu(s1 : Vec<char>, s2 : Vec<char>, n: usize, m: usize) -> usize {
    if n == 0 { return m}
    if m == 0 { return n}

    if s1[n-1] == s2[m-1] {
        return edit_distance_recu(s1.clone(), s2.clone(), n-1, m-1)
    }

    return 1+ min(edit_distance_recu(s1.clone(),s2.clone(),n, m-1), min(
        edit_distance_recu(s1.clone(),s2.clone(),n-1,m), edit_distance_recu(s1.clone(), s2.clone(), n-1, m-1)
    ))
}