pub fn first_subword(mut s: String) -> String {
    // Remove any leading or trailing whitespaces
    s = s.trim().to_string();

    // If the string contains an underscore (snake_case)
    if let Some(pos) = s.find('_') {
        return s[..pos].to_string();  // Return the part before the first underscore
    }
    
    // If the string contains an uppercase letter (camelCase or PascalCase)
    if let Some(pos) = s.chars().position(|c| c.is_uppercase()) {
        return s[..pos].to_string();  // Return the part before the first uppercase letter
    }

    // If the string doesn't contain either an underscore or uppercase letter,
    // it's likely a single word, so return it as is
    s
}
