use std::io;

fn main() {
    let mut guess = String::new();

    println!("Guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{guess}");
}
