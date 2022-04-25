use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
    This is my very first Rust program, which I made
    by following a tutorial.
    It's shit. I'm aware.
    Here's the Cargo.toml (required to use the
    specific version of rand I used here):
    [package]
    name = "godless_guessing_game"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    rand = "0.8.3"
*/

fn main() {
    println!("Guess God's current state!");
    println!("Legend: 0 means dead, 1 means alive.");

    let gods_state = rand::thread_rng().gen_range(1..2);

loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Uh oh! Failed to read line!");

    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&gods_state) {
        Ordering::Less => println!("Wrong! Too small!"),
        Ordering::Greater => println!("Wrong! Too big!"),
        Ordering::Equal => {
            println!("CORRECT! You have correctly guessed God's current state!");
            break;
            }
        }
    }
}
