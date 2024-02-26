use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Guess (1 - 100): ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Guess is not a number!");

    println!("{guess}");

    match guess.cmp(&number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("You got it!"),
        Ordering::Greater => println!("Too big!"),
    }
}
