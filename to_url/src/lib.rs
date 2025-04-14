<<<<<<< HEAD
pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")  // Replace every space with "%20"
}
=======
// To do
// Create a function named to_url which takes a 
// string and substitutes every ASCII space with "%20".

pub fn to_url(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == ' ' {
            result.push_str("%20");
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_url("Hello World");
        assert_eq!(result, "Hello%20World");
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
