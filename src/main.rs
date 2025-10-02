use fastrand as rand;
use prompted::input;

fn main() {
    let target = rand::u8(1..=10);
    loop {
        let guess = input!("Guess the number between 1 and 10: ");
        let guess: u8 = match guess.parse() {
            Ok(number) => number,
            Err(error) => {
                println!("oops, not a number: {}", error);
                continue;
            }
        };
        println!("{}", guess);
        if guess == target {
            println!("You win!");
            return;
        } else if guess < target {
            println!("Try higher.");
        } else {
            println!("Try lower.");
        }
    }
}
