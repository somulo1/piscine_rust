pub fn str_len(s: &str) -> usize {
<<<<<<< HEAD
    s.chars().count()  // Count the number of characters (not bytes)
=======
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = str_len("olá!");
        assert_eq!(result, 4);
    }
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}
