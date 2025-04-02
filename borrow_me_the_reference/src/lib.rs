pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                // If we encounter a "+", just remove the next character (delete)
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

pub fn do_operations(v: &mut [String]) {
    for op in v.iter_mut() {
        let parts: Vec<&str> = op.split(|c| c == '+' || c == '-').collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();
        
        // Determine the operator
        let result = if op.contains('+') {
            left + right
        } else {
            left - right
        };
        
        *op = result.to_string();  // Replace the string in the vector with the result
    }
}
