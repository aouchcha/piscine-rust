pub fn score(s : &str) -> u64{
    let s = s.to_uppercase();
    let mut score : u64 = 0;
    for c in s.chars() {
        match c {
            'A'| 'E'| 'I'| 'O'| 'U'| 'L'| 'N'| 'R'| 'S'| 'T' => score+=1,
            'D'| 'G' => score+=2,
            'B'| 'C'| 'M'| 'P' => score+=3,
            'F'| 'H'| 'V'| 'W'| 'Y' => score+=4,
            'K' => score+=5,
            'J'| 'X' => score+=8,
            'Q'| 'Z' => score+= 10,
            _ => continue
        }
        println!("{c} {score}");
    }
    score
}