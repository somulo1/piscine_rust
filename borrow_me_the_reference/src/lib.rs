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
    }
}