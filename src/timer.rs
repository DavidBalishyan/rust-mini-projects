use std::io;
use std::thread;
use std::time::Duration;

pub fn main() {
    let input = std::env::args().nth(1).expect("no input");
    let time: u64 = input.trim().parse().unwrap_or(0);

    println!("Timer set for {} seconds.", time);
    for i in (1..=time).rev() {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
    }

    print!("\x07"); // ASCII Bell character (produces a beep sound in terminal)
    println!("Time's up!");
}
