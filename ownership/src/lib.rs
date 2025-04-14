<<<<<<< HEAD
pub fn first_subword(mut s: String) -> String {
    // Trim any leading or trailing whitespaces
    s = s.trim().to_string();
    
    // Case 1: Handle snake_case with underscores
    if let Some(pos) = s.find('_') {
        return s[..pos].to_string();
    }
    
    // Case 2: Handle camelCase or PascalCase
    // Find the second uppercase letter for PascalCase or first uppercase for camelCase
    let mut prev_is_upper = false;

    for (i, c) in s.char_indices().skip(1) {  // Skip the first character
        if c.is_uppercase() {
            // If we found an uppercase letter, return everything before it
            return s[..i].to_string();
        }
    }
    
    // Case 3: Return the entire string if no subword delimiters found
    s
=======
/// Returns the first sub-word from a string in camelCase, PascalCase, or snake_case format
/// 
/// # Arguments
/// * `s` - A string to extract the first sub-word from
/// 
/// # Examples
/// ```
// let result = first_subword("HelloWorld".to_string()); // Returns "Hello"
// let result = first_subword("hello_world".to_string()); // Returns "hello"
// let result = first_subword("camelCase".to_string()); // Returns "camel"
// ```
pub fn first_subword(mut s: String) -> String {
    // Skip the first character when checking for boundaries
    let boundary = s.char_indices()
        .skip(1)  // ← Key change: ignore first character
        .find(|(_, c)| *c == '_' || c.is_uppercase())  // ← Key change: dereference the char
        .map(|(i, _)| i)
        .unwrap_or_else(|| s.len());
    
    s.truncate(boundary);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        assert_eq!(first_subword("HelloWorld".to_owned()), "Hello");
        assert_eq!(first_subword("camelCase".to_owned()), "camel");
        assert_eq!(first_subword("snake_case".to_owned()), "snake");
    }
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}