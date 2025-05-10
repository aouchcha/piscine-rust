pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for i in v.chars() {
        if (i.to_ascii_lowercase() as u8 )< 32 || (i.to_ascii_lowercase() as u8) > 128 {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let mut index = 0;
    for _i in v.chars() {
        if v[index..pat.len()] == *pat {
            return true;
        }
        index += 1;
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    let mut index: usize = 0;
    for i in v.chars() {
        if pat == i {
            break
        }
        index += 1
    }
    index
}