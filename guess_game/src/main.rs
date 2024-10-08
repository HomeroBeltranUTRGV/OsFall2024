// Function to check the user's guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Secret number (hard-coded)
    let secret_number = 42;

    // Variable to track the number of guesses
    let mut guess_count = 0;

    // Loop to simulate guessing
    loop {
        // Simulate user input by setting a mutable guess variable
        let guess = 40 + guess_count; // This simulates user guessing different numbers
        println!("Guessing: {}", guess);

        // Call the check_guess function
        let result = check_guess(guess, secret_number);
        guess_count += 1;

        // Use if-else to determine if the guess was correct, too high, or too low
        if result == 0 {
            println!("Correct! The secret number is {}", secret_number);
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }

    // Print how many guesses it took
    println!("It took {} guesses to find the secret number.", guess_count);
}

