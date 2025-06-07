pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    let mut holder = String::new();
    let mut index = 0;
    for name in names {
        for c in name.chars() {
            if c == ' ' {
                index = 0;
                continue;
            }
            if index == 0 && holder.len() == 0{
                holder.push_str(&format!("{}. ",c));
            }else if index == 0 {
                holder.push_str(&format!("{}.",c));
            }
            index += 1;
        }
        index = 0;
        result.push(holder);
        holder = String::new()
    }
    result
}