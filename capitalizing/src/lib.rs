pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),  // If the string is empty, return an empty string
    }
}
pub fn title_case(input: &str) -> String {
    input
        .split_whitespace() // Split by whitespace and remove extra spaces
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first_char) => {
                    // Capitalize the first character and append the rest of the word unchanged
                    first_char.to_uppercase().collect::<String>() + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ") // Join the words back with exactly one space
}


pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}