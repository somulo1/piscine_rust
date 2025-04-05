use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut frequency_map = HashMap::new();
    
    for &word in words {
        // The entry API lets us insert or update the count
        let count = frequency_map.entry(word).or_insert(0);
        *count += 1;
    }
    
    frequency_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    // The number of distinct words is simply the number of entries in the HashMap
    frequency_count.len()
}