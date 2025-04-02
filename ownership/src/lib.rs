pub fn first_subword(mut s: String) -> String {
    // Remove any leading whitespaces, just in case
    s = s.trim().to_string();

    // Case 1: For snake_case, find the first underscore
    if let Some(pos) = s.find('_') {
        s = s[..pos].to_string();  // Slice before the first underscore
    } 
    // Case 2: For camelCase and PascalCase, find the first uppercase letter
    else if let Some(pos) = s.chars().position(|c| c.is_uppercase()) {
        s = s[..pos].to_string();  // Slice before the first uppercase letter
    }

    s  // Return the first subword
}
