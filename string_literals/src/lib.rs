pub fn is_empty(v: &str) -> bool {
    v.is_empty()  // Return true if the string is empty
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()  // Return true if all characters are ASCII
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)  // Return true if the string contains the pattern
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)  // Return a tuple of two slices: before and after the index
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(usize::MAX)  // Return the index of the first occurrence of the character, or usize::MAX if not found
}

