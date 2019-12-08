use std::io;
use rand::Rng;
use std::process::exit;

fn main() {
    println!("Guess my number! It's between 1 and 10 inclusive!");
    let rand_number: u32 = rand::thread_rng().gen_range(1,11);

    let mut guess = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut guess).expect("Input failed");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("Your guess: {}", guess);
    if guess == rand_number {
        println!("Correct! You win!");
    }else {
        println!("Incorrect! The number was {}", rand_number);
    }

    exit(0);
}
