pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        
        if c == '+' {
            // Delete key: remove the character after '+'
            s.remove(i); // Remove the '+' itself first
            if i < s.len() {
                s.remove(i); // Then remove the character after it
            }
            // Don't increment i since we've removed characters
        } else if c == '-' {
            // Backspace key: remove the character before '-'
            s.remove(i); // Remove the '-' itself first
            if i > 0 {
                s.remove(i - 1); // Then remove the character before it
                i = i.saturating_sub(1); // Adjust index
            }
        } else {
            // Regular character, just move to the next
            i += 1;
        }
    }
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