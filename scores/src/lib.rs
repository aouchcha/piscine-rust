pub fn score(s: &str) -> u64 {
    let mut res : u64 = 0;
    for c in s.to_uppercase().chars() {
        if c.is_alphabetic() {
            if c == 'K' {
                res += 5;
            } else if c == 'D' || c == 'G' {
                res += 2;
            } else if c == 'J' || c == 'X' {
                res += 8;
            } else if c == 'Q' || c == 'Z' {
                res += 10;
            } else if c == 'B' || c == 'C' || c == 'M' || c == 'P' {
                res += 3;
            } else if c == 'F' || c == 'H' || c == 'V' || c == 'W' || c == 'Y' {
                res += 4;
            } else if c == 'A' || c == 'E' || c == 'U' || c == 'I' || c == 'O' || c == 'L' || c == 'N' || c == 'S' || c == 'R' || c == 'T'{
                res += 1;
            }
        }
    }
    res
}