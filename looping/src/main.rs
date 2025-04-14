use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
<<<<<<< HEAD
    let answer = "the letter e";
    let mut attempts = 0;
    
    loop {
        // Print the riddle
        println!("{}", riddle);
        
        // Get user input
=======
    let answer = "The letter e";
    let mut trial_count = 0;

    loop {
        trial_count += 1;
        println!("{}", riddle);
        
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
<<<<<<< HEAD
        
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
=======
        if guess.trim().to_lowercase() == answer.to_lowercase() {
            println!("Number of trials: {}", trial_count);
            break;
        }
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
