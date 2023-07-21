use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;
use std::process::exit;
fn main() {
    println!("Guess the nubmer");

    // Generate a answer from 0-100
    let answer = rand::thread_rng().gen_range(1..=100);
    // println!("Answer is {}", answer);

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Type a number.");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Type a number");
                continue},
        };
        println!("Your guessed: {}", guess);
        // println!("Your guessed: {guess}"); // work

        match guess.cmp(&answer) {
            Ordering::Less => println!("too small."),
            Ordering::Greater => println!("too big."),
            Ordering::Equal => {
                println!("correct.");
                // exit(0)
                break;
            },
        }
    }
}
