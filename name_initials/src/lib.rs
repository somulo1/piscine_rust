pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.into_iter().map(|name| {
        let initials: String = name
            .split_whitespace()  // Split the name by spaces
            .filter_map(|word| word.chars().next())  // Get the first character of each word
            .map(|c| c.to_uppercase().to_string() + ".")  // Convert to uppercase and add a period
            .collect::<Vec<String>>()  // Collect into a vector of Strings
            .join(" ");  // Join the vector of initials with a space between each one

        initials
    }).collect()  // Collect the results into a vector
}
