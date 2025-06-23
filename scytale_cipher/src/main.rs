use scytale_cipher::*;

fn main() {
    let tt = scytale_cipher(String::from("scytale Code"), 6);
    println!("{:?}",tt);
    // output :  "sec yCtoadle"
    let vv = scytale_cipher("sec yCtoadle".to_owned(), 6);
    println!("{:?}",vv);

}

#[test]
fn test_scytale_cipher() {
    scytale_cipher(String::from("attack morning"), 6);
    assert_eq!(
        &scytale_cipher(String::from("scytale Code"), 6),
        "sec yCtoadle"
    );
    assert_eq!(
        &scytale_cipher(String::from("scytale Code"), 8),
        "sCcoydtea l e"
    );
    // nothing
    assert_eq!(&scytale_cipher(String::from(""), 4), "");
    // same len
    assert_eq!(
        &scytale_cipher(String::from("qwerty qwerty"), 13),
        "qwerty qwerty"
    );
    // different cylinder
    assert_eq!(
        &scytale_cipher(String::from("attack morning"), 6),
        "a ntmgto ar cn ki"
    );
}