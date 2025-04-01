use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";

    let mut tries = 0;

    loop {
        // Print the riddle before each input attempt
        println!("{}", riddle);

        // Take the user's input
        let mut answer = String::new();
       
        io::stdin().read_line(&mut answer).unwrap();

        // Trim the input to remove any extra whitespace or newline characters
        let answer = answer.trim().to_lowercase();

        // Increment the trial count
        tries += 1;

        // If the answer is correct, break the loop and print the number of trials
        if answer == correct_answer {
            println!("Number of trials: {}", tries);
            break; // Exit the loop once the correct answer is given
        }
    }
}
