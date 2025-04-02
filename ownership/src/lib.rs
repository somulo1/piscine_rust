pub fn first_subword(mut s: String) -> String {
    // Trim any leading or trailing whitespaces from the string
    s = s.trim().to_string();

    // If the string is empty, return an empty string (edge case)
    if s.is_empty() {
        return s;
    }

    // Case 1: Handling snake_case, which contains underscores
    if let Some(pos) = s.find('_') {
        return s[..pos].to_string();  // Return the part before the first underscore
    }

    // Case 2: Handling camelCase or PascalCase, looking for the first uppercase letter
    if let Some(pos) = s.chars().position(|c| c.is_uppercase()) {
        return s[..pos].to_string();  // Return the part before the first uppercase letter
    }

    // Case 3: If no underscore or uppercase letter is found, it's a single word
    // Just return the entire word, as there is no splitting needed
    s
}
