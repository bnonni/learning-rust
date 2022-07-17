use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // thread_rng: uses a particular random number generator that is
    //             local to the current thread of execution and seeded by the OS
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /* match the type u32 against the guess to determine if guess is u32 */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        /* match [ the value here ] to { something in here } */
        /* valueToCheck.compare(patternToChecKAgainst) */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
