pub fn pig_latin(text: &str) -> String {
    if is_start_with_vowel(text.to_lowercase()) {
        return format!("{}ay",text)
    } else {
        return format!("{}ay",get_the_index(text.to_lowercase()));
    }
    String::new()
}

fn is_start_with_vowel(s: String) -> bool {
    let vowels = "aeuio";
    let first = s.chars().nth(0).unwrap();
    vowels.contains(first)
}

fn get_the_index(s: String) -> String {
    let holder: Vec<char> = s.chars().collect();
    let vowels = "aeuio";
    let mut index: usize = 0;
    let mut res = String::new();
    for i in 1..holder.len() {
        if vowels.contains(holder[i]) && holder[i-1] == 'q' && i != 1 {
            index = i+1;
            break
        }
        else if vowels.contains(holder[i]){
            index = i;
            break
        }
    }

    for i in index..holder.len() {
       res.push(holder[i]);
    }
    for i in 0..index {
       res.push(holder[i]);
    }
    res
}