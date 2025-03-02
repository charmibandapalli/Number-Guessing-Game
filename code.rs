use rand::Rng;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Guess the number:");
    io::stdin().read_line(&mut guess).unwrap();
    
    if guess.trim().parse::<i32>().unwrap() == secret {
        println!("Correct!");
    } else {
        println!("Wrong guess!");
    }
}
