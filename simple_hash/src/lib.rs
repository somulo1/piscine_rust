use std::collections::HashMap;

<<<<<<< HEAD
pub fn word_frequency_counter<'a>(words: Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut frequency_map = HashMap::new();
    
    for &word in words.iter() {
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
=======
pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut frequency_count = HashMap::new();
    for word in words {
        *frequency_count.entry(*word).or_insert(0) += 1;
    }
    frequency_count
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency() {
        let words = vec!["hello", "world", "hello", "rust"];
        let freq = word_frequency_counter(&words);
        assert_eq!(freq["hello"], 2);
        assert_eq!(freq["world"], 1);
        assert_eq!(freq.get("missing"), None);
    }

    #[test]
    fn test_distinct_words() {
        let words = vec!["a", "b", "a", "c"];
        let freq = word_frequency_counter(&words);
        assert_eq!(nb_distinct_words(&freq), 3);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
