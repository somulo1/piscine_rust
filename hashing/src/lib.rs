use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len();
    sum as f64 / count as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec(); // Clone the list so we don't change the original
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 1 {
        sorted[len / 2] // Odd number of elements
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2 // Even number of elements
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in list {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mut mode = list[0];
    let mut max_count = 0;

    for (&key, &count) in &occurrences {
        if count > max_count {
            max_count = count;
            mode = key;
        }
    }

    mode
}
