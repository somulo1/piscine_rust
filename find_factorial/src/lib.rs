pub fn factorial(num: u64) -> u64 {
<<<<<<< HEAD
    if num == 0 || num == 1 {
        return 1;
    }

    let mut result = 1;
    for i in 2..=num {
        result *= i;
    }
    result
}


=======
    if num <= 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        let result = factorial(0);
        assert_eq!(result, 1);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
