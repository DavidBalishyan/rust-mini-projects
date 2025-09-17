use std::io;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("Enter time in seconds: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let time: u64 = input.trim().parse().unwrap_or(0);

    println!("Timer set for {} seconds.", time);
    for i in (1..=time).rev() {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
    }

    print!("\x07"); // ASCII Bell character (produces a beep sound in terminal)
    println!("Time's up!");
}
