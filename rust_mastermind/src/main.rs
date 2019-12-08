use std::io;
use rand::Rng;
use std::process::exit;

fn main() {
    println!("Let's play mastermind! Your goal is to guess my number.");

    let stdin = io::stdin();
    let mut rng = rand::thread_rng();

    let mut secret: Vec<u8> = Vec::new();
    for _i in 0..6 {
        secret.push(rng.gen_range(0, 10));
    }

    let mut counter: u32 = 1;
    loop {
        let mut input = String::new();

        println!("Please guess a 6 digit number or type 'quit' to stop playing.");
        stdin.read_line(&mut input).expect("Input failed");
        let input = input.trim();

        //check if user wants to quit
        if input == "quit".to_string() {
            println!("Quitting...");
            print!("Secret number was: ");
            for num in secret.iter() {
                print!("{}", num);
            }
            exit(0);
        }

        //check if input is actually a number
        let _guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };

        //check if input was 6 digits
        if input.len() != 6 {
            println!("Not a 6 digit number");
            continue;
        }

        //convert guess number into array
        let guess_arr: Vec<u8> = guess_to_vec_u8(input.to_string());

        //initialize reply values
        let mut correct_position: u32 = 0;
        let mut wrong_position: u32 = 0;

        //clone vecs for the wrong position checks
        let mut temp_secret = secret.clone();
        let mut temp_guess = guess_arr.clone();

        //determine number right and in correct position
        for i in 0..6 {
            if secret[i] == guess_arr[i] {
                let this_digit = guess_arr[i];
                correct_position += 1;
                let index = temp_secret.iter()
                    .position(|x| *x == this_digit).unwrap();
                temp_secret.remove(index); //pop digit so that it can't used for wrong position
                let index = temp_guess.iter()
                    .position(|x| *x == this_digit).unwrap();
                temp_guess.remove(index); //pop digit so that it can't used for wrong position
            }
        }

        //if all 6 were in correct position, player wins!
        if correct_position == 6 {
            println!("Congratulations! You guessed my number in {} tries!", counter);
            exit(0);
        }

        //determine number right but in the wrong position.
        for digit in temp_guess {
            if temp_secret.contains(&digit) {
                wrong_position += 1;
                let index = temp_secret.iter()
                    .position(|x| *x == digit).unwrap();
                temp_secret.remove(index); //pop digit so that it can't used again
            }
        }

        println!("Digits in correct position: {}", correct_position);
        println!("Digits right but in position: {}", wrong_position);
        counter += 1;
    }
}


fn guess_to_vec_u8(n: String) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    for i in 0..n.len(){
        let splice = &n[i..i+1];
        ret.push(splice.to_string().parse::<u8>().unwrap());
    }
    ret
}
