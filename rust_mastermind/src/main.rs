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
        let mut input = String::new();

        println!("Please guess a 6 digit number or type 'quit' to stop playing.");
        stdin.read_line(&mut input).expect("Input failed");
        let input = input.trim();

        if input == "quit".to_string() {
            println!("Quitting...");
            print!("Secret number was: ");
            for num in secret.iter(){
                print!("{}", num);
            }
            exit(0);
        }
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            },
        };
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
