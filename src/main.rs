use std::{io, cmp::Ordering};
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Input Dude:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");
        
        println!("You guessed {guess}");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    
    loop {
        println!("Please input your guess:");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To much!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
