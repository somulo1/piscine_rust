pub fn insert(vec: &mut Vec<String>, val: String) {
<<<<<<< HEAD
    // Insert the new element at the end of the Vec
=======
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
<<<<<<< HEAD
    // Return the element at the given index from the slice
    &slice[index]
}
=======
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut vec = Vec::new();
        insert(&mut vec, "apple".to_string());
        assert_eq!(vec[0], "apple");
    }
    #[test]
    fn at_index_test() {
        let vec = vec!["apple".to_string(), "banana".to_string()];
        let result = at_index(&vec, 1);
        assert_eq!(result, "banana");
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
