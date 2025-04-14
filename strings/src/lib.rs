pub fn char_length(s: &str) -> usize {
<<<<<<< HEAD
    s.chars().count()  // Count the number of characters (Unicode scalar values) in the string
}
=======
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_length() {
        let result = char_length("hello");
        assert_eq!(result, 5);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
