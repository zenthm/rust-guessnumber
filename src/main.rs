use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number: u8 = rand::thread_rng().gen_range(u8::MIN..=u8::MAX);

    loop {
        let mut guess = String::new();

        println!("Guess ({} - {}): ", u8::MIN, u8::MAX);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = guess.trim().parse().expect("Your guess is not valid!");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => println!("You got it!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
