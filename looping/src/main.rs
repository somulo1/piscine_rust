use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";

    let mut tries = 0;

    loop {
        // Print the riddle
        println!("{}", riddle);

        // Take the user's input
        let mut answer = String::new();
        io::stdout().flush().unwrap(); // Flush to ensure the prompt is printed before taking input
        io::stdin().read_line(&mut answer).unwrap();

        // Trim any excess whitespace or newline characters from the input
        let answer = answer.trim();

        // Increment the trial count
        tries += 1;

        // Check if the answer is correct
        if answer == correct_answer {
            println!("Number of trials: {}", tries);
            break; // Exit the loop once the correct answer is given
        } else {
            // Only print this message when it's an incorrect answer
            println!("Incorrect answer. Try again!\n");
        }
    }
}
