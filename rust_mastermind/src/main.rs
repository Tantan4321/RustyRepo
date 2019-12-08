use std::io;
use rand::Rng;
use std::process::exit;

fn main() {


    println!("Let's play mastermind! Your goal is to guess my number.");

    let stdin = io::stdin();
    let mut rng = rand::thread_rng();

    let secret: [u16; 6] = [
        rng.gen_range(0, 10),
        rng.gen_range(0, 10),
        rng.gen_range(0, 10),
        rng.gen_range(0, 10),
        rng.gen_range(0, 10),
        rng.gen_range(0, 10),
    ];
    loop{
        let mut guess = String::new();

        println!("Please guess a 6 digit number or type 'quit'.");
        stdin.read_line(&mut guess).expect("Input failed");
        let guess = guess.trim();

        if guess == "quit".to_string() {
            println!("Quitting...");
            for num in secret.iter(){
                print!("{}", num);
            }
            exit(0);
        }
    }




/*
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
    */
}
