use std::io;
use rand::Rng;
use std::process::exit;

fn main() {


    println!("Let's play mastermind! Your goal is to guess my number.");

    let stdin = io::stdin();
    let mut rng = rand::thread_rng();

    let mut secret: Vec<u8> = Vec::new();;
    for i in 0..6{
        secret.push(rng.gen_range(0, 10));
    }

    loop{
        let mut input = String::new();

        println!("Please guess a 6 digit number or type 'quit' to stop playing.");
        stdin.read_line(&mut input).expect("Input failed");
        let input = input.trim();

        //check if user wants to quit
        if input == "quit".to_string() {
            println!("Quitting...");
            print!("Secret number was: ");
            for num in secret.iter(){
                print!("{}", num);
            }
            exit(0);
        }

        //check if input is actually a number
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            },
        };

        //check if input was 6 digits
        if input.len() != 6{
            println!("Not a 6 digit number");
            continue;
        }

        //convert guess number into array
        let guess_arr: Vec<u8> = number_to_vec_u8(guess);

        //initialize reply values
        let mut correct_position: u32 = 0;
        let mut wrong_position: u32 = 0;

        //number right and in correct position
        for i in 0..6{
            if secret[i] == guess_arr[i] {
                correct_position += 1;
            }
        }

        //if all 6 were in correct position, player wins!
        if correct_position == 6{
            println!("Congratulations! You guessed my number!");
            exit(0);
        }
        

    }
}


fn number_to_vec_u8(n: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits.push(n as u8);
    digits.reverse();
    digits
}
