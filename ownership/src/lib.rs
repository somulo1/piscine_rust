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
}