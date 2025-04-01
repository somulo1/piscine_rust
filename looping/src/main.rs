use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";
    let mut attempts = 0;
    
    loop {
        // Print the riddle
        println!("{}", riddle);
        
        // Get user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Remove trailing newline and convert to lowercase for comparison
        let guess = guess.trim().to_lowercase();
        attempts += 1;
        
        // Check if answer is correct
        if guess == answer {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}