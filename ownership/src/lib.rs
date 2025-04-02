pub fn first_subword(s: String) -> String {
    // Case 1: Handling snake_case, which contains underscores
    if let Some(pos) = s.find('_') {
        return s[..pos].to_string();  // Return the part before the first underscore
    }

    // Case 2: Handling camelCase or PascalCase, looking for the first uppercase letter
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            return s[..i].to_string();  // Return the part before the first uppercase letter
        }
    }

    // Case 3: If no underscore or uppercase letter is found, it's a single word
    s  // Return the entire string if no cases apply
}
