<<<<<<< HEAD
use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = f64::exp(c as f64); // Exponential of the value
    let ln_value = if c == 0 { f64::NEG_INFINITY } else { f64::ln(c.abs() as f64) }; // Natural log of the absolute value, handle 0 case
    (c, exp_value, ln_value)
}

pub fn str_function(a: String) -> (String, String) {
    let values: Vec<f64> = a
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| f64::exp(n as f64)) // Exponential of each number
        .collect();

    let exp_string = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" "); // Join the exp results into a single string

    (a, exp_string)
=======
// Instructions

// Create the following functions. The objective is to know how ownership works with different types.

//     nbr_function returns a tuple:
//         with the original value.
//         the exponential function of the value.
//         and the natural logarithm of the absolute value.
//     str_function returns a tuple:
//         with the original value.
//         and the exponential function of each value as a string (see the example).
//     vec_function returns a tuple:
//         with the original value.
//         and the natural logarithm of each absolute value.


pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exponential = (c as f64).exp();
    let natural_logarithm = (c.abs() as f64).ln();
    (c, exponential, natural_logarithm)
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<f64> = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    
    let exp_numbers: Vec<String> = numbers
        .iter()
        .map(|&n| n.exp().to_string())
        .collect();
    
    (a, exp_numbers.join(" "))
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b
        .iter()
<<<<<<< HEAD
        .map(|&x| f64::ln(x.abs() as f64)) // Log of the absolute value of each element
        .collect();

    (b, ln_values)
}
=======
        .map(|&n| (n.abs() as f64).ln())
        .collect();
    
    (b, ln_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        let inf = f64::NEG_INFINITY;
        let result = nbr_function(0);
        assert_eq!(result, (0, 1.0, inf));
        // let result = str_function("1 2 4 5 6".to_string());
        // assert_eq!(result, ("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351"));
        // let result = vec_function(vec![1, 2, 3, 4, 5]);
        // assert_eq!(result, ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
