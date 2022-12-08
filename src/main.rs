use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MIN: u32 = 1;
const MAX: u32 = 10;

fn main() {
    let num = rand::thread_rng().gen_range(MIN..=MAX);

    let max_guesses = 5;
    let mut i = 0;
    while i < max_guesses {
        let guesses_left = max_guesses - i;
        println!("Guess a number between {MIN} and {MAX}.");
        println!("You have {guesses_left} guesses left. Input a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Input failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Less => {
                println!("That guess is too small");
                i += 1;
            }
            Ordering::Greater => {
                println!("that guess is too large");
                i += 1;
            }
            Ordering::Equal => {
                i += 1;
                println!("You win! The number was {num} and you got it in {i} guesses");
                break;
            }
        }
    }
    if i == 5{
        println!("Out of guesses! The number was {num}.");
    }

    println!("Press enter to exit.");

    let mut exit = String::new();
    io::stdin().read_line(&mut exit).expect("Failed to read line");

}
