use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let num = rand::thread_rng().gen_range(1..=10);

    println!("The number is {num}.");
    let mut i: i32 = 0;
    while i < 5 {
        println!("Input a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Input failed");

        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The guess is {guess}.");

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
                println!("You win!");
                break;
            }
        }
    }
}
