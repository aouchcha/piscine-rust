pub fn num_to_ordinal(x: u32) -> String {
    match x {
        1 => return format!("{}st",x),
        2 => return format!("{}nd",x),
        3 => return format!("{}rd",x),
        4..20 => return format!("{}th",x),
        _ => return above20(x)
    }
    
    
}

fn above20(x: u32) -> String {
    let num_str : Vec<char>= format!("{}",x).chars().collect();
    match num_str[num_str.len() - 1] {
        '1' => return format!("{}st",x),
        '2' => return format!("{}nd",x),
        '3' => return format!("{}rd",x),
       _ => return format!("{}th",x),
    }
}