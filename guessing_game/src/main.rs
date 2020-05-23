use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut number_generator = rand::thread_rng();
    let secret_number = number_generator.gen::<u8>();
    println!("{}", secret_number);
    println!("\nGuess the number!\n");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("{}", err);
                    continue;
                },
            };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
