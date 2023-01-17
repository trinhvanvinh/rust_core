use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number: {}", secret_number);
    loop {
        println!("Please input keyboard");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        println!("Your guess: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To Small".red()),
            Ordering::Greater => println!("{}", "To Big".red()),
            Ordering::Equal => {
                println!("{}", "You win!!!".green());
                break;
            }
        }
    }
}
