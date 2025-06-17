pub fn get_diamond(c: char) -> Vec<String> {
    let mut num = (c as u8) - 64;
    let mut outerSpaces = 0;
    let mut innerSpaces = 0;
    let mut res : Vec<String> = Vec::new();
    // println!("{}", num);
    for i in 0..num {
    let mut holder = String::new();
        // for j in 65..65+((c as u8) - 64) {}
        // println!("{}", i);
        if i == 0 {
            outerSpaces = num-1;
            // innerSpaces = num - outerSpaces - 1;
            

        } else {
            // num -= 2;
            outerSpaces = num-i - 1;
            innerSpaces = 2*i - 1;
            // println!("{}", outerSpaces);
            // println!("inner {}", innerSpaces);
        }
        let chara = (65 + i) as char;
        if innerSpaces == 0 {
            for i in 0..outerSpaces {
                holder.push(' ');
            }
            holder.push(chara);
            for i in 0..outerSpaces {
                holder.push(' ');
            }
        }else {
            for i in 0..outerSpaces {
                holder.push(' ');
            }
            holder.push(chara);
            for i in 0..innerSpaces {
                holder.push(' ');
            }
            holder.push(chara);
            for i in 0..outerSpaces {
                holder.push(' ');
            }
        }
        res.push(holder)
        // println!("{}", ((i as u8) as char))
    }

    for i in (0..res.len() -1).rev() {
        res.push(res[i].clone());
    }

    // for i in 65..(65+num) as usize{

    // }

    res
}
