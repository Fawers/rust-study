use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0u32;

    println!("Guess the number!");

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error readling line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        tries += 1;

        match guess.cmp(&number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Jackpot!");
                break;
            }
        }
    }

    println!("Guessed correctly after {} tries", tries);
}
