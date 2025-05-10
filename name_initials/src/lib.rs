pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut test : Vec<String> = vec![];
    for i in names {
       test.push(split(i));
    }
    test
}
pub fn split(text :&str) -> String {
    let mut temp = String::new();
    for (i, item )in text.chars().enumerate() {
        if i == 0 {
            temp.push_str(&(item.to_string() + ". "));
        }else if item == ' ' {
            temp.push_str(&(text[i+1..i+2].to_string() + "."));
        }
    }
    temp
}