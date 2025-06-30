use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::random_range(1..100);

    // println!("Secret Number: {}", secret_number);
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess_count = 0;
    loop {
        guess_count += 1;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // println!("You guessed '{}'", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small.".red()),
            Ordering::Greater => println!("{}", "Too Big.".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                println!("Score: {} guesses.", guess_count);
                break;
            }
        }
    }
}
