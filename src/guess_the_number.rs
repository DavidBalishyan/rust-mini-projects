use rand::Rng;
use std::io;

pub fn main() {
    // create random number generator
    let mut rng = rand::rng();

    // generate one secret number
    let secret_number: i32 = rng.random_range(1..=100);
    println!("[debug] Secret number: {}", secret_number);

    // read input from user
    let mut input = String::new();
    println!("Guess a number between 1 and 100:");
    io::stdin().read_line(&mut input).unwrap();

    // parse input
    let usr_guess: i32 = input.trim().parse().unwrap_or(-1);
    println!("You entered: {}", usr_guess);

    // check guess
    if usr_guess == secret_number {
    } else {
        println!("Wrong guess, try again!");
        main(); // recursive call to main to allow another guess
    }
}
