use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess between 1 and 100.");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // match  guess.cmp(secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => println!("You guessed it!"),
        // }

        // convert the string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                return;
            }
        };

        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);

        if guess == secret_number {
            println!("You guessed it!");
        } else {
            println!("Better luck next time!");
        }
    }
}
