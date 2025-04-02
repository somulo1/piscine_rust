pub fn first_subword(mut s: String) -> String {
    // Trim any leading or trailing whitespaces (edge case)
    s = s.trim().to_string();

    // Case 1: Handling snake_case, which contains underscores
    if let Some(pos) = s.find('_') {
        s = s[..pos].to_string();  // Take everything before the first underscore
        return s;  // Return the modified string
    }

    // Case 2: Handling camelCase or PascalCase, looking for the first uppercase letter
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            s = s[..i].to_string();  // Take everything before the first uppercase letter
            return s;  // Return the modified string
        }
    }

    // Case 3: If no underscore or uppercase letter is found, it's a single word
    s  // Return the entire word
}
