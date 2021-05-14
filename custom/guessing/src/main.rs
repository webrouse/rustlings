use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("My super game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..3);

    'outer: loop {
        let mut guess = String::new();

        println!("Your input: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :/");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Super!");
                break 'outer;
            }
        };
    }

    println!("Random number is {}.", secret_number)
}
