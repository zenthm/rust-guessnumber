use rand::Rng;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{guess}");
}
