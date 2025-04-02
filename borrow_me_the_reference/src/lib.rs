pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '+' => {
                // Delete key - remove the next character if it exists
                if i + 1 < chars.len() {
                    chars.remove(i + 1);
                }
                // Remove the + character itself
                chars.remove(i);
                // Don't increment i since we've removed characters
            }
            '-' => {
                // Backspace key - remove the previous character if it exists
                if i > 0 {
                    chars.remove(i - 1);
                    // Remove the - character itself (now at position i-1)
                    chars.remove(i - 1);
                    // Move back one position
                    i = i.saturating_sub(1);
                } else {
                    // Just remove the - if it's at the beginning
                    chars.remove(i);
                    // Don't increment i since we've removed a character
                }
            }
            _ => {
                // For any other character, just move to the next position
                i += 1;
            }
        }
    }
    
    // Convert the vector of chars back to a String
    *s = chars.iter().collect();
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