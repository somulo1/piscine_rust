pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)                             // Infinite iterator starting from 1
        .filter(|x| x % 2 == 0)       // Keep only even numbers
        .map(|x| x * x)               // Square them
        .take(50)                     // Take the first 50
        .collect()                    // Collect into a Vec<i32>
}
