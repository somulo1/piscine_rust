pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '-' => {
                // Backspace: remove the last character from result if it exists
                result.pop();
            },
            '+' => {
                // Delete: skip the next character
                chars.next();
            },
            _ => {
                // Any other character: add to result
                result.push(c);
            }
        }
    }
    
    *s = result;
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
        
        *op = result.to_string();
    }
}