use std::io;

fn main() {
    let qst = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let answer = String::from("The letter e\n");
    let mut tries = 1;
    loop {
        let mut input = String::new();
        println!("{:?}", qst);
        io::stdin().read_line(&mut input).unwrap();
        if answer == input {
            break;
        } else {
            tries += 1;
        }
    }

    println!("Number of trials: {:?}", tries);
}
