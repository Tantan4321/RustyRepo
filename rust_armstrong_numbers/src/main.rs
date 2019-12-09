use std::io;
use std::process::exit;

fn main() {
    println!("Please type in a number and I'll tell you if it's an Armstrong number:");
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Input failed");
    let input = input.trim();

    //check if input is actually a number
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a number!");
            exit(0);
        }
    };

    //convert number into separated list
    let nums: Vec<u32> = vectorize(input.to_string());

    let power: u32 = input.len() as u32;
    let mut sum: u32 = 0;
    for digit in nums {
        sum += digit.pow(power);
    }

    if number == sum {
        println!("{} is an Armstrong number.", number);
    } else {
        println!("{} is not an Armstrong number.", number);
    }
    exit(0);
}

fn vectorize(n: String) -> Vec<u32>{
    let mut ret: Vec<u32> = Vec::new();
    for i in 0..n.len(){
        let splice = &n[i..i+1];
        ret.push(splice.to_string().parse().unwrap());
    }
    ret
}
