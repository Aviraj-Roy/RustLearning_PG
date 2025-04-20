use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the no.!(between 1 & 100)");

    let secret_number: i32 = random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        let _ = io::stdin()
            .read_line(&mut guess) //giving mutable reference
            .expect("Failed to read user input");

        /*let guess: i32 = guess
        .trim() //shadowing //also trimming any new line, spaces b4 & after
        .parse()
        .expect("Please type a number!");*/

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid Input");
                continue;
            }
        };

        //println!("You guessed - {}", guess);
        println!("You guessed - {guess}");

        /*match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                println!("Secret No. is - {secret_number}");
                break;
            }
            _ => println!("Try again!"),
        }*/

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                println!("Secret No. is - {secret_number}");
                break;
            }
            Ordering::Greater => {
                println!("Too Big!");
                println!("Try Again");
            }
            Ordering::Less => {
                println!("Too Small!");
                println!("Try Again");
            }
        }
    }
}
