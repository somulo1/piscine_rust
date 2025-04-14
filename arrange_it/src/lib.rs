pub fn arrange_phrase(phrase: &str) -> String {
<<<<<<< HEAD
    let mut words: Vec<(&str, u8)> = phrase
        .split_whitespace()
        .map(|word| {
            let num = word.chars().find(|c| c.is_digit(10)).unwrap_or('0');
            (word, num.to_digit(10).unwrap() as u8)
        })
        .collect();
    
    words.sort_by_key(|&(_, num)| num);
    
    words.iter().map(|&(word, _)| {
        word.chars().filter(|c| !c.is_digit(10)).collect::<String>()
    }).collect::<Vec<String>>().join(" ")
}
=======
    // Split into words and collect into a vector
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Sort by the number found in each word
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_ascii_digit())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0)
    });
    
    // Remove numbers from words and join
    words.iter()
        .map(|word| word.chars().filter(|c| !c.is_ascii_digit()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
