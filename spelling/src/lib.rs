pub fn spell(n: u64) -> String {
    if n < 0 || n > 1000000 {
        return String::new()
    } 
    if n >= 0 && n <= 19 {
        return one_to_nineten(n)
    } else if n >= 20 && n <= 99 {
        return twenty_to_hundred(n)
    }else if n >= 100 && n <= 999 {
        return one_hundrend_to_nine_hundrend(n)
    }else if n >= 1000 && n <= 999999 {
        return one_thousand_to_nine_thousand(n)
    }
    String::from("one million")
}

fn one_to_nineten(n: u64) -> String {
    let res = match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    };
    res
}


fn twenty_to_hundred(n: u64) -> String {
    let first_part = match n/10 {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string()
    };
    if n%10 == 0 {
        return first_part
    };
    let second_part = one_to_nineten(n%10);
    format!("{}-{}",first_part, second_part)
}

fn one_hundrend_to_nine_hundrend(n: u64) -> String {
    // println!("{}",n/100);

    let first_part = match n/100 {
        1 => "one hundred".to_string(),
        2 => "two hundred".to_string(),
        3 => "three hundred".to_string(),
        4 => "four hundred".to_string(),
        5 => "five hundred".to_string(),
        6 => "six hundred".to_string(),
        7 => "seven hundred".to_string(),
        8 => "eight hundred".to_string(),
        9 => "nine hundred".to_string(),
        _ => "".to_string()
    };
    
    // println!("{}",n%100);
    if n%100 == 0 {
        // println!("hanni");
        return first_part
    }
    let second_part = spell(n%100);

    format!("{} {}", first_part, second_part)
}

fn one_thousand_to_nine_thousand(n: u64) -> String {
    let first_part = match n/1000 {
        1 => "one thousand".to_string(),
        2 => "two thousand".to_string(),
        3 => "three thousand".to_string(),
        4 => "four thousand".to_string(),
        5 => "five thousand".to_string(),
        6 => "six thousand".to_string(),
        7 => "seven thousand".to_string(),
        8 => "eight thousand".to_string(),
        9 => "nine thousand".to_string(),
        _ => format!("{} thousand", spell(n/1000))
    };

    if n%1000 == 0 {
        return first_part
    }

    let second_part = spell(n%1000);
    format!("{} {}", first_part, second_part)
}