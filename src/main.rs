use std::io;


fn main() {
    println!("Guess the number!");

    println!("Input Dude:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");
        
        println!("You got it {guess}")
}
