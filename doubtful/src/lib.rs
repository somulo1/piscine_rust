pub fn doubtful(s: &mut String) {
<<<<<<< HEAD
    s.push('?');  // Appends a question mark to the string
}
=======
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubtful() {
        let mut s = String::from("Hello");
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
