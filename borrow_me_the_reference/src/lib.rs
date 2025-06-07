/*
"bpp--o+er++++ssskroi-++lcw"
"borrow"
*/
pub fn delete_and_backspace(s: &mut String) {
    let mut index = 0;
    let mut holder = String::new();
    let mut jump = false;
    for (_, c) in s.chars().enumerate() {
        println!("{}",index);
        if jump && (c != '-' && c != '+'){
            index -=1;
            if index == 0 {
                jump = false;
            }
            continue;
        }
        if c == '-' {
            holder.pop();
        }
        else if c == '+' {
            index += 1;
            jump = true;
        }else {
            holder.push(c)
        }
    }
    *s = holder;
}

pub fn do_operations(v: &mut [String]) {
    for i in 0..v.len() {
        if v[i].contains("+") {
            let slice: Vec<&str> = v[i].split("+").collect();
            if slice.len() == 2 {
                v[i] = add(
                    slice[0].parse::<i32>().unwrap(),
                    slice[1].parse::<i32>().unwrap(),
                )
                .to_string();
            }
        } else if v[i].contains("-") {
            let slice: Vec<&str> = v[i].split("-").collect();
            if slice.len() == 2 {
                v[i] = sub(
                    slice[0].parse::<i32>().unwrap(),
                    slice[1].parse::<i32>().unwrap(),
                )
                .to_string();
            }
        }
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
