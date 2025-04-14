<<<<<<< HEAD
=======
// capitalize_first function which converts the 
// first letter of the string to uppercase.
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

<<<<<<< HEAD
pub fn title_case(input: &str) -> String {
    // Use a different approach to preserve all whitespace
    let mut result = String::new();
    let mut capitalize_next = true;
    
=======

// title_case function which converts the 
// first letter of each word in a string to uppercase.
pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
<<<<<<< HEAD
            result.push_str(&c.to_uppercase().collect::<String>());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

=======
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.extend(c.to_lowercase());
        }
    }
    result
}

// change_case function which converts all uppercase letters to lowercase and vice versa.
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
<<<<<<< HEAD
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
=======
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        let result = capitalize_first("hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_title_case() {
        let result = title_case("hello my\t\tname is carl");
        assert_eq!(result, "Hello My\t\tName Is Carl");
    }

    #[test]
    fn test_change_case() {
        let result = change_case("hElLo wOrLd");
        assert_eq!(result, "HeLlO WoRlD");
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
