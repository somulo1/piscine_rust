pub fn first_subword(mut s: String) -> String {
    // Remove any leading whitespaces, just in case
    s = s.trim().to_string();

    // If there's an underscore in the string (snake_case)
    if let Some(pos) = s.find('_') {
        s = s[..pos].to_string();  // Slice before the first underscore
    } 
    // If there's an uppercase letter (camelCase or PascalCase)
    else if let Some(pos) = s.chars().position(|c| c.is_uppercase()) {
        s = s[..pos].to_string();  // Slice before the first uppercase letter
    }

    // If no special case was found, return the entire string (single word)
    if s.is_empty() {
        s = s.trim().to_string();  // In case trimming resulted in an empty string
    }

    s  // Return the first subword
}
