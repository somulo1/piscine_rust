pub fn divide(x: i32, y: i32) -> (i32, i32) {
<<<<<<< HEAD
    (x / y, x % y)
=======
    match y {
        0 => panic!("Division by zero"),
        _ => (x / y, x % y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_test() {
        let result = divide(9, 4);
        assert_eq!(result, (2, 1));
    }
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}
