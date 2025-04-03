pub fn arrange_phrase(phrase: &str) -> String {
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
