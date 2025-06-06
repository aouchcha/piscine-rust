pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        if (c as i8) < 32 || (c as i8) > 126 {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let size = pat.len();
    for (i, _) in v.chars().enumerate() {
        if i + size < v.len() && &v[i..i + size] == pat {
            return true;
        }
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[0..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    for (i, c) in v.chars().enumerate() {
        if c == pat {
            return i;
        }
    }
    return 0 as usize
}
