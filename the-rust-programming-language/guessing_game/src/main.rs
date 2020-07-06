use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the name!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input you guess:");

        let mut guess = String::new();

        let error_msg = "Failed to read line";

        io::stdin().read_line(&mut guess).expect(error_msg);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
