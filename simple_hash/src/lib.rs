use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();
    for word in words {
        let counter = map.entry(*word).or_insert(0 as usize);
        *counter += 1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    // let mut number : usize = 0;
    // for (_, _) in frequency_count {
    //     number += 1;
    // }
    // number
    frequency_count.len()
}