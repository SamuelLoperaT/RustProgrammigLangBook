//imports
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Entrypoint or main function
fn main() {
    println!("Guess the number!");
    //Random number generation
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {

        println!("Please input your guess.");
        //mutable string
        let mut guess = String::new();
        //using stdin from the standard library
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        //handling String to number parsing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        //matching result
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
