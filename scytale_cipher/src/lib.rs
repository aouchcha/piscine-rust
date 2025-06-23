pub fn scytale_cipher(message: String, i: u32) -> String {
    if message.len() == 0 || message.len() == (i as usize){
        return message
    }
    let size = (message.len() as f64/i as f64).ceil() as usize;
    let mut holder : Vec<Vec<char>> = vec![vec![' '; i as usize];size];
    let mut res : String = String::new();
    let mut ii: usize = 0;
    let mut j: usize = 0;

    for c in message.chars().rev() {
        if j == i as usize {
            j = 0;
            ii += 1;
        }
        holder[ii][j] = c;
        j += 1;

    }
   
    for j in 0..i as usize {
        for i in 0..size {
            res.push(holder[i][j])
        }
    }
   
    res.trim().to_owned()
}