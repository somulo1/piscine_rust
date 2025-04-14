<<<<<<< HEAD
pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    
    let source_len = source_chars.len();
    let target_len = target_chars.len();
    
    // Handle edge cases
    if source_len == 0 {
        return target_len;
    }
    if target_len == 0 {
        return source_len;
    }
    
    // Create a matrix to store the distances between all prefixes
    let mut dp = vec![vec![0; target_len + 1]; source_len + 1];
    
    // Initialize the first row and column
    for i in 0..=source_len {
        dp[i][0] = i;
    }
    for j in 0..=target_len {
        dp[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=source_len {
        for j in 1..=target_len {
            // Cost is 0 if characters are the same, 1 otherwise
            let cost = if source_chars[i - 1] == target_chars[j - 1] {
                0
            } else {
                1
            };
            
            // Calculate the minimum of delete, insert, or substitute
            dp[i][j] = (dp[i - 1][j] + 1)            // deletion
                .min(dp[i][j - 1] + 1)               // insertion
                .min(dp[i - 1][j - 1] + cost);       // substitution
        }
    }
    
    // The bottom-right cell contains the edit distance
    dp[source_len][target_len]
}
=======
// Create a function named edit_distance, which calculates the minimum 
// number of changes (insertions, deletions and/or substitutions) which
// are needed to transform the source string to the target string.

pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.is_empty() { return target.len(); }
    if target.is_empty() { return source.len(); }

    let (s_head, s_tail) = source.split_at(1);
    let (t_head, t_tail) = target.split_at(1);

    if s_head == t_head {
        edit_distance(s_tail, t_tail)
    } else {
        1 + edit_distance(source, t_tail) // insert t_head into source
            .min(edit_distance(s_tail, target)) // delete s_head from source
            .min(edit_distance(s_tail, t_tail)) // substitute s_head with t_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        let result = edit_distance("alignment", "assignment");
        assert_eq!(result, 2);
    }
    #[test]
    fn test_edit_distance2() {
        let result = edit_distance("sitting", "kitten");
        assert_eq!(result, 3);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
