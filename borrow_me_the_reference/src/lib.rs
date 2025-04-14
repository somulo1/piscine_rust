<<<<<<< HEAD
pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len() ;	
	while i  > 0 {
		i -= 1;
        if chars[i] == '+' {
			chars.remove(i); 
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }	
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i); 
        }  else {
            i += 1;
        }
    }
	
    *s = chars.into_iter().collect();
}


pub fn do_operations(v: &mut [String]) {
    for op in v.iter_mut() {
        let parts: Vec<&str> = op.split(|c| c == '+' || c == '-').collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();
        
        // Determine the operator
        let result = if op.contains('+') {
            left + right
        } else {
            left - right
        };
        
        *op = result.to_string();
=======
// Create the following functions:

//     delete_and_backspace: which receives a borrowed string, and processes it.
//   - represents the backspace key and + represents the delete key, so that "helll-o" 
//      and "he+lllo" are both converted to "hello". The - and + characters should be removed from the string.

//     do_operations: which borrows a vector of string literals representing simple addition
//      and subtraction equations. The function should replace the operation with the result.


pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut skip_count = 0;

    while let Some(c) = chars.next() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match c {
            '-' => {
                result.pop(); // Remove the last character for backspace
            }
            '+' => {
                // Count consecutive '+' signs
                skip_count += 1;
                while let Some(next_char) = chars.peek() {
                    if *next_char == '+' {
                        skip_count += 1;
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Skip characters equal to the number of consecutive '+' signs
            }
            _ => {
                result.push(c); // Add current character to result
            }
        }
    }

    *s = result;
}
pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(pos) = s.find('+') {
            let left = s[..pos].parse::<i32>().unwrap();
            let right = s[pos+1..].parse::<i32>().unwrap();
            *s = (left + right).to_string();
        } else if let Some(pos) = s.find('-') {
            let left = s[..pos].parse::<i32>().unwrap();
            let right = s[pos+1..].parse::<i32>().unwrap();
            *s = (left - right).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        // Basic cases
        let mut s = "bpp--o+er+++sskroi-++lcw".to_string();
        delete_and_backspace(&mut s);
        assert_eq!(s, "borrow");
    
    }
    
    #[test]
    fn test_do_operations() {
        // Basic operations
        let mut equations = [
            "2+2".to_string(),
            "10-3".to_string(),
            "5+5".to_string(),
            "20-10".to_string(),
        ];
        do_operations(&mut equations);
        assert_eq!(equations, ["4", "7", "10", "10"]);
    
        // Single-digit operations
        let mut equations = [
            "1+1".to_string(),
            "9-4".to_string(),
        ];
        do_operations(&mut equations);
        assert_eq!(equations, ["2", "5"]);
    
        // Larger numbers
        let mut equations = [
            "100+200".to_string(),
            "500-100".to_string(),
        ];
        do_operations(&mut equations);
        assert_eq!(equations, ["300", "400"]);
    
        // Edge case (empty string would panic, so not included)
        let mut equations = ["0+0".to_string()];
        do_operations(&mut equations);
        assert_eq!(equations, ["0"]);
    }
    
    #[test]
    #[should_panic]
    fn test_invalid_operations() {
        let mut equations = [
            "2+".to_string(),  // Invalid format
            "abc".to_string(), // Non-numeric
        ];
        do_operations(&mut equations);
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    }
}