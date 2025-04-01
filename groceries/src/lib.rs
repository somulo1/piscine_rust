pub fn insert(vec: &mut Vec<String>, val: String) {
    // Insert the new element at the end of the Vec
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    // Return the element at the given index from the slice
    &slice[index]
}
