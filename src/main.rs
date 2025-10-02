use fastrand as rand;
use prompted::input;

fn main() {
    let target = rand::u8(1..=10);
    let guess = input!("Guess the number between 1 and 10: ");
    let guess: u8 = match guess.parse() {
        Ok(number) => number,
        Err(error) => {
            println!("oops, not a number: {}", error);
            return;
        }
    };
    println!("{} {}", target, guess);
    if guess == target {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
