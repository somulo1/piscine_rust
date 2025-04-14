pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}
<<<<<<< HEAD
=======

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rev_str_test() {
        let result = rev_str("hello");
        assert_eq!(result, "olleh");
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
