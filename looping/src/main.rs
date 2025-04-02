use std::io;

fn main() -> io::Result<()> {
    // The riddle and the correct answer
    let riddl = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
                  let riddle = riddl.trim();
    let correct_answer = "The letter e";

    let mut attempts = 0;
    let mut input = String::new();

    // Start a loop that will continue until the correct answer is given
    loop {
        // Print the riddle
        println!("{}", riddle);

        // Read the user's input
        input.clear(); // Clear the previous input
        io::stdin().read_line(&mut input)?;

        // Increment the attempt counter
        attempts += 1;

        // Check if the input is correct
        if input.trim() == correct_answer.trim() {
            println!("Number of trials: {} ", attempts);
            break;
        } 
    }

    Ok(())
}
