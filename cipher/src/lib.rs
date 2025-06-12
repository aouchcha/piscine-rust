#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut holder : String = String::new();
    for c in original.chars() {
        if c.is_lowercase() {
            holder.push((122 - (c as u8 -  97)) as char);
        }else if c.is_uppercase() {
            holder.push((90 - (c as u8 -  65)) as char);
        }else {
            holder.push(c)
        }
    }
    if &holder != ciphered {
        return Err(CipherError {
            expected : holder
        })
    }
    Ok(())
}