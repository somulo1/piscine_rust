pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                // If we encounter a "+", skip the next character (delete)
                chars.next(); // Skip the next character
            }
            '-' => {
                // If we encounter a "-", remove the last character added to result (backspace)
                result.pop();
            }
            _ => {
                // For any other character, just add it to the result
                result.push(c);
            }
        }
    }
    
    *s = result;  // Update the original string with the processed result
}
