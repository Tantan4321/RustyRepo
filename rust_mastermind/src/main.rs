use std::io;

fn main() {
    println!("Guess my number! It's between 1 and 10 inclusive!");

    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).expect("Input failed");

    println!("Your guess: {}", buffer);
}
