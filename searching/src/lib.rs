pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = i+1;
    while i < array.len() {
        while j < array.len()-1 {
            if array[i] == array[j] {
                return None
            }
            j+=1;
        }
        i+=1;
    }

   for num in 0..array.len() {
       if array[num] == key {
        return Some(num)
       }
    }
    None
}