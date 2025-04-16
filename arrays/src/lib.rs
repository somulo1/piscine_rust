pub fn sum(a: &[i32]) -> i32 {
<<<<<<< HEAD
    a.iter().sum()  // Sums all elements in the array
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]  // Array of 32 elements all initialized to 10
}


=======
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let a = [1, 2, 3];
        let result = sum(&a);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_thirtytwo_tens() {
        let result = thirtytwo_tens();
        assert_eq!(result.len(), 32);
        assert_eq!(result.iter().sum::<i32>(), 320);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let a = [1, 2, 3];
        let result = sum(&a);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_thirtytwo_tens() {
        let result = thirtytwo_tens();
        assert_eq!(result.len(), 32);
        assert_eq!(result.iter().sum::<i32>(), 320);
    }
}
