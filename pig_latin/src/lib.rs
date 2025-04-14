pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if text.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = text.chars().collect();

    if vowels.contains(&chars[0]) {
        // Starts with vowel -> just add "ay"
        return format!("{}ay", text);
    }

    let mut idx = 0;
    while idx < chars.len() {
        if idx > 0 && chars[idx - 1] == 'q' && chars[idx] == 'u' {
            idx += 1; // Include 'u' after 'q'
            break;
        }
        if vowels.contains(&chars[idx]) {
            break;
        }
        idx += 1;
    }

    let (start, end) = chars.split_at(idx);
    format!("{}{}ay", end.iter().collect::<String>(), start.iter().collect::<String>())
}
