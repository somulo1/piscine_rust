use std::collections::HashMap;

pub fn bigger(hash: HashMap<&str, i32>) -> i32 {
    // Find the maximum value in the HashMap
    *hash.values().max().unwrap_or(&0)
}
