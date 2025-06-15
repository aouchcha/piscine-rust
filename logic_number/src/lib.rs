pub fn number_logic(num: u32) -> bool {
    let num_str: Vec<char> = format!("{}", num).chars().collect();
    let mut res = 0;
    for i in 0..num_str.len() {
        let temp : u32 = (num_str[i] as u8 - 48).into();         
        res += temp.pow(num_str.len() as u32);
    }
    res == num
}